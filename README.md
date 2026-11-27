<div align="center">

<img src="assets/logo.svg" alt="Network Monitor Logo" width="88" height="88" />

# Network Monitor

**Minimal, cross-platform floating network speed monitor written in Rust.**  
*Windows ve Linux için Rust ile yazılmış hafif, şeffaf ve tek tıkla açılan yüzen ağ monitörü.*

[![Release](https://img.shields.io/github/v/release/omerseyfettinozd/Network-Monitor?style=for-the-badge&color=38E0D2&label=Release)](https://github.com/omerseyfettinozd/Network-Monitor/releases/latest)
[![CI](https://img.shields.io/github/actions/workflow/status/omerseyfettinozd/Network-Monitor/ci.yml?style=for-the-badge&label=CI)](https://github.com/omerseyfettinozd/Network-Monitor/actions/workflows/ci.yml)
[![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20Linux-FAB248?style=for-the-badge)](#download--indir)
[![Rust](https://img.shields.io/badge/Built%20with-Rust%20%2B%20egui-63B3ED?style=for-the-badge)](#tech-stack--teknoloji)
[![License](https://img.shields.io/badge/License-MIT-9CA3AF?style=for-the-badge)](#license)

</div>

---

## Download / İndir

No installer, no Python, no WebView runtime required — download and launch with a single click from **[GitHub Releases](https://github.com/omerseyfettinozd/Network-Monitor/releases/latest)**.

Kurulum, Python veya ekstra runtime gerektirmez — doğrudan indirip tek tıkla çalıştırabilirsiniz.

| Platform | File / Dosya | Usage / Kullanım |
|---|---|---|
| **Windows (x64)** | [`network-monitor-windows-x86_64.exe`](https://github.com/omerseyfettinozd/Network-Monitor/releases/latest/download/network-monitor-windows-x86_64.exe) | Double-click to run (no console window) · *Çift tıklayıp açın* |
| **Linux (x64)** | [`network-monitor-linux-x86_64`](https://github.com/omerseyfettinozd/Network-Monitor/releases/latest/download/network-monitor-linux-x86_64) | `chmod +x network-monitor-linux-x86_64 && ./network-monitor-linux-x86_64` |
| **Archives** | `.zip` / `.tar.gz` + `SHA256SUMS.txt` | Available on the [Releases page](https://github.com/omerseyfettinozd/Network-Monitor/releases/latest) |

---

## Features / Özellikler

- **Zero-Dependency Native Binary:** Single standalone executable powered by `eframe`/`egui` and `sysinfo` — low memory footprint and no root/admin privileges needed.
- **Card & Compact Pill Modes:**
  - **Card Mode (`364 × 246`):** Real-time Download/Upload cards, session peak speeds, live 60-sample sparkline, and session totals (`Σ ↓` / `Σ ↑`).
  - **Compact Pill Mode (`292 × 44`):** Minimal single-line floating capsule (`↓ ... | ↑ ...`) for gaming, streaming, or coding.
- **Live Dual Sparkline:** Smooth 60-sample area chart with dynamic auto-scaling, grid lines, and hover tooltips showing exact historical speeds.
- **Smart Interface Filtering:** Monitor `All Physical` (automatically ignores `lo`, `docker`, `veth`, `virbr`, virtual adapters), `All Interfaces`, or any specific NIC (`wlan0`, `eth0`, `Wi-Fi`, `Ethernet`, etc.).
- **Flexible Units & Controls:** Switch between `Byte/s (MB/s)` and `Bit/s (Mbps)`, adjust refresh interval (`0.5s` / `1s` / `2s`), window opacity (`55%–100%`), and `Always on Top`.
- **Persistent Settings:** Automatically remembers your theme, unit, interface, opacity, and window position.

---

## Themes / Temalar

| Theme | Style / Stil | Download Accent | Upload Accent |
|---|---|---|---|
| **Obsidian Glass** | Translucent dark slate glass | `#38E0D2` Cyan | `#FAB248` Amber |
| **Nord Frost** | Nord desktop palette (`#2E3440`) | `#88C0D0` Frost | `#EBCB8B` Aurora |
| **Earth Classic** | Tribute to v1.0 earthy brown palette | `#A3D9A5` Sage | `#F4C078` Sand |
| **Cyber Pulse** | High-contrast neon dark | `#00F5D4` Neon | `#FF56B2` Magenta |

---

## Keyboard Shortcuts & Controls / Kısayollar

Drag anywhere on the widget background to move it freely across your screen.  
*Pencerenin boş herhangi bir yerinden tutup serbestçe sürükleyebilirsiniz.*

| Key / Action | Description / Açıklama |
|---|---|
| `M` | Toggle **Card** / **Compact Pill** mode · *Kompakt moda geç* |
| `T` | Cycle through the 4 built-in **Themes** · *Tema değiştir* |
| `R` | **Reset** session totals & peak speeds · *Oturumu sıfırla* |
| `S` | Toggle **Settings** panel · *Ayarları aç/kapat* |
| `Esc` | Close Settings panel · *Ayarları kapat* |

---

## Build from Source / Kaynak Koddan Derleme

### Prerequisites
- [Rust toolchain](https://rustup.rs/) (`cargo`, `rustc`)
- **Linux only:** OpenGL, X11, and Wayland dev headers (`libgl-dev`, `libx11-dev`, `libwayland-dev`, `libxkbcommon-dev`)

```bash
git clone https://github.com/omerseyfettinozd/Network-Monitor.git
cd Network-Monitor
cargo test
cargo build --release
./target/release/network-monitor
```

Cross-compile a Windows `.exe` from Linux (`x86_64-pc-windows-gnu` target + `mingw-w64`):

```bash
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu
```

---

## Project Structure / Proje Yapısı

```text
Network-Monitor/
├── src/
│   ├── main.rs               # Entry point, viewport setup, windows_subsystem
│   ├── app.rs                # Card/Compact/Settings UI, drag & shortcuts
│   ├── monitor.rs            # sysinfo sampler, interface filter, 60-sample ring buffer
│   ├── config.rs             # Persistent JSON config (OS config directory)
│   ├── theme.rs              # 4 built-in color palettes
│   ├── icon.rs               # Runtime RGBA window icon generator
│   └── widgets/
│       ├── sparkline.rs      # Dual-series area sparkline with hover tooltip
│       └── format.rs         # Byte/s & Bit/s human-readable formatters
├── assets/
│   ├── icon.ico              # Multi-size Windows executable icon
│   └── logo.svg              # Vector logo for documentation
├── legacy/
│   └── network_monitor.py    # Original v1.0 Python/Tkinter implementation
├── build.rs                  # Windows resource & icon embedding
└── .github/workflows/        # Automated CI & cross-platform GitHub Releases
```

---

## Configuration Path / Ayar Dosyası Konumu

- **Linux:** `~/.config/network-monitor/config.json`
- **Windows:** `%APPDATA%\omerseyfettinozd\network-monitor\config\config.json`

---

## License

MIT — Free to use, modify, and distribute.
