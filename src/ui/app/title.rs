use super::{AppTab, CharacterApp};
use eframe::egui::{self, Color32, Pos2, Rect, Vec2};

// 素材をexeへ埋め込み、展示先でも元ファイルのパスに依存しない。
const BACKGROUND: &[u8] = include_bytes!("../../../resources/title/background.png");
const LOGO: &[u8] = include_bytes!("../../../resources/title/logo.png");

#[derive(Default)]
pub(super) struct TitleMedia {
    background: Option<egui::TextureHandle>,
    logo: Option<egui::TextureHandle>,
    logo_uv: Option<Rect>,

    failed: bool,
}

impl TitleMedia {
    fn load_logo(&mut self, context: &egui::Context) {
        if self.logo.is_some() {
            return;
        }
        if let Ok(image) = image::load_from_memory(LOGO) {
            // 古い展示PCのテクスチャ上限にも収め、十分な表示解像度を保つ。
            let rgba = image.thumbnail(1400, 900).to_rgba8();
            // 元PNGの透明余白をUVで除き、ロゴ本体を中央に配置する。
            let mut min = [rgba.width(), rgba.height()];
            let mut max = [0, 0];
            for (x, y, pixel) in rgba.enumerate_pixels() {
                if pixel[3] > 8 {
                    min[0] = min[0].min(x);
                    min[1] = min[1].min(y);
                    max[0] = max[0].max(x + 1);
                    max[1] = max[1].max(y + 1);
                }
            }
            self.logo_uv = Some(if max[0] > min[0] && max[1] > min[1] {
                Rect::from_min_max(
                    Pos2::new(
                        min[0] as f32 / rgba.width() as f32,
                        min[1] as f32 / rgba.height() as f32,
                    ),
                    Pos2::new(
                        max[0] as f32 / rgba.width() as f32,
                        max[1] as f32 / rgba.height() as f32,
                    ),
                )
            } else {
                Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0))
            });
            self.logo = Some(context.load_texture(
                "title-logo",
                color_image(&rgba),
                egui::TextureOptions::LINEAR,
            ));
        }
    }

    fn load(&mut self, context: &egui::Context) {
        self.load_logo(context);
        if self.background.is_some() || self.failed {
            return;
        }
        match image::load_from_memory(BACKGROUND) {
            Ok(image) => {
                // 一度だけ読み込み、画面の再描画やタイトルへの復帰時にも再利用する。
                let rgba = image.thumbnail(2048, 2048).to_rgba8();
                self.background = Some(context.load_texture(
                    "title-background",
                    color_image(&rgba),
                    egui::TextureOptions::LINEAR,
                ));
            }
            Err(error) => {
                eprintln!("タイトル背景の読み込みに失敗しました: {error}");
                self.failed = true;
            }
        }
    }
}
fn color_image(image: &image::RgbaImage) -> egui::ColorImage {
    egui::ColorImage::from_rgba_unmultiplied(
        [image.width() as usize, image.height() as usize],
        image.as_raw(),
    )
}

/// 画像の比率を保って画面を埋める。縦長画面では人物のいる右側を残す。
fn background_uv(image: Vec2, viewport: Vec2) -> Rect {
    let image_ratio = image.x / image.y;
    let viewport_ratio = viewport.x / viewport.y;
    if viewport_ratio > image_ratio {
        let height = image_ratio / viewport_ratio;
        Rect::from_min_max(
            Pos2::new(0.0, (1.0 - height) * 0.35),
            Pos2::new(1.0, (1.0 - height) * 0.35 + height),
        )
    } else {
        let width = viewport_ratio / image_ratio;
        Rect::from_min_max(Pos2::new(1.0 - width, 0.0), Pos2::new(1.0, 1.0))
    }
}

fn title_layout(viewport: Rect, logo_ratio: f32) -> (Rect, Rect, Rect) {
    let narrow = viewport.width() / viewport.height() < 1.2;
    let margin = (viewport.width() * 0.045).clamp(12.0, 80.0);
    let width = if narrow {
        (viewport.width() - margin * 2.0).min(480.0)
    } else {
        (viewport.width() * 0.43).min(680.0)
    };
    let padding = (width * 0.07).clamp(12.0, 36.0);
    let button_height = (viewport.height() * 0.07).clamp(40.0, 64.0);
    let gap = (viewport.height() * 0.035).clamp(12.0, 32.0);
    let logo_width = (width - padding * 2.0).min(viewport.height() * 0.43 * logo_ratio);
    let logo_height = logo_width / logo_ratio;
    let height = padding * 2.0 + logo_height + gap + button_height;
    let left = if narrow {
        viewport.center().x - width / 2.0
    } else {
        viewport.left() + margin
    };
    let top = if narrow {
        viewport.bottom() - margin - height
    } else {
        viewport.center().y - height / 2.0
    };
    let panel = Rect::from_min_size(Pos2::new(left, top), egui::vec2(width, height));
    let logo = Rect::from_center_size(
        Pos2::new(panel.center().x, top + padding + logo_height / 2.0),
        egui::vec2(logo_width, logo_height),
    );
    let button = Rect::from_center_size(
        Pos2::new(panel.center().x, logo.bottom() + gap + button_height / 2.0),
        egui::vec2((width * 0.72).min(320.0), button_height),
    );
    (panel, logo, button)
}

