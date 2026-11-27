use crate::config::{AppConfig, InterfaceFilter, SpeedUnit, ThemeId, ViewMode};
use crate::monitor::NetworkMonitor;
use crate::theme::Palette;
use crate::widgets::format::{format_bytes, format_speed};
use crate::widgets::sparkline::draw_sparkline;
use eframe::egui::{
    self, Align, Button, Color32, CornerRadius, FontId, Frame, Key, Layout, Margin, RichText,
    Sense, Stroke, Vec2, ViewportCommand, WindowLevel,
};
use std::time::Duration;

pub const CARD_SIZE: Vec2 = Vec2::new(364.0, 246.0);
pub const SETTINGS_SIZE: Vec2 = Vec2::new(364.0, 336.0);
pub const COMPACT_SIZE: Vec2 = Vec2::new(292.0, 44.0);

pub struct NetworkMonitorApp {
    config: AppConfig,
    monitor: NetworkMonitor,
    show_settings: bool,
    last_applied_size: Vec2,
    dirty_config: bool,
}

impl NetworkMonitorApp {
    pub fn new(config: AppConfig) -> Self {
        let initial_size = match config.view_mode {
            ViewMode::Card => CARD_SIZE,
            ViewMode::Compact => COMPACT_SIZE,
        };
        Self {
            config,
            monitor: NetworkMonitor::new(),
            show_settings: false,
            last_applied_size: initial_size,
            dirty_config: false,
        }
    }

    fn desired_window_size(&self) -> Vec2 {
        if self.show_settings {
            SETTINGS_SIZE
        } else {
            match self.config.view_mode {
                ViewMode::Card => CARD_SIZE,
                ViewMode::Compact => COMPACT_SIZE,
            }
        }
    }

    fn sync_window_size(&mut self, ctx: &egui::Context) {
        let target = self.desired_window_size();
        if (target - self.last_applied_size).length_sq() > 0.5 {
            ctx.send_viewport_cmd(ViewportCommand::InnerSize(target));
            self.last_applied_size = target;
        }
    }

    fn handle_shortcuts(&mut self, ctx: &egui::Context) {
        ctx.input(|i| {
            if i.key_pressed(Key::M) {
                self.config.view_mode = match self.config.view_mode {
                    ViewMode::Card => ViewMode::Compact,
                    ViewMode::Compact => ViewMode::Card,
                };
                self.show_settings = false;
                self.dirty_config = true;
            }
            if i.key_pressed(Key::T) {
                self.config.theme = self.config.theme.next();
                self.dirty_config = true;
            }
            if i.key_pressed(Key::R) {
                self.monitor.reset_session();
            }
            if i.key_pressed(Key::S) {
                self.show_settings = !self.show_settings;
                if self.show_settings {
                    self.config.view_mode = ViewMode::Card;
                }
            }
            if i.key_pressed(Key::Escape) && self.show_settings {
                self.show_settings = false;
            }
        });
    }

