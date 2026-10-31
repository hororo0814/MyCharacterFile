//! 空のストアでタイトル画面を撮影する。ユーザーの保存データには触れない。
#![allow(dead_code)]

#[path = "../src/domain/mod.rs"]
mod domain;
#[path = "../src/repository/mod.rs"]
mod repository;
#[path = "../src/ui/mod.rs"]
mod ui;

use eframe::egui;

struct Preview {
    app: ui::app::CharacterApp,
    frames: usize,
}

impl eframe::App for Preview {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        self.app.ui(ui, frame);
        self.frames += 1;
        let screenshot = ui.ctx().input(|input| {
            input.events.iter().find_map(|event| {
                if let egui::Event::Screenshot { image, .. } = event {
                    Some(image.clone())
                } else {
                    None
                }
            })
        });
        if let Some(image) = screenshot {
            let rgba: Vec<u8> = image
                .pixels
                .iter()
                .flat_map(|pixel| pixel.to_array())
                .collect();
            image::save_buffer(
                "target/title-screen-preview.png",
                &rgba,
                image.width() as u32,
                image.height() as u32,
                image::ColorType::Rgba8,
            )
            .expect("プレビュー画像を保存できませんでした");
            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
        } else if self.frames == 2 {
            ui.ctx()
                .send_viewport_cmd(egui::ViewportCommand::Screenshot(Default::default()));
        } else if self.frames < 2 {
            ui.ctx().request_repaint();
        }
    }
}

fn main() -> eframe::Result {
    eframe::run_native(
        "Title preview",
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default().with_inner_size([1280.0, 720.0]),
            ..Default::default()
        },
        Box::new(|cc| {
            Ok(Box::new(Preview {
                app: ui::app::CharacterApp::new(
                    cc,
                    repository::character_store::CharacterStore::new(),
                ),
                frames: 0,
            }))
        }),
    )
}
