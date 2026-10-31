use crate::domain::ability::{Ability, AbilityType};
use crate::domain::character::{CharacterId, Gender, Race};
use crate::repository::character_file::{copy_icon_file, save_store};
use crate::repository::character_store::CharacterStore;
use eframe::egui;
use std::collections::{HashMap, HashSet};

mod assets;
mod exhibition;
mod registration;
mod title;

#[derive(Clone, Copy, PartialEq)]
enum AppTab {
    //図鑑ページ
    Encyclopedia,
    //新規登録ページ
    Register,
    //キャラクター管理ページ
    Manage,
    Results,
}

pub struct CharacterApp {
    store: CharacterStore,
    //キャラクターの情報の入力状態
    name: String,
    age: String,
    race: Race,
    gender: Gender,
    story: String,
    //能力の情報の入力状態
    ability_name: String,
    ability_power: String,
    ability_type: AbilityType,
    ability_mp: String,
    select_character: Option<CharacterId>,
    //既存キャラクターへの追加専用の入力状態
    add_ability_name: String,
    add_ability_power: String,
    add_ability_type: AbilityType,
    add_ability_mp: String,
    //キャラクターの名前検索用の状態
    search_name: String,
    // キャラクター編集用
    edit_name: String,
    edit_age: String,
    edit_race: Race,
    edit_gender: Gender,
    edit_story: String,
    edit_icon_path: Option<String>,
    //確認画面を表示するかどうかを保持する
    display_confirm: bool,
    //削除予定のキャラクターIDを保持
    delete_character_id: Option<CharacterId>,
    active_tab: AppTab,
    error_message: Option<String>,
    icon_path: Option<String>,
    detail_character: Option<CharacterId>,
    icon_textures: HashMap<CharacterId, egui::TextureHandle>,
    on_title: bool,
    visitor_votes: HashSet<CharacterId>,
    vote_message: Option<String>,
    title_media: title::TitleMedia,
}

impl CharacterApp {
    //フォントの読み込み
    pub fn read_fonts(cc: &eframe::CreationContext<'_>) {
        let mut fonts = egui::FontDefinitions::default();
        if let Ok(font_data) = std::fs::read("C:\\Windows\\Fonts\\meiryo.ttc") {
            fonts.font_data.insert(
                "meiryo".to_owned(),
                egui::FontData::from_owned(font_data).into(),
            );

            fonts
                .families
                .entry(egui::FontFamily::Proportional)
                .or_default()
                .insert(0, "meiryo".to_owned());
        }
        // 読み込みに成功しても失敗しても、フォント設定を適用する
        cc.egui_ctx.set_fonts(fonts);
    }

    pub fn new(cc: &eframe::CreationContext<'_>, store: CharacterStore) -> Self {
        //read_fonts()の呼び出し
        Self::read_fonts(cc);
        Self::from_store(store)
    }

    fn from_store(store: CharacterStore) -> Self {
        //初期化する(毎回空文字にする)
        let blank_name = String::new();
        let blank_age = String::new();
        let blank_race = Race::Human;
        let blank_gender = Gender::Male;
        let blank_story = String::new();
        let blank_abi_name = String::new();
        let blank_abi_power = String::new();
        let blank_abi_type = AbilityType::Attack;
        let blank_abi_mp = String::new();
        let first_select_character: Option<CharacterId> = None;
        let blank_add_abi_name: String = String::new();
        let blank_add_abi_power: String = String::new();
        let blank_add_abi_type = AbilityType::Attack;
        let blank_add_abi_mp: String = String::new();
        let first_search_name: String = String::new();
        let blank_edit_name = String::new();
        let blank_edit_age = String::new();
        let blank_edit_race = Race::Human;
        let blank_edit_gender = Gender::Male;
        let blank_edit_story = String::new();
        let first_display_confirm = false;
        let first_delete_confirm: Option<CharacterId> = None;

        //キャラクターを返す
        Self {
            store,
            name: blank_name,
            age: blank_age,
            race: blank_race,
            gender: blank_gender,
            story: blank_story,
            ability_name: blank_abi_name,
            ability_power: blank_abi_power,
            ability_type: blank_abi_type,
            ability_mp: blank_abi_mp,
            select_character: first_select_character,
            add_ability_name: blank_add_abi_name,
            add_ability_power: blank_add_abi_power,
            add_ability_type: blank_add_abi_type,
            add_ability_mp: blank_add_abi_mp,
            search_name: first_search_name,
            edit_name: blank_edit_name,
            edit_age: blank_edit_age,
            edit_gender: blank_edit_gender,
            edit_race: blank_edit_race,
            edit_story: blank_edit_story,
            edit_icon_path: None,
            display_confirm: first_display_confirm,
            delete_character_id: first_delete_confirm,
            active_tab: AppTab::Encyclopedia,
            error_message: None,
            icon_path: None,
            detail_character: None,
            icon_textures: HashMap::new(),
            on_title: true,
            visitor_votes: HashSet::new(),
            vote_message: None,
            title_media: title::TitleMedia::default(),
        }
    }

