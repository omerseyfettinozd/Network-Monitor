use crate::config::ThemeId;
use eframe::egui::Color32;

#[derive(Debug, Clone, Copy)]
pub struct Palette {
    pub bg: Color32,
    pub card_bg: Color32,
    pub border: Color32,
    pub text_primary: Color32,
    pub text_secondary: Color32,
    pub download: Color32,
    pub upload: Color32,
    pub accent: Color32,
    pub danger: Color32,
    pub graph_bg: Color32,
    pub graph_grid: Color32,
}

impl Palette {
    pub fn from_theme(theme: ThemeId, opacity: f32) -> Self {
        let a = (opacity.clamp(0.55, 1.0) * 255.0).round() as u8;
        let card_a = ((opacity.clamp(0.55, 1.0) * 0.85) * 255.0).round() as u8;
        let graph_a = ((opacity.clamp(0.55, 1.0) * 0.72) * 255.0).round() as u8;

        match theme {
            ThemeId::ObsidianGlass => Self {
                bg: Color32::from_rgba_unmultiplied(15, 19, 28, a),
                card_bg: Color32::from_rgba_unmultiplied(24, 30, 44, card_a),
                border: Color32::from_rgba_unmultiplied(88, 114, 152, 115),
                text_primary: Color32::from_rgb(236, 241, 248),
                text_secondary: Color32::from_rgb(142, 156, 178),
                download: Color32::from_rgb(56, 224, 210),
                upload: Color32::from_rgb(250, 178, 72),
                accent: Color32::from_rgb(99, 179, 237),
                danger: Color32::from_rgb(248, 113, 113),
                graph_bg: Color32::from_rgba_unmultiplied(10, 14, 22, graph_a),
                graph_grid: Color32::from_rgba_unmultiplied(120, 142, 175, 42),
            },
            ThemeId::NordFrost => Self {
                bg: Color32::from_rgba_unmultiplied(46, 52, 64, a),
                card_bg: Color32::from_rgba_unmultiplied(59, 66, 82, card_a),
                border: Color32::from_rgba_unmultiplied(136, 192, 208, 110),
                text_primary: Color32::from_rgb(236, 239, 244),
                text_secondary: Color32::from_rgb(165, 177, 198),
                download: Color32::from_rgb(136, 192, 208),
                upload: Color32::from_rgb(235, 203, 139),
                accent: Color32::from_rgb(129, 161, 193),
                danger: Color32::from_rgb(191, 97, 106),
                graph_bg: Color32::from_rgba_unmultiplied(36, 41, 51, graph_a),
                graph_grid: Color32::from_rgba_unmultiplied(216, 222, 233, 35),
            },
            ThemeId::EarthClassic => Self {
                bg: Color32::from_rgba_unmultiplied(51, 36, 13, a),
                card_bg: Color32::from_rgba_unmultiplied(68, 49, 22, card_a),
                border: Color32::from_rgba_unmultiplied(150, 124, 86, 145),
                text_primary: Color32::from_rgb(244, 218, 176),
                text_secondary: Color32::from_rgb(202, 194, 179),
                download: Color32::from_rgb(163, 217, 165),
                upload: Color32::from_rgb(244, 192, 120),
                accent: Color32::from_rgb(196, 164, 114),
                danger: Color32::from_rgb(232, 122, 104),
                graph_bg: Color32::from_rgba_unmultiplied(38, 27, 10, graph_a),
                graph_grid: Color32::from_rgba_unmultiplied(202, 194, 179, 38),
            },
            ThemeId::CyberPulse => Self {
                bg: Color32::from_rgba_unmultiplied(13, 10, 24, a),
                card_bg: Color32::from_rgba_unmultiplied(25, 19, 44, card_a),
                border: Color32::from_rgba_unmultiplied(180, 92, 255, 130),
                text_primary: Color32::from_rgb(245, 240, 255),
                text_secondary: Color32::from_rgb(164, 148, 196),
                download: Color32::from_rgb(0, 245, 212),
                upload: Color32::from_rgb(255, 86, 178),
                accent: Color32::from_rgb(180, 92, 255),
                danger: Color32::from_rgb(255, 92, 114),
                graph_bg: Color32::from_rgba_unmultiplied(9, 7, 18, graph_a),
                graph_grid: Color32::from_rgba_unmultiplied(180, 92, 255, 42),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn palette_respects_opacity_bounds() {
        let p_low = Palette::from_theme(ThemeId::ObsidianGlass, 0.1);
        let p_high = Palette::from_theme(ThemeId::ObsidianGlass, 1.5);
        assert_eq!(p_low.bg.a(), (0.55_f32 * 255.0).round() as u8);
        assert_eq!(p_high.bg.a(), 255);
    }
}
