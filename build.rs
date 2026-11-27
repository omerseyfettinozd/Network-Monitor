fn main() {
    println!("cargo:rerun-if-changed=assets/icon.ico");

    #[cfg(windows)]
    {
        // Note: cfg(windows) in build.rs checks HOST OS.
        // For Linux->Windows cross-compilation, check CARGO_CFG_TARGET_OS instead.
    }

    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    if target_os == "windows" {
        embed_windows_resources();
    }
}

#[cfg(windows)]
fn embed_windows_resources() {
    let mut res = winresource::WindowsResource::new();
    res.set_icon("assets/icon.ico");
    res.set("ProductName", "Network Monitor");
    res.set("FileDescription", "Floating Real-Time Network Monitor");
    res.set("LegalCopyright", "MIT License");
    if let Err(err) = res.compile() {
        println!("cargo:warning=Failed to embed Windows resources: {err}");
    }
}

#[cfg(not(windows))]
fn embed_windows_resources() {
    // When cross-compiling from Linux to Windows (x86_64-pc-windows-gnu),
    // compile the .rc file with x86_64-w64-mingw32-windres if available.
    let windres = std::process::Command::new("x86_64-w64-mingw32-windres")
        .arg("--version")
        .output();

    if windres.is_err() {
        return;
    }

    let out_dir = match std::env::var("OUT_DIR") {
        Ok(dir) => std::path::PathBuf::from(dir),
        Err(_) => return,
    };

    let rc_path = out_dir.join("network_monitor.rc");
    let obj_path = out_dir.join("network_monitor_res.o");
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".into());
    let icon_path = std::path::PathBuf::from(&manifest_dir)
        .join("assets")
        .join("icon.ico");
    let icon_escaped = icon_path.to_string_lossy().replace('\\', "\\\\");

    let rc_content = format!("1 ICON \"{}\"\n", icon_escaped);

    if std::fs::write(&rc_path, rc_content).is_err() {
        return;
    }

    let status = std::process::Command::new("x86_64-w64-mingw32-windres")
        .arg(&rc_path)
        .arg("-O")
        .arg("coff")
        .arg("-o")
        .arg(&obj_path)
        .status();

    if let Ok(s) = status
        && s.success()
    {
        println!("cargo:rustc-link-arg={}", obj_path.display());
    }
}
