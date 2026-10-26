use super::{AppTab, CharacterApp};
use crate::domain::character::CharacterId;
use crate::repository::character_file::save_store;
use eframe::egui;

impl CharacterApp {
    pub(super) fn show_title_screen(&mut self, ui: &mut egui::Ui) {
        // スクロール領域の外で、有限のウィンドウ高さから余白を求める。
        let top_padding = (ui.available_height() * 0.16).max(24.0);
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.add_space(top_padding);
            ui.vertical_centered(|ui| {
                ui.set_max_width(680.0);
                egui::Frame::new()
                    .fill(egui::Color32::from_rgb(208, 215, 253))
                    .inner_margin(egui::Margin::same(24))
                    .corner_radius(egui::CornerRadius::same(20))
                    .stroke(egui::Stroke::new(
                        2.0,
                        egui::Color32::from_rgb(174, 154, 255),
                    ))
                    .show(ui, |ui| {
                        ui.add_space(20.0);
                        ui.label(
                            egui::RichText::new("オリジナルキャラクター図鑑")
                                .size(32.0)
                                .strong(),
                        );
                        ui.add_space(16.0);
                        ui.label("お気に入りのキャラクターを見つけよう");
                        ui.label("イラストや物語を楽しんで、好きなキャラクターにいいね！");
                        ui.add_space(28.0);
                        if ui
                            .add_sized(
                                [240.0_f32.min(ui.available_width()), 56.0],
                                egui::Button::new(egui::RichText::new("図鑑を見る").size(22.0))
                                    .fill(egui::Color32::WHITE),
                            )
                            .clicked()
                        {
                            self.on_title = false;
                            self.active_tab = AppTab::Encyclopedia;
                            ui.ctx().request_repaint();
                        }
                        ui.add_space(20.0);
                        ui.label(format!(
                            "{}人のキャラクターが待っています",
                            self.store.get_all().len()
                        ));
                        ui.add_space(12.0);
                    });
                ui.add_space(16.0);
                ui.label("見終わったら「タイトルに戻る」で次の方へどうぞ。");
            });
        });
    }

    pub(super) fn return_to_title(&mut self) {
        self.on_title = true;
        self.active_tab = AppTab::Encyclopedia;
        self.visitor_votes.clear();
        self.vote_message = None;
        self.search_name.clear();
        self.detail_character = None;
        self.display_confirm = false;
        self.delete_character_id = None;
        self.error_message = None;
    }

    pub(super) fn cast_vote(&mut self, id: CharacterId) {
        if self.visitor_votes.contains(&id) {
            return;
        }
        let Some(character) = self.store.find_mut(id) else {
            self.vote_message = Some("キャラクターが見つかりません。".to_string());
            return;
        };
        let previous_likes = character.likes;
        let Some(next_likes) = previous_likes.checked_add(1) else {
            self.vote_message = Some("票数が上限に達しているため投票できません。".to_string());
            return;
        };
        let name = character.name.clone();
        character.likes = next_likes;

        match save_store(&self.store) {
            Ok(()) => {
                self.visitor_votes.insert(id);
                self.vote_message = Some(format!(
                    "{name}にいいねしました！ 投票ありがとうございます。"
                ));
            }
            Err(error) => {
                // 保存できなかった票は集計せず、再度投票できる状態に戻す。
                if let Some(character) = self.store.find_mut(id) {
                    character.likes = previous_likes;
                }
                self.vote_message = Some(format!(
                    "投票を保存できませんでした。もう一度お試しください: {error}"
                ));
            }
        }
    }

    pub(super) fn show_voting_results(&self, ui: &mut egui::Ui) {
        ui.separator();
        ui.heading("みんなのいいねランキング");
        ui.label("このアプリでの投票を、すぐに集計へ反映します。");

        let mut characters: Vec<_> = self.store.get_all().iter().collect();
        characters.sort_by(|a, b| b.likes.cmp(&a.likes).then_with(|| a.id.0.cmp(&b.id.0)));
        let total: u128 = characters
            .iter()
            .map(|character| u128::from(character.likes))
            .sum();
        ui.add_space(12.0);
        ui.label(
            egui::RichText::new(format!("合計 {total} 票"))
                .size(26.0)
                .strong(),
        );
        ui.add_space(12.0);

        if characters.is_empty() {
            ui.label("まだキャラクターが登録されていません。");
            return;
        }
        if total == 0 {
            ui.label("まだ投票がありません。図鑑から最初のいいねを送ってみましょう！");
            ui.add_space(12.0);
        }

        let max_likes = characters[0].likes.max(1);
        let mut rank = 0;
        let mut previous_likes = None;
        for (index, character) in characters.iter().enumerate() {
            if previous_likes != Some(character.likes) {
                rank = index + 1;
            }
            previous_likes = Some(character.likes);
            let rank_label = if character.likes == 0 {
                "未投票".to_string()
            } else {
                format!("{rank}位")
            };
            let leading = rank == 1 && character.likes > 0;
            egui::Frame::new()
                .fill(if leading {
                    egui::Color32::from_rgb(255, 242, 207)
                } else {
                    egui::Color32::from_rgb(230, 232, 255)
                })
                .inner_margin(egui::Margin::same(16))
                .corner_radius(egui::CornerRadius::same(12))
                .show(ui, |ui| {
                    ui.set_min_width((ui.available_width().min(840.0) - 4.0).max(0.0));
                    ui.horizontal_wrapped(|ui| {
                        ui.label(egui::RichText::new(rank_label).size(20.0).strong());
                        ui.label(egui::RichText::new(&character.name).size(22.0));
                        ui.label(format!("{} 票", character.likes));
                    });
                    ui.add_space(8.0);
                    ui.add(
                        egui::ProgressBar::new(character.likes as f32 / max_likes as f32)
                            .desired_width(ui.available_width())
                            .fill(egui::Color32::from_rgb(174, 154, 255))
                            .text(format!("{} いいね", character.likes)),
                    );
                });
            ui.add_space(10.0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::character::{Character, Gender, Race};
    use crate::repository::character_store::CharacterStore;

    fn exhibition_app() -> CharacterApp {
        let mut store = CharacterStore::new();
        for id in 1..=4 {
            let mut character = Character::new(
                CharacterId(id),
                format!("Character {id}"),
                None,
                Race::Human,
                Gender::Other,
                vec![],
                String::new(),
            );
            character.likes = id;
            store.add(character).unwrap();
        }
        CharacterApp::from_store(store)
    }

    #[test]
    fn returning_to_title_starts_a_new_visitor_and_keeps_totals() {
        let mut app = exhibition_app();
        assert!(app.on_title);
        app.on_title = false;
        app.active_tab = AppTab::Results;
        app.visitor_votes.insert(CharacterId(1));
        app.search_name = "previous search".to_string();
        app.detail_character = Some(CharacterId(1));
        app.display_confirm = true;
        app.return_to_title();

        assert!(app.on_title);
        assert!(app.active_tab == AppTab::Encyclopedia);
        assert!(app.visitor_votes.is_empty());
        assert!(app.search_name.is_empty());
        assert!(app.detail_character.is_none());
        assert!(!app.display_confirm);
        assert_eq!(app.store.find(CharacterId(1)).unwrap().likes, 1);
    }

    #[test]
    fn duplicate_vote_in_same_visit_is_ignored() {
        let mut app = exhibition_app();
        app.visitor_votes.insert(CharacterId(1));
        app.cast_vote(CharacterId(1));
        assert_eq!(app.store.find(CharacterId(1)).unwrap().likes, 1);
    }

    #[test]
    fn exhibition_screens_render_at_window_and_fullscreen_sizes() {
        for size in [egui::vec2(800.0, 600.0), egui::vec2(1920.0, 1080.0)] {
            let context = egui::Context::default();
            let mut app = exhibition_app();
            for screen in 0..3 {
                let input = egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, size)),
                    ..Default::default()
                };
                let mut output = context.run_ui(input, |ui| {
                    egui::CentralPanel::default().show(ui, |ui| match screen {
                        0 => app.show_title_screen(ui),
                        1 => app.show_character_list(ui),
                        _ => app.show_voting_results(ui),
                    });
                });
                // このテストはGPUを使わず、フォントのテクスチャ更新を破棄する。
                output.textures_delta.clear();
                assert!(!output.shapes.is_empty());
            }
        }
    }
}
