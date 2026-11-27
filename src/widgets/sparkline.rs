use crate::config::SpeedUnit;
use crate::monitor::TrafficSample;
use crate::theme::Palette;
use crate::widgets::format::format_speed;
use eframe::egui::{
    self, Align2, Color32, CornerRadius, FontId, Pos2, Rect, Sense, Shape, Stroke, Vec2,
};
use std::collections::VecDeque;

/// Renders a live dual-series area sparkline with dynamic scaling, grid lines, and hover tooltip.
pub fn draw_sparkline(
    ui: &mut egui::Ui,
    history: &VecDeque<TrafficSample>,
    palette: &Palette,
    unit: SpeedUnit,
    height: f32,
) {
    let desired_size = Vec2::new(ui.available_width(), height);
    let (rect, response) = ui.allocate_exact_size(desired_size, Sense::hover());

    if !ui.is_rect_visible(rect) {
        return;
    }

    let painter = ui.painter_at(rect);

    // Graph container background
    painter.rect(
        rect,
        CornerRadius::same(10),
        palette.graph_bg,
        Stroke::new(1.0_f32, palette.border.gamma_multiply(0.5)),
        egui::StrokeKind::Inside,
    );

    let inner = rect.shrink2(Vec2::new(8.0, 8.0));
    if inner.width() <= 4.0 || inner.height() <= 4.0 {
        return;
    }

    // Horizontal grid lines (25%, 50%, 75%)
    for frac in [0.25_f32, 0.5, 0.75] {
        let y = inner.bottom() - inner.height() * frac;
        painter.line_segment(
            [Pos2::new(inner.left(), y), Pos2::new(inner.right(), y)],
            Stroke::new(1.0_f32, palette.graph_grid),
        );
    }

    // Dynamic max scale (minimum 10 KB/s so idle noise doesn't spike visually)
    let raw_max = history
        .iter()
        .fold(0.0_f64, |acc, s| acc.max(s.download_bps).max(s.upload_bps));
    let max_val = (raw_max * 1.15).max(10_240.0);

    let count = history.len();
    if count >= 2 {
        let step_x = inner.width() / (count - 1) as f32;

        // Draw download area + line
        draw_series(
            &painter,
            inner,
            step_x,
            history.iter().map(|s| s.download_bps),
            max_val,
            palette.download,
            0.18,
            1.9,
        );

        // Draw upload area + line
        draw_series(
            &painter,
            inner,
            step_x,
            history.iter().map(|s| s.upload_bps),
            max_val,
            palette.upload,
            0.14,
            1.6,
        );

        // Hover crosshair & tooltip
        if let Some(pointer) = response.hover_pos()
            && rect.contains(pointer)
        {
            let rel = ((pointer.x - inner.left()) / inner.width()).clamp(0.0, 1.0);
            let idx = ((rel * (count - 1) as f32).round() as usize).min(count - 1);
            if let Some(sample) = history.get(idx) {
                let x = inner.left() + idx as f32 * step_x;
                painter.line_segment(
                    [Pos2::new(x, inner.top()), Pos2::new(x, inner.bottom())],
                    Stroke::new(1.0_f32, palette.text_secondary.gamma_multiply(0.55)),
                );

                let dl_y = value_to_y(sample.download_bps, max_val, inner);
                let ul_y = value_to_y(sample.upload_bps, max_val, inner);
                painter.circle_filled(Pos2::new(x, dl_y), 3.0, palette.download);
                painter.circle_filled(Pos2::new(x, ul_y), 3.0, palette.upload);

                let ago = count - 1 - idx;
                let tip = format!(
                    "{}  ↓ {}  ↑ {}",
                    if ago == 0 {
                        "now".to_string()
                    } else {
                        format!("-{ago}")
                    },
                    format_speed(sample.download_bps, unit),
                    format_speed(sample.upload_bps, unit)
                );
                let tip_pos = Pos2::new(inner.left() + 4.0, inner.top() + 2.0);
                painter.text(
                    tip_pos,
                    Align2::LEFT_TOP,
                    tip,
                    FontId::monospace(10.0),
                    palette.text_primary,
                );
            }
        }
    }

    // Top-right scale label
    let scale_label = format_speed(max_val, unit);
    painter.text(
        Pos2::new(inner.right() - 2.0, inner.top() + 1.0),
        Align2::RIGHT_TOP,
        scale_label,
        FontId::monospace(9.5),
        palette.text_secondary.gamma_multiply(0.85),
    );
}

#[allow(clippy::too_many_arguments)]
fn draw_series(
    painter: &egui::Painter,
    inner: Rect,
    step_x: f32,
    values: impl Iterator<Item = f64>,
    max_val: f64,
    color: Color32,
    fill_alpha: f32,
    stroke_width: f32,
) {
    let points: Vec<Pos2> = values
        .enumerate()
        .map(|(i, val)| {
            let x = inner.left() + i as f32 * step_x;
            let y = value_to_y(val, max_val, inner);
            Pos2::new(x, y)
        })
        .collect();

    if points.len() < 2 {
        return;
    }

    // Area fill under the curve using a triangle strip mesh
    let fill_col = color.gamma_multiply(fill_alpha);
    let mut mesh = egui::Mesh::default();
    for (i, p) in points.iter().enumerate() {
        let base_idx = (i * 2) as u32;
        mesh.colored_vertex(*p, fill_col);
        mesh.colored_vertex(Pos2::new(p.x, inner.bottom()), Color32::TRANSPARENT);
        if i + 1 < points.len() {
            mesh.add_triangle(base_idx, base_idx + 1, base_idx + 2);
            mesh.add_triangle(base_idx + 1, base_idx + 3, base_idx + 2);
        }
    }
    painter.add(Shape::mesh(mesh));

    // Crisp line on top
    painter.add(Shape::line(points, Stroke::new(stroke_width, color)));
}

fn value_to_y(val: f64, max_val: f64, inner: Rect) -> f32 {
    let norm = (val / max_val).clamp(0.0, 1.0) as f32;
    inner.bottom() - norm * inner.height()
}