    fn draw_compact_mode(&mut self, ui: &mut egui::Ui, ctx: &egui::Context, palette: &Palette) {
        let dl_text = format_speed(self.monitor.current_download_bps, self.config.speed_unit);
        let ul_text = format_speed(self.monitor.current_upload_bps, self.config.speed_unit);

        ui.horizontal_centered(|ui| {
            ui.spacing_mut().item_spacing.x = 8.0;

            // Activity LED
            let led_color = if self.monitor.is_active_traffic {
                palette.download
            } else {
                palette.text_secondary.gamma_multiply(0.5)
            };
            let (led_rect, _) = ui.allocate_exact_size(Vec2::splat(8.0), Sense::hover());
            ui.painter()
                .circle_filled(led_rect.center(), 3.6, led_color);

            ui.label(
                RichText::new(format!("↓ {dl_text}"))
                    .font(FontId::monospace(12.5))
                    .strong()
                    .color(palette.download),
            );

            ui.label(
                RichText::new("|")
                    .font(FontId::proportional(11.0))
                    .color(palette.border),
            );

            ui.label(
                RichText::new(format!("↑ {ul_text}"))
                    .font(FontId::monospace(12.5))
                    .strong()
                    .color(palette.upload),
            );

            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                ui.spacing_mut().item_spacing.x = 4.0;

                if icon_button(ui, "✕", "Close", palette.text_secondary, palette.danger).clicked()
                {
                    ctx.send_viewport_cmd(ViewportCommand::Close);
                }

                if icon_button(
                    ui,
                    "□",
                    "Expand (M)",
                    palette.text_secondary,
                    palette.accent,
                )
                .clicked()
                {
                    self.config.view_mode = ViewMode::Card;
                    self.dirty_config = true;
                }
            });
        });
    }

    fn draw_card_mode(&mut self, ui: &mut egui::Ui, ctx: &egui::Context, palette: &Palette) {
        // Header Bar
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 6.0;

            let led_color = if self.monitor.is_active_traffic {
                palette.download
            } else {
                palette.text_secondary.gamma_multiply(0.45)
            };
            let (led_rect, _) = ui.allocate_exact_size(Vec2::splat(10.0), Sense::hover());
            ui.painter()
                .circle_filled(led_rect.center(), 4.0, led_color);

            ui.label(
                RichText::new("NET MONITOR")
                    .font(FontId::proportional(11.5))
                    .strong()
                    .color(palette.text_primary),
            );

            // Interface badge
            let iface_badge = truncate_label(&self.config.interface_filter.short_label(), 13);
            let badge_frame = Frame::NONE
                .fill(palette.card_bg)
                .stroke(Stroke::new(1.0_f32, palette.border.gamma_multiply(0.45)))
                .corner_radius(CornerRadius::same(6))
                .inner_margin(Margin::symmetric(6, 2));
            badge_frame.show(ui, |ui| {
                ui.label(
                    RichText::new(iface_badge)
                        .font(FontId::monospace(9.5))
                        .color(palette.text_secondary),
                );
            });

            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                ui.spacing_mut().item_spacing.x = 4.0;

                if icon_button(ui, "✕", "Close", palette.text_secondary, palette.danger).clicked()
                {
                    ctx.send_viewport_cmd(ViewportCommand::Close);
                }

                let gear_color = if self.show_settings {
                    palette.accent
                } else {
                    palette.text_secondary
                };
                if icon_button(ui, "⚙", "Settings (S)", gear_color, palette.accent).clicked() {
                    self.show_settings = !self.show_settings;
                }

                if icon_button(
                    ui,
                    "—",
                    "Compact Mode (M)",
                    palette.text_secondary,
                    palette.accent,
                )
                .clicked()
                {
                    self.show_settings = false;
                    self.config.view_mode = ViewMode::Compact;
                    self.dirty_config = true;
                }
            });
        });

        ui.add_space(6.0);

        if self.show_settings {
            self.draw_settings_panel(ui, ctx, palette);
            return;
        }

        // Dual Speed Cards
        let card_width = (ui.available_width() - 8.0) * 0.5;
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 8.0;

            speed_card(
                ui,
                card_width,
                "↓ DOWNLOAD",
                &format_speed(self.monitor.current_download_bps, self.config.speed_unit),
                &format!(
                    "Peak {}",
                    format_speed(self.monitor.peak_download_bps, self.config.speed_unit)
                ),
                palette.download,
                palette,
            );

            speed_card(
                ui,
                card_width,
                "↑ UPLOAD",
                &format_speed(self.monitor.current_upload_bps, self.config.speed_unit),
                &format!(
                    "Peak {}",
                    format_speed(self.monitor.peak_upload_bps, self.config.speed_unit)
                ),
                palette.upload,
                palette,
            );
        });

        ui.add_space(8.0);

        // Live Dual Sparkline
        draw_sparkline(
            ui,
            self.monitor.history(),
            palette,
            self.config.speed_unit,
            78.0,
        );

        ui.add_space(6.0);

        // Footer: Session Totals + Reset
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 10.0;

            ui.label(
                RichText::new(format!(
                    "Σ ↓ {}",
                    format_bytes(self.monitor.session_download_bytes)
                ))
                .font(FontId::monospace(10.5))
                .color(palette.text_secondary),
            );

            ui.label(
                RichText::new(format!(
                    "Σ ↑ {}",
                    format_bytes(self.monitor.session_upload_bytes)
                ))
                .font(FontId::monospace(10.5))
                .color(palette.text_secondary),
            );

            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                if icon_button(
                    ui,
                    "↺",
                    "Reset Session (R)",
                    palette.text_secondary,
                    palette.accent,
                )
                .clicked()
                {
                    self.monitor.reset_session();
                }
            });
        });
    }

    fn draw_settings_panel(&mut self, ui: &mut egui::Ui, ctx: &egui::Context, palette: &Palette) {
        let panel_frame = Frame::NONE
            .fill(palette.card_bg)
            .stroke(Stroke::new(1.0_f32, palette.border.gamma_multiply(0.5)))
            .corner_radius(CornerRadius::same(10))
            .inner_margin(Margin::same(10));

        panel_frame.show(ui, |ui| {
            ui.spacing_mut().item_spacing.y = 8.0;

            // Theme selector
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new("Theme")
                        .size(11.5)
                        .color(palette.text_secondary),
                );
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    egui::ComboBox::from_id_salt("theme_combo")
                        .width(165.0)
                        .selected_text(self.config.theme.label())
                        .show_ui(ui, |ui| {
                            for t in ThemeId::ALL {
                                if ui
                                    .selectable_value(&mut self.config.theme, t, t.label())
                                    .changed()
                                {
                                    self.dirty_config = true;
                                }
                            }
                        });
                });
            });

            // Interface selector
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new("Interface")
                        .size(11.5)
                        .color(palette.text_secondary),
                );
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    let selected = truncate_label(&self.config.interface_filter.short_label(), 18);
                    egui::ComboBox::from_id_salt("iface_combo")
                        .width(165.0)
                        .selected_text(selected)
                        .show_ui(ui, |ui| {
                            if ui
                                .selectable_value(
                                    &mut self.config.interface_filter,
                                    InterfaceFilter::AllPhysical,
                                    "All Physical",
                                )
                                .changed()
                            {
                                self.dirty_config = true;
                            }
                            if ui
                                .selectable_value(
                                    &mut self.config.interface_filter,
                                    InterfaceFilter::AllIncludingVirtual,
                                    "All Interfaces",
                                )
                                .changed()
                            {
                                self.dirty_config = true;
                            }
                            ui.separator();
                            for name in self.monitor.interfaces().to_vec() {
                                let target = InterfaceFilter::Specific(name.clone());
                                if ui
                                    .selectable_value(
                                        &mut self.config.interface_filter,
                                        target,
                                        name,
                                    )
                                    .changed()
                                {
                                    self.dirty_config = true;
                                }
                            }
                        });
                });
            });

            // Unit selector
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new("Unit")
                        .size(11.5)
                        .color(palette.text_secondary),
                );
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    for unit in [SpeedUnit::BitsPerSec, SpeedUnit::BytesPerSec] {
                        let active = self.config.speed_unit == unit;
                        let btn =
                            Button::new(RichText::new(unit.label()).size(10.5).color(if active {
                                palette.text_primary
                            } else {
                                palette.text_secondary
                            }))
                            .fill(if active {
                                palette.accent.gamma_multiply(0.35)
                            } else {
                                Color32::TRANSPARENT
                            })
                            .stroke(Stroke::new(
                                1.0_f32,
                                if active {
                                    palette.accent
                                } else {
                                    palette.border.gamma_multiply(0.45)
                                },
                            ))
                            .corner_radius(CornerRadius::same(6));
                        if ui.add(btn).clicked() {
                            self.config.speed_unit = unit;
                            self.dirty_config = true;
                        }
                    }
                });
            });

            // Refresh interval
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new("Refresh")
                        .size(11.5)
                        .color(palette.text_secondary),
                );
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    for (ms, label) in [(2000u64, "2s"), (1000, "1s"), (500, "0.5s")] {
                        let active = self.config.refresh_ms == ms;
                        let btn = Button::new(RichText::new(label).size(10.5).color(if active {
                            palette.text_primary
                        } else {
                            palette.text_secondary
                        }))
                        .fill(if active {
                            palette.accent.gamma_multiply(0.35)
                        } else {
                            Color32::TRANSPARENT
                        })
                        .stroke(Stroke::new(
                            1.0_f32,
                            if active {
                                palette.accent
                            } else {
                                palette.border.gamma_multiply(0.45)
                            },
                        ))
                        .corner_radius(CornerRadius::same(6));
                        if ui.add(btn).clicked() {
                            self.config.refresh_ms = ms;
                            self.dirty_config = true;
                        }
                    }
                });
            });

            // Opacity slider
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new(format!("Opacity ({:.0}%)", self.config.opacity * 100.0))
                        .size(11.5)
                        .color(palette.text_secondary),
                );
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    let slider =
                        egui::Slider::new(&mut self.config.opacity, 0.55..=1.0).show_value(false);
                    if ui.add_sized(Vec2::new(155.0, 18.0), slider).changed() {
                        self.dirty_config = true;
                    }
                });
            });

            // Always on top toggle
            ui.horizontal(|ui| {
                if ui
                    .checkbox(
                        &mut self.config.always_on_top,
                        RichText::new("Always on Top")
                            .size(11.5)
                            .color(palette.text_primary),
                    )
                    .changed()
                {
                    let level = if self.config.always_on_top {
                        WindowLevel::AlwaysOnTop
                    } else {
                        WindowLevel::Normal
                    };
                    ctx.send_viewport_cmd(ViewportCommand::WindowLevel(level));
                    self.dirty_config = true;
                }

                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    if ui
                        .small_button(
                            RichText::new("Reset Session")
                                .size(10.5)
                                .color(palette.text_primary),
                        )
                        .clicked()
                    {
                        self.monitor.reset_session();
                    }
                });
            });

            ui.separator();

            ui.horizontal(|ui| {
                ui.label(
                    RichText::new("Shortcuts: M mode · T theme · R reset · S settings")
                        .size(9.5)
                        .color(palette.text_secondary),
                );
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    if ui
                        .small_button(RichText::new("Done").size(10.5).color(palette.accent))
                        .clicked()
                    {
                        self.show_settings = false;
                    }
                });
            });
        });
    }
}

