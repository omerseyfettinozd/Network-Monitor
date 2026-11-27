use crate::config::InterfaceFilter;
use std::collections::VecDeque;
use std::time::{Duration, Instant};
use sysinfo::Networks;

pub const HISTORY_LEN: usize = 60;

#[derive(Debug, Clone, Copy, Default)]
pub struct TrafficSample {
    pub download_bps: f64,
    pub upload_bps: f64,
}

pub struct NetworkMonitor {
    networks: Networks,
    last_tick: Instant,
    available_interfaces: Vec<String>,
    history: VecDeque<TrafficSample>,
    pub current_download_bps: f64,
    pub current_upload_bps: f64,
    pub peak_download_bps: f64,
    pub peak_upload_bps: f64,
    pub session_download_bytes: u64,
    pub session_upload_bytes: u64,
    pub is_active_traffic: bool,
}

impl NetworkMonitor {
    pub fn new() -> Self {
        let networks = Networks::new_with_refreshed_list();
        let available_interfaces = collect_interface_names(&networks);
        let mut history = VecDeque::with_capacity(HISTORY_LEN);
        for _ in 0..HISTORY_LEN {
            history.push_back(TrafficSample::default());
        }

        Self {
            networks,
            last_tick: Instant::now(),
            available_interfaces,
            history,
            current_download_bps: 0.0,
            current_upload_bps: 0.0,
            peak_download_bps: 0.0,
            peak_upload_bps: 0.0,
            session_download_bytes: 0,
            session_upload_bytes: 0,
            is_active_traffic: false,
        }
    }

    pub fn interfaces(&self) -> &[String] {
        &self.available_interfaces
    }

    pub fn history(&self) -> &VecDeque<TrafficSample> {
        &self.history
    }

    pub fn reset_session(&mut self) {
        self.session_download_bytes = 0;
        self.session_upload_bytes = 0;
        self.peak_download_bps = 0.0;
        self.peak_upload_bps = 0.0;
        self.history.clear();
        for _ in 0..HISTORY_LEN {
            self.history.push_back(TrafficSample::default());
        }
    }

    /// Samples network counters if `interval` has elapsed since last tick.
    /// Returns `true` when a new sample was recorded.
    pub fn tick(&mut self, filter: &InterfaceFilter, interval: Duration) -> bool {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_tick);
        if elapsed < interval {
            return false;
        }
        let dt = elapsed.as_secs_f64().max(0.05);
        self.last_tick = now;

        self.networks.refresh(true);
        self.available_interfaces = collect_interface_names(&self.networks);

        let mut delta_rx: u64 = 0;
        let mut delta_tx: u64 = 0;

        for (name, data) in &self.networks {
            if matches_filter(name, filter) {
                delta_rx = delta_rx.saturating_add(data.received());
                delta_tx = delta_tx.saturating_add(data.transmitted());
            }
        }

        let dl_bps = delta_rx as f64 / dt;
        let ul_bps = delta_tx as f64 / dt;

        self.record_sample(delta_rx, delta_tx, dl_bps, ul_bps);
        true
    }

    fn record_sample(&mut self, delta_rx: u64, delta_tx: u64, dl_bps: f64, ul_bps: f64) {
        self.current_download_bps = dl_bps;
        self.current_upload_bps = ul_bps;
        self.peak_download_bps = self.peak_download_bps.max(dl_bps);
        self.peak_upload_bps = self.peak_upload_bps.max(ul_bps);
        self.session_download_bytes = self.session_download_bytes.saturating_add(delta_rx);
        self.session_upload_bytes = self.session_upload_bytes.saturating_add(delta_tx);
        self.is_active_traffic = (dl_bps + ul_bps) >= 256.0;

        if self.history.len() >= HISTORY_LEN {
            self.history.pop_front();
        }
        self.history.push_back(TrafficSample {
            download_bps: dl_bps,
            upload_bps: ul_bps,
        });
    }
}

fn collect_interface_names(networks: &Networks) -> Vec<String> {
    let mut names: Vec<String> = networks.keys().cloned().collect();
    names.sort_by(|a, b| {
        let a_virt = is_virtual_interface(a);
        let b_virt = is_virtual_interface(b);
        a_virt.cmp(&b_virt).then_with(|| a.cmp(b))
    });
    names
}

pub fn is_virtual_interface(name: &str) -> bool {
    let lower = name.to_ascii_lowercase();
    lower == "lo"
        || lower.starts_with("loopback")
        || lower.starts_with("docker")
        || lower.starts_with("veth")
        || lower.starts_with("br-")
        || lower.starts_with("virbr")
        || lower.starts_with("podman")
        || lower.starts_with("cni")
        || lower.starts_with("flannel")
        || lower.starts_with("kube")
        || lower.contains("vethernet")
        || lower.contains("virtual")
        || lower.contains("npcap")
        || lower.contains("isatap")
        || lower.contains("teredo")
}

pub fn matches_filter(name: &str, filter: &InterfaceFilter) -> bool {
    match filter {
        InterfaceFilter::AllPhysical => !is_virtual_interface(name),
        InterfaceFilter::AllIncludingVirtual => true,
        InterfaceFilter::Specific(target) => name == target,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn virtual_interface_detection() {
        assert!(is_virtual_interface("lo"));
        assert!(is_virtual_interface("docker0"));
        assert!(is_virtual_interface("veth1234a"));
        assert!(is_virtual_interface("Loopback Pseudo-Interface 1"));
        assert!(!is_virtual_interface("wlan0"));
        assert!(!is_virtual_interface("enp4s0"));
        assert!(!is_virtual_interface("Wi-Fi"));
        assert!(!is_virtual_interface("Ethernet"));
    }

    #[test]
    fn filter_matching_works() {
        assert!(!matches_filter("lo", &InterfaceFilter::AllPhysical));
        assert!(matches_filter("wlan0", &InterfaceFilter::AllPhysical));
        assert!(matches_filter("lo", &InterfaceFilter::AllIncludingVirtual));
        assert!(matches_filter(
            "eth0",
            &InterfaceFilter::Specific("eth0".into())
        ));
        assert!(!matches_filter(
            "wlan0",
            &InterfaceFilter::Specific("eth0".into())
        ));
    }

    #[test]
    fn ring_buffer_and_session_reset() {
        let mut mon = NetworkMonitor::new();
        assert_eq!(mon.history().len(), HISTORY_LEN);

        mon.record_sample(4096, 1024, 4096.0, 1024.0);
        assert_eq!(mon.history().len(), HISTORY_LEN);
        assert_eq!(mon.session_download_bytes, 4096);
        assert_eq!(mon.session_upload_bytes, 1024);
        assert_eq!(mon.peak_download_bps, 4096.0);
        assert!(mon.is_active_traffic);

        mon.reset_session();
        assert_eq!(mon.session_download_bytes, 0);
        assert_eq!(mon.session_upload_bytes, 0);
        assert_eq!(mon.peak_download_bps, 0.0);
        assert_eq!(mon.history().len(), HISTORY_LEN);
    }
}