impl CharacterApp {
    pub(super) fn show_title_screen(&mut self, ui: &mut egui::Ui) {
        self.title_media.load(ui.ctx());
        let viewport = ui.max_rect();
        ui.allocate_rect(viewport, egui::Sense::hover());
        let painter = ui.painter();
        painter.rect_filled(viewport, 0, Color32::from_rgb(31, 47, 67));
        if let Some(texture) = &self.title_media.background {
            painter.image(
                texture.id(),
                viewport,
                background_uv(texture.size_vec2(), viewport.size()),
                Color32::WHITE,
            );
        }
        painter.rect_filled(viewport, 0, Color32::from_black_alpha(18));
        let logo_uv = self
            .title_media
            .logo_uv
            .unwrap_or(Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)));
        let logo_ratio = self
            .title_media
            .logo
            .as_ref()
            .map(|texture| {
                let size = texture.size_vec2() * logo_uv.size();
                size.x / size.y
            })
            .unwrap_or(1.7);
        let (panel, logo, button) = title_layout(viewport, logo_ratio);
        painter.rect_filled(
            panel.translate(egui::vec2(0.0, 8.0)).expand(2.0),
            24,
            Color32::from_black_alpha(40),
        );
        painter.rect_filled(
            panel,
            24,
            Color32::from_rgba_unmultiplied(246, 249, 255, 225),
        );
        painter.rect_stroke(
            panel,
            24,
            egui::Stroke::new(1.0, Color32::from_white_alpha(190)),
            egui::StrokeKind::Inside,
        );
        if let Some(texture) = &self.title_media.logo {
            painter.image(texture.id(), logo, logo_uv, Color32::WHITE);
        }
        let hovered = ui.rect_contains_pointer(button);
        let response = ui
            .put(
                button,
                egui::Button::new(
                    egui::RichText::new("図鑑を見る")
                        .size((button.height() * 0.38).clamp(17.0, 25.0))
                        .color(Color32::WHITE)
                        .strong(),
                )
                .min_size(button.size())
                .corner_radius(12)
                .fill(if hovered {
                    Color32::from_rgb(45, 106, 173)
                } else {
                    Color32::from_rgb(8, 50, 96)
                })
                .stroke(egui::Stroke::new(1.0, Color32::from_rgb(87, 136, 184))),
            )
            .on_hover_cursor(egui::CursorIcon::PointingHand);
        if response.clicked() {
            self.on_title = false;
            self.active_tab = AppTab::Encyclopedia;
            ui.ctx().request_repaint();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn title_controls_fit_window_fullscreen_and_portrait() {
        for size in [
            egui::vec2(800.0, 600.0),
            egui::vec2(1920.0, 1080.0),
            egui::vec2(390.0, 844.0),
        ] {
            let viewport = Rect::from_min_size(Pos2::ZERO, size);
            let (panel, logo, button) = title_layout(viewport, 1.7);
            assert!(viewport.contains_rect(panel));
            assert!(panel.contains_rect(logo));
            assert!(panel.contains_rect(button));
            assert!(logo.bottom() < button.top());
        }
    }

    #[test]
    fn static_background_is_loaded_once_and_reused() {
        assert_eq!(
            image::guess_format(BACKGROUND).unwrap(),
            image::ImageFormat::Png
        );
        let context = egui::Context::default();
        let mut media = TitleMedia::default();
        let mut first_id = None;
        for _ in 0..2 {
            let mut output = context.run_ui(Default::default(), |ui| {
                media.load(ui.ctx());
                let texture = media.background.as_ref().unwrap();
                assert!(texture.size()[0] <= 2048 && texture.size()[1] <= 2048);
                if let Some(id) = first_id {
                    assert_eq!(texture.id(), id);
                } else {
                    first_id = Some(texture.id());
                }
            });
            output.textures_delta.clear();
        }
    }
}