impl eframe::App for NetworkMonitorApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        [0.0, 0.0, 0.0, 0.0]
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.handle_shortcuts(ctx);

        let interval = Duration::from_millis(self.config.refresh_ms);
        self.monitor.tick(&self.config.interface_filter, interval);

        // Request timely repaint for smooth periodic updates without burning CPU
        ctx.request_repaint_after(Duration::from_millis(self.config.refresh_ms.min(500)));

        self.sync_window_size(ctx);

        let palette = Palette::from_theme(self.config.theme, self.config.opacity);
        apply_visuals(ctx, &palette);

        let is_compact = self.config.view_mode == ViewMode::Compact && !self.show_settings;
        let corner_radius = if is_compact { 22 } else { 16 };
        let inner_margin = if is_compact {
            Margin::symmetric(14, 6)
        } else {
            Margin::symmetric(14, 12)
        };

        let outer_frame = Frame::NONE.fill(Color32::TRANSPARENT);

        egui::CentralPanel::default()
            .frame(outer_frame)
            .show(ctx, |ui| {
                // Register background drag BEFORE child widgets so buttons/sliders/combos stay on top.
                let drag_resp = ui.interact(
                    ui.max_rect(),
                    ui.id().with("window_drag"),
                    Sense::drag(),
                );

                let container = Frame::NONE
                    .fill(palette.bg)
                    .stroke(Stroke::new(1.2_f32, palette.border))
                    .corner_radius(CornerRadius::same(corner_radius))
                    .inner_margin(inner_margin);

                container.show(ui, |ui| {
                    ui.set_min_size(ui.available_size());
                    if is_compact {
                        self.draw_compact_mode(ui, ctx, &palette);
                    } else {
                        self.draw_card_mode(ui, ctx, &palette);
                    }
                });

                if drag_resp.drag_started_by(egui::PointerButton::Primary) {
                    ctx.send_viewport_cmd(ViewportCommand::StartDrag);
                }
            });

        // Persist window position & dirty config lazily
        if let Some(outer) = ctx.input(|i| i.viewport().outer_rect) {
            let pos = [outer.min.x, outer.min.y];
            if self.config.window_pos != Some(pos) {
                self.config.window_pos = Some(pos);
            }
        }
        if self.dirty_config {
            self.config.save();
            self.dirty_config = false;
        }
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.config.save();
    }
}

