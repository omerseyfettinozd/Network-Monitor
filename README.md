# Network Monitor v2.0

A lightweight, cross-platform floating network speed monitor rewritten from scratch in **Rust** (`eframe`/`egui` + `sysinfo`). Works out of the box on **Windows** and **Linux** (Wayland & X11) as a single standalone executable — no Python, WebView, or runtime installation required.

**Rust** ile sıfırdan yazılmış, hafif ve çapraz platform (Windows & Linux) yüzen ağ monitörü. Tek çalıştırılabilir dosya ile kurulumsuz ve tek tıkla açılır.

---

## Features / Özellikler

- **Zero-Dependency Single Binary / Tek Tıkla Çalışır**: Standalone `.exe` on Windows (no console window) and native binary on Linux.
- **Live Dual Sparkline / Canlı Grafik**: Smooth 60-sample download & upload area chart with dynamic scaling and hover tooltips.
- **Two View Modes / İki Görünüm Modu**:
  - **Card Mode**: Download/Upload cards, peak speeds, live sparkline, and session totals.
  - **Compact Pill Mode**: Minimal single-line floating bar (`↓ 12.4 MB/s | ↑ 1.8 MB/s`) ideal while gaming or coding.
- **4 Built-in Themes / 4 Yerleşik Tema**:
  - `Obsidian Glass` — Modern translucent dark glass
  - `Nord Frost` — Clean Nord desktop palette
  - `Earth Classic` — Modern tribute to the v1.0 earthy brown palette
  - `Cyber Pulse` — High-contrast neon purple & cyan
- **Smart Interface Filter / Arayüz Seçimi**: Monitor `All Physical` (excludes loopback/docker/veth by default), `All Interfaces`, or any specific NIC (`wlan0`, `eth0`, `Wi-Fi`, `Ethernet`, etc.).
- **Customizable & Persistent / Kalıcı Ayarlar**: Byte/s (`MB/s`) or Bit/s (`Mbps`), opacity slider, always-on-top toggle, refresh rate (`0.5s` / `1s` / `2s`), and saved window position.

---

## Download (One-Click) / Tek Tıkla İndirme

Grab the latest prebuilt binary from the **[GitHub Releases](https://github.com/omerseyfettinozd/Network-Monitor/releases)** page:

- **Windows (x64):** Download `network-monitor-windows-x86_64.exe` and double-click to launch.
- **Linux (x64):** Download `network-monitor-linux-x86_64`, mark executable (`chmod +x network-monitor-linux-x86_64`), and run.

---

## Keyboard Shortcuts / Klavye Kısayolları

| Key | Action / İşlev |
|---|---|
| `M` | Toggle Card / Compact Pill mode |
| `T` | Cycle themes |
| `R` | Reset session totals & peak speeds |
| `S` | Toggle Settings panel |
| `Esc` | Close Settings panel |

Drag anywhere on the widget background to move it freely.

---

## Build from Source / Kaynak Koddan Derleme

### Prerequisites
- Rust toolchain (`rustup` / `cargo`)
- **Linux only:** X11 / Wayland / OpenGL dev libraries (`libgl-dev`, `libx11-dev`, `libwayland-dev`, `libxkbcommon-dev`)

```bash
git clone https://github.com/omerseyfettinozd/Network-Monitor.git
cd Network-Monitor
cargo build --release
./target/release/network-monitor
```

Cross-compile for Windows from Linux (requires `x86_64-pc-windows-gnu` target & `mingw-w64`):

```bash
cargo build --release --target x86_64-pc-windows-gnu
```

---

## Legacy v1.0

The original Python/Tkinter version is archived in [`legacy/network_monitor.py`](legacy/network_monitor.py).

## License

MIT
