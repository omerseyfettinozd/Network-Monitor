use eframe::egui::IconData;

/// Generates a 64x64 anti-aliased RGBA window icon matching `assets/icon.ico`.
pub fn generate_window_icon() -> IconData {
    let size: u32 = 64;
    let mut rgba = vec![0u8; (size * size * 4) as usize];

    for py in 0..size {
        for px in 0..size {
            let x = (px as f32 + 0.5) / size as f32 * 2.0 - 1.0;
            let y = (py as f32 + 0.5) / size as f32 * 2.0 - 1.0;
            let aa = 2.2 / size as f32;

            let d_bg = sdf_rounded_rect(x, y, 0.90, 0.90, 0.34);
            let bg_a = ((0.0 - d_bg) / aa + 0.5).clamp(0.0, 1.0);
            let t = (y + 1.0) * 0.5;
            let mut col = [18.0 + 12.0 * t, 24.0 + 14.0 * t, 36.0 + 18.0 * t, bg_a];

            let d_border = (d_bg + 0.03).abs() - 0.025;
            let b_a = ((0.0 - d_border) / aa + 0.5).clamp(0.0, 1.0) * bg_a * 0.55;
            if b_a > 0.0 {
                col = blend(col, [100.0, 210.0, 235.0], b_a);
            }

            let w = 0.095;
            let d_dl = (sdf_segment(x, y, -0.32, -0.44, -0.32, 0.40) - w)
                .min(sdf_segment(x, y, -0.56, 0.14, -0.32, 0.44) - w)
                .min(sdf_segment(x, y, -0.08, 0.14, -0.32, 0.44) - w);
            let dl_a = ((0.0 - d_dl) / aa + 0.5).clamp(0.0, 1.0) * bg_a;
            if dl_a > 0.0 {
                col = blend(col, [56.0, 224.0, 210.0], dl_a);
            }

            let d_ul = (sdf_segment(x, y, 0.32, 0.44, 0.32, -0.40) - w)
                .min(sdf_segment(x, y, 0.08, -0.14, 0.32, -0.44) - w)
                .min(sdf_segment(x, y, 0.56, -0.14, 0.32, -0.44) - w);
            let ul_a = ((0.0 - d_ul) / aa + 0.5).clamp(0.0, 1.0) * bg_a;
            if ul_a > 0.0 {
                col = blend(col, [250.0, 178.0, 72.0], ul_a);
            }

            let idx = ((py * size + px) * 4) as usize;
            rgba[idx] = col[0].clamp(0.0, 255.0) as u8;
            rgba[idx + 1] = col[1].clamp(0.0, 255.0) as u8;
            rgba[idx + 2] = col[2].clamp(0.0, 255.0) as u8;
            rgba[idx + 3] = (col[3] * 255.0).clamp(0.0, 255.0) as u8;
        }
    }

    IconData {
        rgba,
        width: size,
        height: size,
    }
}

fn sdf_rounded_rect(x: f32, y: f32, bx: f32, by: f32, r: f32) -> f32 {
    let qx = x.abs() - bx + r;
    let qy = y.abs() - by + r;
    qx.max(0.0).hypot(qy.max(0.0)) + qx.max(qy).min(0.0) - r
}

fn sdf_segment(px: f32, py: f32, ax: f32, ay: f32, bx: f32, by: f32) -> f32 {
    let pax = px - ax;
    let pay = py - ay;
    let bax = bx - ax;
    let bay = by - ay;
    let h = ((pax * bax + pay * bay) / (bax * bax + bay * bay + 1e-9)).clamp(0.0, 1.0);
    (pax - bax * h).hypot(pay - bay * h)
}

fn blend(dst: [f32; 4], src: [f32; 3], a: f32) -> [f32; 4] {
    let a = a.clamp(0.0, 1.0);
    [
        dst[0] * (1.0 - a) + src[0] * a,
        dst[1] * (1.0 - a) + src[1] * a,
        dst[2] * (1.0 - a) + src[2] * a,
        dst[3].max(a),
    ]
}