fn speed_card(
    ui: &mut egui::Ui,
    width: f32,
    title: &str,
    value: &str,
    subtext: &str,
    accent: Color32,
    palette: &Palette,
) {
    let frame = Frame::NONE
        .fill(palette.card_bg)
        .stroke(Stroke::new(1.0_f32, accent.gamma_multiply(0.32)))
        .corner_radius(CornerRadius::same(10))
        .inner_margin(Margin::symmetric(10, 8));

    frame.show(ui, |ui| {
        ui.set_width((width - 20.0).max(80.0));
        ui.vertical(|ui| {
            ui.spacing_mut().item_spacing.y = 2.0;
            ui.label(
                RichText::new(title)
                    .font(FontId::proportional(9.5))
                    .strong()
                    .color(accent.gamma_multiply(0.9)),
            );
            ui.label(
                RichText::new(value)
                    .font(FontId::monospace(16.5))
                    .strong()
                    .color(palette.text_primary),
            );
            ui.label(
                RichText::new(subtext)
                    .font(FontId::monospace(9.5))
                    .color(palette.text_secondary),
            );
        });
    });
}

fn icon_button(
    ui: &mut egui::Ui,
    label: &str,
    tooltip: &str,
    color: Color32,
    hover_color: Color32,
) -> egui::Response {
    let btn = Button::new(RichText::new(label).size(12.0).color(color))
        .fill(Color32::TRANSPARENT)
        .stroke(Stroke::NONE)
        .min_size(Vec2::splat(20.0));
    let resp = ui.add(btn);
    if resp.hovered() {
        ui.painter().text(
            resp.rect.center(),
            egui::Align2::CENTER_CENTER,
            label,
            FontId::proportional(12.0),
            hover_color,
        );
    }
    resp.on_hover_text(tooltip)
}

fn apply_visuals(ctx: &egui::Context, palette: &Palette) {
    let mut visuals = egui::Visuals::dark();
    visuals.window_fill = palette.bg;
    visuals.panel_fill = Color32::TRANSPARENT;
    visuals.widgets.noninteractive.fg_stroke.color = palette.text_primary;
    visuals.widgets.inactive.bg_fill = palette.card_bg;
    visuals.widgets.hovered.bg_fill = palette.accent.gamma_multiply(0.28);
    visuals.widgets.active.bg_fill = palette.accent.gamma_multiply(0.45);
    visuals.selection.bg_fill = palette.accent.gamma_multiply(0.45);
    ctx.set_visuals(visuals);
}

fn truncate_label(text: &str, max_chars: usize) -> String {
    if text.chars().count() <= max_chars {
        text.to_string()
    } else {
        let mut s: String = text.chars().take(max_chars.saturating_sub(1)).collect();
        s.push('…');
        s
    }
}
