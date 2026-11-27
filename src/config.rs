use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ThemeId {
    #[default]
    ObsidianGlass,
    NordFrost,
    EarthClassic,
    CyberPulse,
}

impl ThemeId {
    pub const ALL: [ThemeId; 4] = [
        ThemeId::ObsidianGlass,
        ThemeId::NordFrost,
        ThemeId::EarthClassic,
        ThemeId::CyberPulse,
    ];

    pub fn label(self) -> &'static str {
        match self {
            ThemeId::ObsidianGlass => "Obsidian Glass",
            ThemeId::NordFrost => "Nord Frost",
            ThemeId::EarthClassic => "Earth Classic",
            ThemeId::CyberPulse => "Cyber Pulse",
        }
    }

    pub fn next(self) -> Self {
        match self {
            ThemeId::ObsidianGlass => ThemeId::NordFrost,
            ThemeId::NordFrost => ThemeId::EarthClassic,
            ThemeId::EarthClassic => ThemeId::CyberPulse,
            ThemeId::CyberPulse => ThemeId::ObsidianGlass,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum SpeedUnit {
    #[default]
    BytesPerSec,
    BitsPerSec,
}

impl SpeedUnit {
    pub fn label(self) -> &'static str {
        match self {
            SpeedUnit::BytesPerSec => "Byte/s (MB/s)",
            SpeedUnit::BitsPerSec => "Bit/s (Mbps)",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum InterfaceFilter {
    #[default]
    AllPhysical,
    AllIncludingVirtual,
    Specific(String),
}

impl InterfaceFilter {
    pub fn short_label(&self) -> String {
        match self {
            InterfaceFilter::AllPhysical => "All Physical".to_string(),
            InterfaceFilter::AllIncludingVirtual => "All Interfaces".to_string(),
            InterfaceFilter::Specific(name) => name.clone(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ViewMode {
    #[default]
    Card,
    Compact,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct AppConfig {
    pub theme: ThemeId,
    pub speed_unit: SpeedUnit,
    pub interface_filter: InterfaceFilter,
    pub view_mode: ViewMode,
    pub always_on_top: bool,
    pub opacity: f32,
    pub refresh_ms: u64,
    pub window_pos: Option<[f32; 2]>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            theme: ThemeId::ObsidianGlass,
            speed_unit: SpeedUnit::BytesPerSec,
            interface_filter: InterfaceFilter::AllPhysical,
            view_mode: ViewMode::Card,
            always_on_top: true,
            opacity: 0.92,
            refresh_ms: 1000,
            window_pos: None,
        }
    }
}

impl AppConfig {
    pub fn config_path() -> Option<PathBuf> {
        let proj = directories::ProjectDirs::from("com", "omerseyfettinozd", "network-monitor")?;
        Some(proj.config_dir().join("config.json"))
    }

    pub fn load() -> Self {
        let Some(path) = Self::config_path() else {
            return Self::default();
        };
        let Ok(raw) = fs::read_to_string(path) else {
            return Self::default();
        };
        let mut cfg: Self = serde_json::from_str(&raw).unwrap_or_default();
        cfg.sanitize();
        cfg
    }

    pub fn save(&self) {
        let Some(path) = Self::config_path() else {
            return;
        };
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        if let Ok(json) = serde_json::to_string_pretty(self) {
            let _ = fs::write(path, json);
        }
    }

    pub fn sanitize(&mut self) {
        self.opacity = self.opacity.clamp(0.55, 1.0);
        if !matches!(self.refresh_ms, 500 | 1000 | 2000) {
            self.refresh_ms = 1000;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_roundtrip_and_sanitize() {
        let mut cfg = AppConfig {
            theme: ThemeId::NordFrost,
            speed_unit: SpeedUnit::BitsPerSec,
            interface_filter: InterfaceFilter::Specific("wlan0".into()),
            view_mode: ViewMode::Compact,
            always_on_top: false,
            opacity: 0.1,
            refresh_ms: 123,
            window_pos: Some([100.0, 200.0]),
        };
        cfg.sanitize();
        assert!((cfg.opacity - 0.55).abs() < f32::EPSILON);
        assert_eq!(cfg.refresh_ms, 1000);

        let json = serde_json::to_string(&cfg).expect("serialize");
        let parsed: AppConfig = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(parsed.theme, ThemeId::NordFrost);
        assert_eq!(parsed.speed_unit, SpeedUnit::BitsPerSec);
        assert_eq!(
            parsed.interface_filter,
            InterfaceFilter::Specific("wlan0".into())
        );
        assert_eq!(parsed.view_mode, ViewMode::Compact);
    }
}