    //新規キャラクター登録機能
    fn show_registration_ui(&mut self, ui: &mut egui::Ui) {
        //名前の入力
        ui.separator();
        ui.heading("新規キャラクター登録欄");
        ui.separator();
        self.show_icon_picker(ui);
        ui.add_space(12.0);
        egui::Frame::new()
            .fill(egui::Color32::from_rgb(174, 191, 255))
            .inner_margin(egui::Margin::same(10))
            .corner_radius(egui::CornerRadius::same(10))
            .show(ui, |ui| {
                egui::Frame::new()
                    .fill(egui::Color32::from_rgb(215, 215, 255))
                    .inner_margin(egui::Margin::same(10))
                    .corner_radius(egui::CornerRadius::same(8))
                    .show(ui, |ui| {
                        ui.label("名前");
                        ui.add_sized([360.0, 40.0], egui::TextEdit::singleline(&mut self.name));
                    });
                ui.add_space(20.0);

                //年齢の入力
                egui::Frame::new()
                    .fill(egui::Color32::from_rgb(215, 215, 255))
                    .inner_margin(egui::Margin::same(10))
                    .corner_radius(egui::CornerRadius::same(8))
                    .show(ui, |ui| {
                        ui.label("年齢");
                        ui.add_sized([360.0, 40.0], egui::TextEdit::singleline(&mut self.age));
                    });
                ui.add_space(20.0);

                //選択中の種族によって枠線の色を変える
                let race_back_color = match self.race {
                    Race::Human => egui::Color32::from_rgb(255, 104, 102),
                    Race::Beast => egui::Color32::from_rgb(95, 255, 138),
                    Race::Elf => egui::Color32::from_rgb(255, 159, 52),
                    Race::Demon => egui::Color32::from_rgb(100, 100, 255),
                    Race::Other(_) => egui::Color32::from_rgb(100, 100, 255),
                };

                //種族の選択
                egui::Frame::new()
                    .fill(egui::Color32::from_rgb(215, 215, 255))
                    .inner_margin(egui::Margin::same(10))
                    .corner_radius(egui::CornerRadius::same(8))
                    .stroke(egui::Stroke::new(3.0, race_back_color))
                    .show(ui, |ui| {
                        ui.label("種族");
                        ui.allocate_ui_with_layout(
                            egui::vec2(ui.available_width(), 30.0),
                            egui::Layout::left_to_right(egui::Align::Center),
                            |ui| {
                                ui.with_layout(
                                    egui::Layout::left_to_right(egui::Align::Center),
                                    |ui| {
                                        ui.add_space((ui.available_width() / 2.0 - 100.0).max(0.0));

                                        ui.selectable_value(&mut self.race, Race::Human, "人間");
                                        ui.selectable_value(&mut self.race, Race::Elf, "エルフ");
                                        ui.selectable_value(&mut self.race, Race::Beast, "獣人");
                                        ui.selectable_value(&mut self.race, Race::Demon, "悪魔");
                                    },
                                );
                            },
                        );
                    });
                ui.add_space(20.0);

                //選択中の性別によって枠線の色を変える
                let gender_back_color = match self.gender {
                    Gender::Male => egui::Color32::from_rgb(255, 104, 102),
                    Gender::Female => egui::Color32::from_rgb(147, 219, 255),
                    Gender::Other => egui::Color32::from_rgb(255, 159, 52),
                };

                //性別の選択
                egui::Frame::new()
                    .fill(egui::Color32::from_rgb(215, 215, 255))
                    .inner_margin(egui::Margin::same(10))
                    .corner_radius(egui::CornerRadius::same(8))
                    .stroke(egui::Stroke::new(3.0, gender_back_color))
                    .show(ui, |ui| {
                        ui.label("性別");
                        ui.allocate_ui_with_layout(
                            egui::vec2(ui.available_width(), 30.0),
                            egui::Layout::left_to_right(egui::Align::Center),
                            |ui| {
                                ui.with_layout(
                                    egui::Layout::left_to_right(egui::Align::Center),
                                    |ui| {
                                        ui.add_space((ui.available_width() / 2.0 - 100.0).max(0.0));
                                        ui.selectable_value(&mut self.gender, Gender::Male, "男性");
                                        ui.selectable_value(
                                            &mut self.gender,
                                            Gender::Female,
                                            "女性",
                                        );
                                        ui.selectable_value(
                                            &mut self.gender,
                                            Gender::Other,
                                            "その他",
                                        );
                                    },
                                );
                            },
                        );
                    });
                ui.add_space(20.0);

                //ストーリーの入力
                egui::Frame::new()
                    .fill(egui::Color32::from_rgb(215, 215, 255))
                    .inner_margin(egui::Margin::same(10))
                    .corner_radius(egui::CornerRadius::same(8))
                    .show(ui, |ui| {
                        ui.label("ストーリー");
                        ui.text_edit_multiline(&mut self.story);
                    });
                ui.add_space(20.0);

                //能力の入力
                ui.label("能力の設定（任意）");
                ui.label("能力名・威力・消費MPを空欄にすると、能力なしで登録できます。");
                ui.label("能力は登録後に「キャラ管理」から追加できます。");

                egui::Frame::new()
                    .fill(egui::Color32::from_rgb(215, 215, 255))
                    .inner_margin(egui::Margin::same(10))
                    .corner_radius(egui::CornerRadius::same(8))
                    .show(ui, |ui| {
                        ui.label("能力名");
                        ui.add_sized(
                            [360.0, 40.0],
                            egui::TextEdit::singleline(&mut self.ability_name),
                        );
                    });
                ui.add_space(20.0);

                egui::Frame::new()
                    .fill(egui::Color32::from_rgb(215, 215, 255))
                    .inner_margin(egui::Margin::same(10))
                    .corner_radius(egui::CornerRadius::same(8))
                    .show(ui, |ui| {
                        ui.label("威力");
                        ui.add_sized(
                            [360.0, 40.0],
                            egui::TextEdit::singleline(&mut self.ability_power),
                        );
                    });
                ui.add_space(20.0);

                egui::Frame::new()
                    .fill(egui::Color32::from_rgb(215, 215, 255))
                    .inner_margin(egui::Margin::same(10))
                    .corner_radius(egui::CornerRadius::same(8))
                    .show(ui, |ui| {
                        ui.label("能力タイプ");
                        let rgb_color = match self.ability_type {
                            AbilityType::Attack => egui::Color32::from_rgb(255, 104, 102),
                            AbilityType::Heal => egui::Color32::from_rgb(95, 255, 138),
                            AbilityType::Buff => egui::Color32::from_rgb(255, 159, 52),
                            AbilityType::Debuff => egui::Color32::from_rgb(100, 100, 255),
                        };

                        // 白い枠の幅を決める
                        let frame_width = 360.0;

                        ui.allocate_ui_with_layout(
                            egui::vec2(ui.available_width(), 50.0),
                            egui::Layout::top_down(egui::Align::Center),
                            |ui| {
                                ui.allocate_ui_with_layout(
                                    egui::vec2(frame_width, 50.0),
                                    egui::Layout::left_to_right(egui::Align::Center),
                                    |ui| {
                                        egui::Frame::new()
                                            .fill(egui::Color32::WHITE)
                                            .inner_margin(egui::Margin::same(10))
                                            .corner_radius(egui::CornerRadius::same(10))
                                            .stroke(egui::Stroke::new(2.0, rgb_color))
                                            .show(ui, |ui| {
                                                ui.horizontal(|ui| {
                                                    ui.add_space(40.0);
                                                    ui.selectable_value(
                                                        &mut self.ability_type,
                                                        AbilityType::Attack,
                                                        "攻撃",
                                                    );
                                                    ui.selectable_value(
                                                        &mut self.ability_type,
                                                        AbilityType::Heal,
                                                        "回復",
                                                    );
                                                    ui.selectable_value(
                                                        &mut self.ability_type,
                                                        AbilityType::Buff,
                                                        "バフ",
                                                    );
                                                    ui.selectable_value(
                                                        &mut self.ability_type,
                                                        AbilityType::Debuff,
                                                        "デバフ",
                                                    );
                                                });
                                            });
                                    },
                                );
                            },
                        );
                    });
                ui.add_space(20.0);

                egui::Frame::new()
                    .fill(egui::Color32::from_rgb(215, 215, 255))
                    .inner_margin(egui::Margin::same(10))
                    .corner_radius(egui::CornerRadius::same(8))
                    .show(ui, |ui| {
                        ui.label("消費MP");
                        ui.add_sized(
                            [360.0, 40.0],
                            egui::TextEdit::singleline(&mut self.ability_mp),
                        );
                    });
                ui.add_space(20.0);
            });

        ui.add_space(20.0);
        //登録ボタンのデザイン
        let regist_button = egui::Button::new("登録")
            .min_size(egui::vec2(220.0, 44.0))
            .fill(egui::Color32::from_rgb(215, 215, 255));

        // 入力の検証・保存は登録用の関数にまとめる。
        if ui.add(regist_button).clicked() {
            self.error_message = self.register_character().err();
        }
        if let Some(message) = &self.error_message {
            ui.colored_label(egui::Color32::RED, message);
        }
    }

