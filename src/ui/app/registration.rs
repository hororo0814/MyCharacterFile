use super::CharacterApp;
use crate::domain::ability::{Ability, AbilityType};
use crate::domain::character::{Character, CharacterId};
use crate::repository::character_file::{copy_icon_file, save_store};

impl CharacterApp {
    /// 能力欄がすべて空なら能力なし。一部入力されていれば従来どおり検証する。
    fn registration_abilities(&self) -> Result<Vec<Ability>, String> {
        let name = self.ability_name.trim();
        let power = self.ability_power.trim();
        let mp = self.ability_mp.trim();
        if name.is_empty() && power.is_empty() && mp.is_empty() {
            return Ok(Vec::new());
        }
        if name.is_empty() {
            return Err("能力を設定する場合は能力名を入力してください".to_string());
        }
        let power = power
            .parse::<u32>()
            .map_err(|_| "威力は半角数字で入力してください".to_string())?;
        let mp = mp
            .parse::<u32>()
            .map_err(|_| "消費MPは半角数字で入力してください".to_string())?;
        Ok(vec![Ability::new(
            self.ability_name.clone(),
            power,
            self.ability_type.clone(),
            mp,
        )])
    }

    // 入力の検証とキャラクター生成を、ファイル保存から分離する。
    fn registration_character(&self) -> Result<Character, String> {
        if self.name.trim().is_empty() {
            return Err("名前を空欄にすることはできません".to_string());
        }
        if [&self.name, &self.story, &self.ability_name]
            .iter()
            .any(|value| value.contains(['|', ',', ';']))
        {
            return Err("'|', ',', ';'を入力に含めることはできません".to_string());
        }
        let age = if self.age.trim().is_empty() {
            None
        } else {
            Some(
                self.age
                    .trim()
                    .parse::<u32>()
                    .map_err(|_| "年齢は半角数字で入力してください".to_string())?,
            )
        };
        let abilities = self.registration_abilities()?;
        let next_id = self
            .store
            .get_all()
            .iter()
            .map(|character| character.id.0)
            .max()
            .unwrap_or(0)
            .checked_add(1)
            .ok_or_else(|| "キャラクターIDの上限に達しました".to_string())?;
        Ok(Character::new(
            CharacterId(next_id),
            self.name.clone(),
            age,
            self.race.clone(),
            self.gender.clone(),
            abilities,
            self.story.clone(),
        ))
    }

    pub(super) fn register_character(&mut self) -> Result<(), String> {
        let mut character = self.registration_character()?;
        if let Some(path) = &self.icon_path {
            character.icon_path = Some(
                copy_icon_file(path, character.id)
                    .map_err(|error| format!("画像を保存できませんでした: {error}"))?,
            );
        }
        let id = character.id;
        self.store
            .add(character)
            .map_err(|error| format!("登録エラー: {error}"))?;
        if let Err(error) = save_store(&self.store) {
            self.store.remove(id);
            return Err(format!("保存エラー: {error}"));
        }
        self.name.clear();
        self.age.clear();
        self.story.clear();
        self.ability_name.clear();
        self.ability_power.clear();
        self.ability_mp.clear();
        self.ability_type = AbilityType::Attack;
        self.icon_path = None;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository::character_store::CharacterStore;

    fn registration_form() -> CharacterApp {
        let mut app = CharacterApp::from_store(CharacterStore::new());
        app.name = "亜紀".to_string();
        app
    }

    #[test]
    fn character_can_be_created_without_an_ability() {
        let mut app = registration_form();
        assert!(app.registration_character().unwrap().abilities.is_empty());
        app.ability_name = "  ".to_string();
        app.ability_power = " ".to_string();
        app.ability_mp = " ".to_string();
        app.ability_type = AbilityType::Heal;
        assert!(app.registration_character().unwrap().abilities.is_empty());
    }

    #[test]
    fn partially_entered_ability_is_not_discarded() {
        for (name, power, mp) in [
            ("", "10", "0"),
            ("フレア", "", "0"),
            ("フレア", "10", ""),
            ("フレア", "abc", "0"),
            ("フレア", "10", "-1"),
        ] {
            let mut app = registration_form();
            app.ability_name = name.to_string();
            app.ability_power = power.to_string();
            app.ability_mp = mp.to_string();
            assert!(app.registration_character().is_err());
        }
    }

    #[test]
    fn entered_ability_is_preserved_including_zero_values() {
        let mut app = registration_form();
        app.ability_name = "回復".to_string();
        app.ability_power = "0".to_string();
        app.ability_mp = "0".to_string();
        app.ability_type = AbilityType::Heal;
        let character = app.registration_character().unwrap();
        assert_eq!(
            character.abilities,
            vec![Ability::new("回復".to_string(), 0, AbilityType::Heal, 0)]
        );
    }

    #[test]
    fn character_validation_still_applies_without_abilities() {
        let mut app = registration_form();
        app.name.clear();
        assert!(app.registration_character().is_err());
        app.name = "亜紀".to_string();
        app.age = "abc".to_string();
        assert!(app.registration_character().is_err());
    }
}
