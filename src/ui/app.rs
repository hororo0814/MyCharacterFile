use crate::domain::ability::{Ability, AbilityType};
use crate::domain::character::{Character, CharacterId, Gender, Race};
use crate::repository::character_file::save_store;
use crate::repository::character_store::CharacterStore;
use eframe::egui;

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
        }
    }
}

impl eframe::App for CharacterApp {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _frame: &mut eframe::Frame) {
        eframe::egui::CentralPanel::default().show(ui, |ui| {
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    ui.heading("オリジナルキャラクター図鑑");

                    //キャラクター新規登録
                    //名前の入力
                    ui.heading("新規キャラクター登録欄");
                    ui.label("名前");
                    ui.text_edit_singleline(&mut self.name);

                    //年齢の入力
                    ui.label("年齢");
                    ui.text_edit_singleline(&mut self.age);

                    //種族の選択
                    ui.label("種族");
                    ui.selectable_value(&mut self.race, Race::Human, "人間");
                    ui.selectable_value(&mut self.race, Race::Elf, "エルフ");
                    ui.selectable_value(&mut self.race, Race::Beast, "獣人");
                    ui.selectable_value(&mut self.race, Race::Demon, "悪魔");

                    //性別の選択
                    ui.label("性別");
                    ui.selectable_value(&mut self.gender, Gender::Male, "男性");
                    ui.selectable_value(&mut self.gender, Gender::Female, "女性");
                    ui.selectable_value(&mut self.gender, Gender::Other, "その他");

                    //ストーリーの入力
                    ui.label("ストーリー");
                    ui.text_edit_singleline(&mut self.story);

                    //区切り線で区切る
                    ui.separator();

                    //能力の入力
                    ui.label("所持している能力の設定を行います");
                    ui.label("能力名");
                    ui.text_edit_singleline(&mut self.ability_name);
                    ui.label("威力");
                    ui.text_edit_singleline(&mut self.ability_power);
                    ui.label("能力タイプ");
                    ui.selectable_value(&mut self.ability_type, AbilityType::Attack, "攻撃");
                    ui.selectable_value(&mut self.ability_type, AbilityType::Heal, "回復");
                    ui.selectable_value(&mut self.ability_type, AbilityType::Buff, "バフ");
                    ui.selectable_value(&mut self.ability_type, AbilityType::Debuff, "デバフ");
                    ui.label("消費MP");
                    ui.text_edit_singleline(&mut self.ability_mp);

                    // 能力の追加機能
                    ui.separator();
                    ui.heading("既存キャラクターへの能力追加");

                    for character in self.store.get_all() {
                        ui.selectable_value(
                            &mut self.select_character,
                            Some(character.id),
                            format!("名前：{}", character.name),
                        );
                    }
                    ui.label("能力の追加を行います");
                    ui.label("追加する能力の能力名を記入");
                    ui.text_edit_singleline(&mut self.add_ability_name);
                    ui.label("威力を入力");
                    ui.text_edit_singleline(&mut self.add_ability_power);
                    ui.label("能力タイプを入力");
                    ui.selectable_value(&mut self.add_ability_type, AbilityType::Attack, "攻撃");
                    ui.selectable_value(&mut self.add_ability_type, AbilityType::Heal, "回復");
                    ui.selectable_value(&mut self.add_ability_type, AbilityType::Buff, "バフ");
                    ui.selectable_value(&mut self.add_ability_type, AbilityType::Debuff, "デバフ");
                    ui.label("消費MPを入力");
                    ui.text_edit_singleline(&mut self.add_ability_mp);

                    if ui.button("能力を追加").clicked() {
                        if let Some(id) = self.select_character {
                            //能力名が空欄でないかチェックする
                            if self.add_ability_name.trim().is_empty() {
                                ui.label("能力名を空欄にすることはできません");
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
                                                            ui.label("正しく保存することができませんでした");
                                                        }
                                                    }
                                                }
                                            }
                                            Err(_) => {
                                                ui.label("消費MPは半角数字で入力してください");
                                            }
                                        }
                                    }
                                    Err(_) => {
                                        ui.label("威力は半角数字で入力してください");
                                    }
                                }
                            }
                        } else {
                            ui.label("キャラクターが選択されていません");
                        }
                    }

                    //新規キャラクター登録ボタン
                    ui.label("新規キャラクター登録ボタン");
                    // 登録ボタンが押されたときの処理
                    if ui.button("登録").clicked() {
                        if self.name.trim().is_empty() {
                            ui.label("名前を空欄にすることはできません");
                        } else if self.ability_name.trim().is_empty() {
                            ui.label("能力名を空欄にすることはできません");
                        } else {
                            // 年齢を Option<u32> に変換する
                            let age_result: Result<Option<u32>, std::num::ParseIntError> =
                                if self.age.trim().is_empty() {
                                    Ok(None)
                                } else {
                                    self.age.trim().parse::<u32>().map(Some)
                                };

                            match age_result {
                                Ok(s_age) => {
                                    // 威力を u32 に変換する
                                    match self.ability_power.trim().parse::<u32>() {
                                        Ok(power) => {
                                            // 消費MPを u32 に変換する
                                            match self.ability_mp.trim().parse::<u32>() {
                                                Ok(mp) => {
                                                    // 次のIDを決める
                                                    let next_id = self
                                                        .store
                                                        .get_all()
                                                        .iter()
                                                        .map(|character| character.id.0)
                                                        .max()
                                                        .unwrap_or(0)
                                                        + 1;

                                                    // 能力を生成する
                                                    let new_ability = Ability::new(
                                                        self.ability_name.clone(),
                                                        power,
                                                        self.ability_type.clone(),
                                                        mp,
                                                    );

                                                    // 能力をリストに入れる
                                                    let abilities = vec![new_ability];

                                                    // キャラクターを生成する
                                                    let new_character = Character::new(
                                                        CharacterId(next_id),
                                                        self.name.clone(),
                                                        s_age,
                                                        self.race.clone(),
                                                        self.gender.clone(),
                                                        abilities,
                                                        self.story.clone(),
                                                    );

                                                    // キャラクターを登録する
                                                    match self.store.add(new_character) {
                                                        Ok(()) => {
                                                            // ファイルに保存する
                                                            match save_store(&self.store) {
                                                                Ok(()) => {
                                                                    self.name.clear();
                                                                    self.age.clear();
                                                                    self.story.clear();
                                                                    self.ability_name.clear();
                                                                    self.ability_power.clear();
                                                                    self.ability_mp.clear();
                                                                }
                                                                Err(error) => {
                                                                    ui.label(format!(
                                                                        "保存エラー: {}",
                                                                        error
                                                                    ));
                                                                }
                                                            }
                                                        }
                                                        Err(error) => {
                                                            ui.label(format!(
                                                                "登録エラー: {}",
                                                                error
                                                            ));
                                                        }
                                                    }
                                                }
                                                Err(_) => {
                                                    ui.label("消費MPは半角数字で入力してください");
                                                }
                                            }
                                        }
                                        Err(_) => {
                                            ui.label("威力は半角数字で入力してください");
                                        }
                                    }
                                }
                                Err(_) => {
                                    ui.label("年齢は半角数字で入力してください");
                                }
                            }
                        }
                    }

                    // 登録済みキャラクターを表示する
                    ui.separator();
                    ui.heading("登録済みキャラクター");

                    for character in self.store.get_all() {
                        let age_text = match character.age {
                            Some(age) => format!("年齢：{}歳", age),
                            None => "年齢不詳".to_string(),
                        };

                        ui.label(format!("名前：{}", character.name));
                        ui.label(age_text);
                        ui.label(format!("種族：{}", character.race));
                        ui.label(format!("性別：{}", character.gender));
                        ui.label(format!("ストーリー：{}", character.story));

                        for ability in &character.abilities {
                            ui.label(format!("能力名：{}", ability.name));
                            ui.label(format!("威力：{}", ability.power));
                            ui.label(format!("タイプ：{}", ability.ability_type));
                            ui.label(format!("消費MP：{}", ability.cost_mp));
                        }

                        ui.separator();
                    }
                });
        });
    }
}