    fn show_icon_picker(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.button("アイコン画像を選択").clicked() {
                match assets::pick_character_image() {
                    Ok(Some(path)) => self.icon_path = Some(path.to_string_lossy().into_owned()),
                    Ok(None) => {}
                    Err(error) => {
                        self.error_message =
                            Some(format!("画像フォルダを開けませんでした: {error}"))
                    }
                }
            }
            ui.label(
                self.icon_path
                    .as_deref()
                    .and_then(|path| std::path::Path::new(path).file_name())
                    .and_then(|name| name.to_str())
                    .unwrap_or("画像未選択"),
            );
            if self.icon_path.is_some() && ui.button("解除").clicked() {
                self.icon_path = None;
            }
        });
    }

    // 一覧は画面幅に応じて折り返し、選択時に詳細ウィンドウを開く
    fn show_character_list(&mut self, ui: &mut egui::Ui) {
        ui.separator();
        ui.heading("登録済みキャラクター");
        if let Some(message) = &self.vote_message {
            ui.label(message);
        }
        let ids: Vec<CharacterId> = self
            .store
            .search_by_name(&self.search_name)
            .iter()
            .map(|character| character.id)
            .collect();
        if ids.is_empty() {
            ui.label("対象のキャラクターが見つかりません");
            return;
        }

        let columns = if ui.available_width() >= 760.0 {
            3
        } else if ui.available_width() >= 500.0 {
            2
        } else {
            1
        };
        let card_width =
            ((ui.available_width() - 14.0 * (columns - 1) as f32) / columns as f32).max(220.0);
        // 縦:横をおよそ3:4にして、正方形に近いカードにする
        let card_height = card_width * 0.75;
        // 投票ボタンの高さも確保し、カード内に収める。
        let icon_size = (card_width * 0.64).min(card_height - 138.0).max(24.0);
        let mut vote_requested = None;
        egui::Grid::new("character_cards").num_columns(columns).spacing([14.0, 14.0]).show(ui, |ui| {
            for (index, id) in ids.iter().enumerate() {
                let character = self.store.find(*id).expect("一覧のIDはストア内に存在する");
                let selected = ui.allocate_ui_with_layout(
                    egui::vec2(card_width, card_height),
                    egui::Layout::top_down(egui::Align::Center),
                    |ui| {
                        let mut clicked = false;
                        egui::Frame::new().fill(egui::Color32::from_rgb(208, 215, 253))
                            .inner_margin(egui::Margin::same(12)).corner_radius(egui::CornerRadius::same(16))
                            .stroke(egui::Stroke::new(2.0, egui::Color32::from_rgb(174, 154, 255)))
                            .show(ui, |ui| {
                                // Frame自体も割り当て領域いっぱいに広げ、全画面でも比率を保つ
                                ui.set_min_size(ui.available_size());
                                if let Some(path) = &character.icon_path {
                                    if !self.icon_textures.contains_key(id) {
                                        if let Ok(bytes) = std::fs::read(path) {
                                            if let Ok(image) = image::load_from_memory(&bytes) {
                                                let image = image.thumbnail(512, 512).to_rgba8();
                                                let size = [image.width() as usize, image.height() as usize];
                                                let color = egui::ColorImage::from_rgba_unmultiplied(size, image.as_raw());
                                                self.icon_textures.insert(*id, ui.ctx().load_texture(format!("character_{}", id.0), color, Default::default()));
                                            }
                                        }
                                    }
                                    if let Some(texture) = self.icon_textures.get(id) {
                                        clicked |= ui
                                            .add(
                                                egui::Image::new((
                                                    texture.id(),
                                                    egui::vec2(icon_size, icon_size),
                                                ))
                                                .sense(egui::Sense::click()),
                                            )
                                            .clicked();
                                    }
                                }
                                if character.icon_path.is_none()
                                    || !self.icon_textures.contains_key(id)
                                {
                                    egui::Frame::new()
                                        .fill(egui::Color32::WHITE)
                                        .corner_radius(egui::CornerRadius::same(12))
                                        .show(ui, |ui| {
                                            ui.allocate_ui_with_layout(
                                                egui::vec2(icon_size, icon_size),
                                                egui::Layout::centered_and_justified(
                                                    egui::Direction::TopDown,
                                                ),
                                                |ui| {
                                                    ui.label("画像なし");
                                                },
                                            );
                                        });
                                }
                                ui.add_space(10.0);
                                ui.allocate_ui_with_layout(
                                    egui::vec2(ui.available_width(), 44.0),
                                    egui::Layout::top_down(egui::Align::Center),
                                    |ui| {
                                        clicked |= ui
                                            .add_sized(
                                                [card_width * 0.68, 40.0],
                                                egui::Button::new(
                                                    egui::RichText::new(&character.name).size(18.0),
                                                ),
                                            )
                                            .clicked();
                                    },
                                );
                                ui.label(
                                    egui::RichText::new(format!(
                                        "{} ・ {}",
                                        character.race, character.gender
                                    ))
                                    .size(14.0),
                                );
                                let voted = self.visitor_votes.contains(id);
                                let label = if voted {
                                    format!("いいね済み  {}票", character.likes)
                                } else {
                                    format!("いいね！  {}票", character.likes)
                                };
                                if ui.add_enabled(
                                    !voted,
                                    egui::Button::new(label)
                                        .min_size(egui::vec2(card_width * 0.58, 30.0))
                                        .fill(egui::Color32::from_rgb(255, 225, 235)),
                                ).clicked() {
                                    vote_requested = Some(*id);
                                }
                            });
                        clicked
                    }
                ).inner;
                if selected { self.detail_character = Some(*id); }
                if (index + 1) % columns == 0 { ui.end_row(); }
            }
        });

        // 描画中のストアへの参照を解放してから投票を確定する。
        if let Some(id) = vote_requested {
            self.cast_vote(id);
            ui.ctx().request_repaint();
        }

        if let Some(id) = self.detail_character {
            let mut open = true;
            egui::Window::new("キャラクター詳細")
                .open(&mut open)
                .show(ui.ctx(), |ui| {
                    if let Some(character) = self.store.find(id) {
                        ui.heading(&character.name);
                        if let Some(texture) = self.icon_textures.get(&id) {
                            ui.image((texture.id(), egui::vec2(160.0, 160.0)));
                        }
                        ui.label(format!(
                            "年齢：{}",
                            character
                                .age
                                .map(|age| format!("{}歳", age))
                                .unwrap_or_else(|| "不詳".to_string())
                        ));
                        ui.label(format!("種族：{}", character.race));
                        ui.label(format!("性別：{}", character.gender));
                        ui.label("ストーリー");
                        ui.label(&character.story);
                        ui.separator();
                        ui.heading("能力一覧");
                        if character.abilities.is_empty() {
                            ui.label("能力は未設定です");
                        }
                        for ability in &character.abilities {
                            ui.label(format!(
                                "{}　威力：{}　タイプ：{}　消費MP：{}",
                                ability.name, ability.power, ability.ability_type, ability.cost_mp
                            ));
                        }
                    }
                });
            if !open {
                self.detail_character = None;
            }
        }
    }
    //キャラクターの編集機能
    fn show_character_edit_ui(&mut self, ui: &mut egui::Ui) {
        //編集フォームの表示
        ui.separator();

        egui::Frame::new()
            //カードの背景色を指定
            .fill(egui::Color32::from_rgb(240, 255, 201))
            //カードの内側に余白を作る
            .inner_margin(egui::Margin::same(16))
            //カードの角を丸くする
            .corner_radius(egui::CornerRadius::same(20))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    if ui.button("アイコン画像を選択・変更").clicked() {
                        match assets::pick_character_image() {
                            Ok(Some(path)) => {
                                self.edit_icon_path = Some(path.to_string_lossy().into_owned())
                            }
                            Ok(None) => {}
                            Err(error) => {
                                self.error_message =
                                    Some(format!("画像フォルダを開けませんでした: {error}"))
                            }
                        }
                    }
                    ui.label(
                        self.edit_icon_path
                            .as_deref()
                            .and_then(|path| std::path::Path::new(path).file_name())
                            .and_then(|name| name.to_str())
                            .unwrap_or("画像未設定"),
                    );
                    if self.edit_icon_path.is_some() && ui.button("画像を外す").clicked() {
                        self.edit_icon_path = None;
                    }
                });

                // 緑色の枠の幅
                let frame_width = 300.0;

                //名前
                ui.allocate_ui_with_layout(
                    egui::vec2(frame_width, 80.0),
                    egui::Layout::top_down(egui::Align::Min),
                    |ui| {
                        egui::Frame::new()
                            .fill(egui::Color32::from_rgb(175, 246, 201))
                            .inner_margin(egui::Margin::same(10))
                            .corner_radius(egui::CornerRadius::same(10))
                            .show(ui, |ui| {
                                ui.label("名前");
                                ui.text_edit_singleline(&mut self.edit_name);
                            });
                    },
                );

                //年齢
                ui.allocate_ui_with_layout(
                    egui::vec2(frame_width, 80.0),
                    egui::Layout::top_down(egui::Align::Min),
                    |ui| {
                        egui::Frame::new()
                            .fill(egui::Color32::from_rgb(175, 246, 201))
                            .inner_margin(egui::Margin::same(10))
                            .corner_radius(egui::CornerRadius::same(10))
                            .show(ui, |ui| {
                                ui.label("年齢");
                                ui.text_edit_singleline(&mut self.edit_age);
                            });
                    },
                );

                //種族
                ui.label("種族");
                ui.allocate_ui_with_layout(
                    egui::vec2(ui.available_width(), 30.0),
                    egui::Layout::left_to_right(egui::Align::Center),
                    |ui| {
                        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                            ui.add_space((ui.available_width() / 2.0 - 100.0).max(0.0));
                            ui.selectable_value(&mut self.edit_race, Race::Human, "人間");
                            ui.selectable_value(&mut self.edit_race, Race::Elf, "エルフ");
                            ui.selectable_value(&mut self.edit_race, Race::Beast, "獣人");
                            ui.selectable_value(&mut self.edit_race, Race::Demon, "悪魔");
                        });
                    },
                );
                //性別の選択
                ui.label("性別");
                ui.allocate_ui_with_layout(
                    egui::vec2(ui.available_width(), 30.0),
                    egui::Layout::left_to_right(egui::Align::Center),
                    |ui| {
                        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                            ui.add_space((ui.available_width() / 2.0 - 100.0).max(0.0));
                            ui.horizontal_centered(|ui| {
                                ui.selectable_value(&mut self.edit_gender, Gender::Male, "男性");
                                ui.selectable_value(&mut self.edit_gender, Gender::Female, "女性");
                                ui.selectable_value(&mut self.edit_gender, Gender::Other, "その他");
                            });
                        })
                    },
                );
                //ストーリー
                ui.allocate_ui_with_layout(
                    egui::vec2(frame_width, 80.0),
                    egui::Layout::top_down(egui::Align::Min),
                    |ui| {
                        egui::Frame::new()
                            .fill(egui::Color32::from_rgb(175, 246, 201))
                            .inner_margin(egui::Margin::same(10))
                            .corner_radius(egui::CornerRadius::same(10))
                            .show(ui, |ui| {
                                ui.label("ストーリー");
                                ui.text_edit_multiline(&mut self.edit_story);
                            });
                    },
                );

                //不正文字
                let invalid_chars = ['|', ',', ';'];

                let has_invalid_char_name = invalid_chars.iter().any(|&ch| self.name.contains(ch));
                let has_invalid_char_story =
                    invalid_chars.iter().any(|&ch| self.story.contains(ch));

                //編集後キャラクター保存ボタン
                if ui.button("編集").clicked() {
                    if self.edit_name.is_empty() {
                        self.error_message = Some("能力名を空欄にすることはできません".to_string());
                    } else if has_invalid_char_name || has_invalid_char_story {
                        self.error_message =
                            Some("'|', ',', ';'を入力に含めることはできません".to_string());
                    } else if let Some(id) = self.select_character {
                        let icon_unchanged = self
                            .store
                            .find(id)
                            .is_some_and(|character| character.icon_path == self.edit_icon_path);
                        let icon_result = if icon_unchanged {
                            Ok(self.edit_icon_path.clone())
                        } else if let Some(path) = &self.edit_icon_path {
                            copy_icon_file(path, id).map(Some)
                        } else {
                            Ok(None)
                        };
                        let icon_path = match icon_result {
                            Ok(path) => path,
                            Err(error) => {
                                self.error_message =
                                    Some(format!("画像を保存できませんでした: {error}"));
                                return;
                            }
                        };
                        let age_result: Result<Option<u32>, std::num::ParseIntError> =
                            if self.edit_age.trim().is_empty() {
                                Ok(None)
                            } else {
                                //年齢をOption<u32>に変換したものを返す
                                self.edit_age.trim().parse::<u32>().map(Some)
                            };
                        if let Some(character) = self.store.find_mut(id) {
                            // キャラクターの各フィールドを更新する
                            character.name = self.edit_name.clone();
                            match age_result {
                                Ok(age) => {
                                    character.age = age;
                                    character.race = self.edit_race.clone();
                                    character.gender = self.edit_gender.clone();
                                    character.story = self.edit_story.clone();
                                    character.icon_path = icon_path;
                                    self.icon_textures.remove(&id);

                                    //データを保存する処理
                                    match save_store(&self.store) {
                                        Ok(()) => {
                                            self.edit_name.clear();
                                            self.edit_age.clear();
                                            self.edit_story.clear();
                                        }
                                        Err(error) => {
                                            ui.label(format!("保存エラー: {}", error));
                                        }
                                    }
                                }
                                Err(_) => {
                                    self.error_message =
                                        Some("年齢は半角英数字で入力してください".to_string());
                                }
                            }
                        }
                    } else {
                        self.error_message = Some("キャラクターが選択されていません".to_string());
                    }
                }

                if let Some(message) = &self.error_message {
                    ui.colored_label(egui::Color32::RED, message);
                }
            });
    }

    //能力の追加機能
    fn show_ability_add_ui(&mut self, ui: &mut egui::Ui) {
        //能力の追加
        egui::Frame::new()
            //カードの背景色を指定
            .fill(egui::Color32::from_rgb(223, 244, 255))
            //カードの内側に余白を作る
            .inner_margin(egui::Margin::same(16))
            //カードの角を丸くする
            .corner_radius(egui::CornerRadius::same(20))
            .show(ui, |ui| {
                ui.heading("能力の追加を行います");

                // 緑色の枠の幅
                let frame_width = 300.0;
                //各項目の背景色(青)
                let blue_back = egui::Color32::from_rgb(197, 227, 255);

                //能力名
                ui.allocate_ui_with_layout(
                    egui::vec2(frame_width, 80.0),
                    egui::Layout::top_down(egui::Align::Min),
                    |ui| {
                        egui::Frame::new()
                            .fill(blue_back)
                            .inner_margin(egui::Margin::same(10))
                            .corner_radius(egui::CornerRadius::same(10))
                            .show(ui, |ui| {
                                ui.label("追加する能力の能力名を記入");
                                ui.text_edit_singleline(&mut self.add_ability_name);
                            });
                    },
                );

                //威力
                ui.allocate_ui_with_layout(
                    egui::vec2(frame_width, 80.0),
                    egui::Layout::top_down(egui::Align::Min),
                    |ui| {
                        egui::Frame::new()
                            .fill(blue_back)
                            .inner_margin(egui::Margin::same(10))
                            .corner_radius(egui::CornerRadius::same(10))
                            .show(ui, |ui| {
                                ui.label("威力を入力");
                                ui.text_edit_singleline(&mut self.add_ability_power);
                            });
                    },
                );

                //能力タイプ
                ui.label("能力タイプを入力");
                ui.allocate_ui_with_layout(
                    egui::vec2(ui.available_width(), 30.0),
                    egui::Layout::left_to_right(egui::Align::Center),
                    |ui| {
                        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                            ui.add_space((ui.available_width() / 2.0 - 100.0).max(0.0));
                            ui.selectable_value(
                                &mut self.add_ability_type,
                                AbilityType::Attack,
                                "攻撃",
                            );
                            ui.selectable_value(
                                &mut self.add_ability_type,
                                AbilityType::Heal,
                                "回復",
                            );
                            ui.selectable_value(
                                &mut self.add_ability_type,
                                AbilityType::Buff,
                                "バフ",
                            );
                            ui.selectable_value(
                                &mut self.add_ability_type,
                                AbilityType::Debuff,
                                "デバフ",
                            );
                        });
                    },
                );
                //消費MP
                ui.allocate_ui_with_layout(
                    egui::vec2(frame_width, 80.0),
                    egui::Layout::top_down(egui::Align::Min),
                    |ui| {
                        egui::Frame::new()
                            .fill(blue_back)
                            .inner_margin(egui::Margin::same(10))
                            .corner_radius(egui::CornerRadius::same(10))
                            .show(ui, |ui| {
                                ui.label("消費MPを入力");
                                ui.text_edit_singleline(&mut self.add_ability_mp);
                            });
                    },
                );

                let invalid_chars = ['|', ',', ';'];
                let has_invalid_char_abi_name = invalid_chars
                    .iter()
                    .any(|&ch| self.add_ability_name.contains(ch));

                if ui.button("能力を追加").clicked() {
                    if let Some(id) = self.select_character {
                        //能力名が空欄でないかチェックする
                        if self.add_ability_name.trim().is_empty() {
                            self.error_message =
                                Some("能力名を空欄にすることはできません".to_string());
                        } else if has_invalid_char_abi_name {
                            self.error_message =
                                Some("'|', ',', ';'を入力に含めることはできません".to_string());
                        } else {
                            // 威力を u32 に変換する
                            match self.add_ability_power.trim().parse::<u32>() {
                                Ok(power) => {
                                    // 消費MPを u32 に変換する
                                    match self.add_ability_mp.trim().parse::<u32>() {
                                        Ok(mp) => {
                                            let add_ability = Ability::new(
                                                self.add_ability_name.clone(),
                                                power,
                                                self.add_ability_type.clone(),
                                                mp,
                                            );
                                            if let Some(character) = self.store.find_mut(id) {
                                                character.abilities.push(add_ability);
                                                match save_store(&self.store) {
                                                    Ok(()) => {
                                                        // 保存成功時の処理
                                                        self.add_ability_name.clear();
                                                        self.add_ability_power.clear();
                                                        self.add_ability_type = AbilityType::Attack;
                                                        self.add_ability_mp.clear();
                                                    }
                                                    Err(_error) => {
                                                        // 保存エラーの表示
                                                        self.error_message = Some(
                                                            "正しく保存することができませんでした"
                                                                .to_string(),
                                                        );
                                                    }
                                                }
                                            }
                                        }
                                        Err(_) => {
                                            self.error_message = Some(
                                                "消費MPは半角数字で入力してください".to_string(),
                                            );
                                        }
                                    }
                                }
                                Err(_) => {
                                    self.error_message =
                                        Some("威力は半角数字で入力してください".to_string());
                                }
                            }
                        }
                    } else {
                        self.error_message = Some("キャラクターが選択されていません".to_string());
                    }
                }

                if let Some(message) = &self.error_message {
                    ui.colored_label(egui::Color32::RED, message);
                }
            });
    }

    //削除確認ウィンドウの表示と削除処理機能
    fn show_delete_confirmation(&mut self, ui: &mut egui::Ui) {
        let mut confirm_delete = false;
        let mut cancel_confirm = false;

        //display_confirmがtrueのときにウィンドウを表示する
        egui::Window::new("削除確認")
            .open(&mut self.display_confirm)
            .show(ui, |ui| {
                // 確認メッセージとボタン
                ui.label("本当に削除しますか？");

                if ui.button("削除する").clicked() {
                    confirm_delete = true;
                }

                if ui.button("キャンセル").clicked() {
                    cancel_confirm = true;
                }
            });

        //キャンセル確認が真のとき，削除ウィンドウを消して偽にする
        if cancel_confirm {
            self.display_confirm = false;
            self.delete_character_id = None;
        }

        //削除確認が真のとき，ウィンドウを閉じて削除処理を行う
        if confirm_delete {
            //IDが見つかればウィンドウを閉じる
            if let Some(id) = self.delete_character_id {
                self.display_confirm = false;

                //削除処理
                match self.store.remove(id) {
                    Some(character) => match save_store(&self.store) {
                        Ok(()) => {
                            self.error_message =
                                Some(format!("キャラクター{}を削除しました", character.name));
                            self.visitor_votes.remove(&id);
                            self.select_character = None;
                            self.edit_name.clear();
                            self.edit_age.clear();
                            self.edit_story.clear();
                        }
                        Err(error) => {
                            self.error_message = Some(format!("保存エラー: {}", error));
                        }
                    },
                    None => {
                        self.error_message = Some("正常に削除できませんでした".to_string());
                    }
                }
            }
        }

        if let Some(message) = &self.error_message {
            ui.colored_label(egui::Color32::RED, message);
        }
    }
}

