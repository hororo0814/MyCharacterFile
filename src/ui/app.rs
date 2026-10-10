use crate::domain::character::{Character, CharacterId, Gender, Race};
use crate::repository::character_file::save_store;
use crate::repository::character_store::CharacterStore;
use eframe::egui;

pub struct CharacterApp {
    store: CharacterStore,
    name: String,
}

impl CharacterApp {
    pub fn new(cc: &eframe::CreationContext<'_>, store: CharacterStore) -> Self {
        let mut fonts = egui::FontDefinitions::default();
        let font_data = std::fs::read("C:\\Windows\\Fonts\\meiryo.ttc")
            .expect("日本語フォントを読み込めませんでした");

        fonts.font_data.insert(
            "meiryo".to_owned(),
            egui::FontData::from_owned(font_data).into(),
        );

        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(0, "meiryo".to_owned());

        cc.egui_ctx.set_fonts(fonts);

        //毎回空文字にする
        let blank_name = String::new();

        //キャラクターを返す
        Self {
            store,
            name: blank_name,
        }
    }
}

impl eframe::App for CharacterApp {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _frame: &mut eframe::Frame) {
        eframe::egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("オリジナルキャラクター図鑑");

            //テキスト入力欄
            //名前の入力
            ui.label(format!("名前"));
            ui.text_edit_singleline(&mut self.name);

            //年齢の入力
            ui.label(format!("年齢"));

            //種族の入力
            ui.label(format!("種族"));

            //性別の入力
            ui.label(format!("性別"));

            //能力の入力
            ui.label(format!("所持している能力の設定を行います"));
            ui.label(format!("能力名"));

            //登録ボタンが押されたときの処理
            if ui.button("登録").clicked() {
                //名前が空でないか確認する
                if self.name.trim().is_empty() {
                    ui.label(format!("名前を空欄にすることはできません"));
                } else {
                    //次のIDを決める
                    let next_id = self
                        .store
                        .get_all()
                        .iter()
                        .map(|character| character.id.0)
                        .max()
                        .unwrap_or(0)
                        + 1;
                    //Character::new()で生成
                    let new_character = Character::new(
                        CharacterId(next_id),
                        self.name.clone(),
                        None,
                        Race::Human,
                        Gender::Male,
                        Vec::new(),
                        String::new(),
                    );
                    //self.store.add() で登録
                    match self.store.add(new_character) {
                        Ok(()) => self.name.clear(),
                        Err(error) => {
                            ui.label(error.to_string());
                        }
                    }
                }

                println!("ボタンが押された{}", self.name);
            }

            for character in self.store.get_all() {
                //キャラクターの情報を表示
                let age_text = match character.age {
                    Some(age) => format!("年齢：{}歳", age),
                    None => "年齢不詳".to_string(),
                };
                ui.label(format!("名前：{}", character.name));
                ui.label(format!("{}", age_text));
                ui.label(format!("種族：{}", character.race));
                ui.label(format!("性別：{}", character.gender));
                ui.label(format!("ストーリー：{}", character.story));

                //区切り線で区切る
                ui.separator();

                //能力の情報を表示
                for ability in &character.abilities {
                    ui.label(format!("< {}の所持する能力 >", &character.name,));
                    ui.label(format!("能力名:{}", ability.name,));
                    ui.label(format!("威力:{}", ability.power,));
                    ui.label(format!("タイプ:{}", ability.ability_type,));
                    ui.label(format!("消費MP:{}", ability.cost_mp,));
                }
                ui.separator();
            }
        });
    }
}