impl eframe::App for CharacterApp {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _frame: &mut eframe::Frame) {
        if self.on_title {
            egui::CentralPanel::default()
                .frame(egui::Frame::NONE)
                .show(ui, |ui| self.show_title_screen(ui));
            return;
        }

        // スクロールしても右上の戻るボタンを利用できる。
        egui::Panel::top("exhibition_navigation").show(ui, |ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("タイトルに戻る").clicked() {
                    self.return_to_title();
                    ui.ctx().request_repaint();
                }
            });
        });
        if self.on_title {
            return;
        }
        eframe::egui::CentralPanel::default().show(ui, |ui| {
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    ui.vertical_centered(|ui| {


                        // ここに現在の画面全体のUIを置く
                        ui.heading("オリジナルキャラクター図鑑");

                        ui.horizontal(|ui| {
                            for (tab,label) in [
                                (AppTab::Encyclopedia, "図鑑"),
                                (AppTab::Results, "投票結果"),
                                (AppTab::Register, "新規登録"),
                                (AppTab::Manage, "キャラ管理"),
                            ]{
                                //今開いているタブかどうかを判定する
                                let selected = self.active_tab == tab;

                                //背景色を指定する
                                let fill = if selected {
                                    egui::Color32::from_rgb(220, 235, 250)
                                }else{
                                    egui::Color32::from_rgb(245, 247, 250)
                                };

                                //frameという変数に背景，余白，角の丸みなどの見た目の情報を格納する
                                let frame = egui::Frame::new()
                                    .fill(fill)
                                    .inner_margin(egui::Margin::symmetric(16,10))
                                    .corner_radius(egui::CornerRadius::same(8));

                                //active_tabにframeを適用させる
                                frame.show(ui,|ui|{

                                    //クリック時のタブの切り替え
                                    if ui
                                        .selectable_value(&mut self.active_tab,tab,label)
                                        .changed()
                                    {
                                        // タブの選択状態は selectable_value が更新する
                                    }
                                });
                            }
                        });

                        match self.active_tab {
                            AppTab::Results => self.show_voting_results(ui),
                            AppTab::Encyclopedia => {
                                //区切り線で区切る
                                ui.separator();
                                //キャラクター検索欄
                                egui::Frame::new()
                                    .fill(egui::Color32::from_rgb(213, 243, 255))
                                    .inner_margin(egui::Margin::same(10))
                                    .corner_radius(egui::CornerRadius::same(6))
                                    .show(ui,|ui|{
                                        ui.label("名前で検索");
                                        ui.text_edit_singleline(&mut self.search_name);
                                    });
                                //区切り線で区切る
                                ui.separator();

                                //キャラクターの表示機能
                                self.show_character_list(ui);
                            },
                            AppTab::Register => {
                                //登録機能
                                self.show_registration_ui(ui);
                            }
                            AppTab::Manage => {
                                ui.separator();
                                ui.heading("既存キャラクターの編集");
                                ui.separator();

                                for character in self.store.get_all() {
                                    ui.allocate_ui_with_layout(
                                        egui::vec2(ui.available_width(), 30.0),
                                        egui::Layout::left_to_right(egui::Align::Center),
                                        |ui| {
                                            ui.with_layout(
                                                egui::Layout::left_to_right(egui::Align::Center),
                                                |ui| {
                                                    ui.add_space(
                                                        (ui.available_width() / 2.0 - 100.0).max(0.0),
                                                    );
                                                    let changed = ui.selectable_value(
                                                        &mut self.select_character,
                                                        Some(character.id),
                                                        character.name.clone(),
                                                    );

                                                    //削除ボタンが押されたキャラクターを削除する
                                                    if ui.button("削除").clicked() {
                                                        self.display_confirm = true;
                                                        self.delete_character_id = Some(character.id);
                                                    }

                                                    if changed.changed() {
                                                        // 選択したキャラクターの情報をself.edit_name などにコピーする
                                                        if let Some(id) = self.select_character {
                                                            match self.store.find(id) {
                                                                Some(character) => {
                                                                    self.edit_name = character.name.clone();
                                                                    self.edit_age = match character.age {
                                                                        Some(age) => age.to_string(),
                                                                        None => String::new(),
                                                                    };
                                                                    self.edit_gender =
                                                                        character.gender.clone();
                                                                    self.edit_race = character.race.clone();
                                                                    self.edit_story =
                                                                        character.story.clone();
                                                                    self.edit_icon_path =
                                                                        character.icon_path.clone();
                                                                }
                                                                None => {
                                                                    ui.label(
                                                                        "キャラクターが見つかりません",
                                                                    );
                                                                }
                                                            }
                                                        }
                                                    }
                                                },
                                            )
                                        },
                                    );
                                }

                                //編集機能
                                self.show_character_edit_ui(ui);

                                //能力の追加
                                self.show_ability_add_ui(ui);
                            }
                        }
                    });
                });

            //削除確認機能
            self.show_delete_confirmation(ui);
        });
    }
}
