use crate::Race;
use crate::domain::ability::{Ability, AbilityType};
use std::fs;

use crate::domain::character::{Character, CharacterId, Gender};
use crate::repository::character_store::{CharacterStore, CharacterStoreErr};

///キャラクターの情報を保存用の文字列に変換する．
pub fn character_to_text(character: &Character) -> String {
    //年齢はu32型なのでString型に変換する
    let age = match character.age {
        Some(age) => age.to_string(),
        None => String::new(),
    };

    //Abilityについても変換を行う
    let mut abilities_text = String::new();

    for (index, ability) in character.abilities.iter().enumerate() {
        if index == 0 {
            let ability_text = format!(
                "{},{},{},{}",
                ability.name, ability.power, ability.ability_type, ability.cost_mp
            );
            abilities_text.push_str(&ability_text);
        } else {
            let ability_text = format!(
                ";{},{},{},{}",
                ability.name, ability.power, ability.ability_type, ability.cost_mp
            );
            abilities_text.push_str(&ability_text);
        }
    }

    format!(
        "{}|{}|{}|{}|{}|{}|{}",
        character.id,
        character.name,
        age,
        character.race,
        character.gender,
        abilities_text,
        character.story
    )
    .to_string()
}

//Store内のCharacterをすべて文字列にする．
pub fn store_character_to_text(store: &CharacterStore) -> String {
    let mut text = String::new();
    for character in store {
        text.push_str(&character_to_text(character));
        text.push('\n');
    }
    text
}

//文字列をファイルに保存，できなければエラーを出す
pub fn save_store(store: &CharacterStore) -> Result<(), CharacterStoreErr> {
    let store_text = store_character_to_text(store);
    fs::write("file.txt", store_text)?;
    Ok(())
}

//ファイルから文字列を読み込む
pub fn load_text() -> Result<CharacterStore, CharacterStoreErr> {
    let text = fs::read_to_string("file.txt")?;
    text_to_store(&text)
}

//取得した文字列をCharacterStoreに変換
fn text_to_store(text: &str) -> Result<CharacterStore, CharacterStoreErr> {
    println!("読み込んだ文字列: {:?}", text);

    let mut store = CharacterStore::new();

    for line in text.lines().enumerate() {
        if line.1.is_empty() {
            continue;
        }
        let mut fields = line.1.split('|');

        //idについて，&str型をu64型にして，それをCharacterId型にする
        let id_text = fields
            .next()
            .ok_or(CharacterStoreErr::InvalidFormat("file.txt".to_string()))?;
        let id: u64 = id_text
            .parse()
            .map_err(|_| CharacterStoreErr::InvalidFormat("file.txt".to_string()))?;
        let character_id = CharacterId(id);

        //nameについて，&strをStringにする．
        let name_text = fields
            .next()
            .ok_or(CharacterStoreErr::InvalidFormat("file.txt".to_string()))?;
        let character_name: String = name_text.to_string();

        //ageについて，&strをu32型にして，それをOption<>に入れる
        let age_text = fields
            .next()
            .ok_or(CharacterStoreErr::InvalidFormat("file.txt".to_string()))?;
        let character_age: Option<u32> = if age_text.is_empty() {
            None
        } else {
            Some(
                age_text
                    .parse()
                    .map_err(|_| CharacterStoreErr::InvalidFormat("file.txt".to_string()))?,
            )
        };

        //raceについて，&strをOtherの場合とそうでない場合に分けて処理する
        let race_text = fields
            .next()
            .ok_or(CharacterStoreErr::InvalidFormat("file.txt".to_string()))?;
        let character_race = match race_text {
            "Human" => Race::Human,
            "Demon" => Race::Demon,
            "Elf" => Race::Elf,
            "Beast" => Race::Beast,
            other => Race::Other(other.to_string()),
        };

        //genderについて
        let gender_text = fields
            .next()
            .ok_or(CharacterStoreErr::InvalidFormat("file.txt".to_string()))?;
        let character_gender = match gender_text {
            "Male" => Gender::Male,
            "Female" => Gender::Female,
            _ => Gender::Other,
        };

        //abilityについて
        let abilities_text = fields
            .next()
            .ok_or(CharacterStoreErr::InvalidFormat("file.txt".to_string()))?;
        let ability_text = abilities_text.split(";");

        let mut abilities = Vec::new();
        if !abilities_text.is_empty() {
            for one_ability in ability_text {
                let mut ability_fields = one_ability.split(',');

                let abi_name = ability_fields
                    .next()
                    .ok_or(CharacterStoreErr::InvalidFormat("file.txt".to_string()))?;
                let abi_power = ability_fields
                    .next()
                    .ok_or(CharacterStoreErr::InvalidFormat("file.txt".to_string()))?
                    .parse()
                    .map_err(|_| CharacterStoreErr::InvalidFormat("file.txt".to_string()))?;
                let abi_type = ability_fields
                    .next()
                    .ok_or(CharacterStoreErr::InvalidFormat("file.txt".to_string()))?;
                let abi_mp = ability_fields
                    .next()
                    .ok_or(CharacterStoreErr::InvalidFormat("file.txt".to_string()))?
                    .parse()
                    .map_err(|_| CharacterStoreErr::InvalidFormat("file.txt".to_string()))?;

                //abilityの名前
                let chara_abi_name: String = abi_name.to_string();

                //abilityのパワー
                let chara_abi_power: u32 = abi_power;

                //abilityのタイプ
                let chara_abi_type = match abi_type {
                    "Attack" => AbilityType::Attack,
                    "Buff" => AbilityType::Buff,
                    "Debuff" => AbilityType::Debuff,
                    "Heal" => AbilityType::Heal,
                    _ => panic!("未知のAbilityTypeです"),
                };

                //abilityのcost_mp
                let chara_abi_mp: u32 = abi_mp;

                let ability = Ability::new(
                    chara_abi_name,
                    chara_abi_power,
                    chara_abi_type,
                    chara_abi_mp,
                );
                abilities.push(ability);
            }
        }

        //storyについて
        let story_text = fields
            .next()
            .ok_or(CharacterStoreErr::InvalidFormat("file.txt".to_string()))?;
        if story_text.len() > 120 {
            return Err(CharacterStoreErr::WordLenError());
        }
        let character_story: String = story_text.to_string();

        //キャラクターを作る
        let character = Character::new(
            character_id,
            character_name,
            character_age,
            character_race,
            character_gender,
            abilities,
            character_story,
        );

        store.add(character)?;
    }
    Ok(store)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_character_to_text() {
        //テスト用の能力を宣言する
        let ability = Ability::new("フレア".to_string(), 50, AbilityType::Attack, 20);

        //テスト用のキャラクターを宣言する
        let character = Character::new(
            CharacterId(1),
            "亜紀".to_string(),
            Some(17),
            Race::Human,
            Gender::Male,
            vec![ability],
            "由紀とともに旅をする".to_string(),
        );

        //キャラクターの情報を文字列に変換する
        let text = character_to_text(&character);

        assert_eq!(
            text,
            "1|亜紀|17|Human|Male|フレア,50,Attack,20|由紀とともに旅をする"
        );
    }

    #[test]
    fn test_character_to_text_multiple_abilities() {
        //テスト用の能力を宣言する
        let ability1 = Ability::new("フレア".to_string(), 50, AbilityType::Attack, 20);
        let ability2 = Ability::new("グリーンヒール".to_string(), 20, AbilityType::Heal, 15);

        //テスト用のキャラクターを宣言する
        let character = Character::new(
            CharacterId(1),
            "亜紀".to_string(),
            Some(17),
            Race::Human,
            Gender::Male,
            vec![ability1, ability2],
            "由紀とともに旅をする".to_string(),
        );

        //キャラクターの情報を文字列に変換する
        let text = character_to_text(&character);

        assert_eq!(
            text,
            "1|亜紀|17|Human|Male|フレア,50,Attack,20;グリーンヒール,20,Heal,15|由紀とともに旅をする"
        );
    }

    #[test]
    fn test_text_to_store() {
        let text = "1|亜紀|17|Human|Male|フレア,50,Attack,20|由紀とともに旅をする";

        let store = text_to_store(text).unwrap();

        let character = &store.get_all()[0];

        assert_eq!(character.name, "亜紀");
        assert_eq!(character.age, Some(17));
        assert_eq!(character.race.to_string(), "Human");
        assert_eq!(character.gender.to_string(), "Male");
        assert_eq!(character.abilities[0].name, "フレア");
        assert_eq!(character.story, "由紀とともに旅をする");
    }

    #[test]
    fn round_trip_test() {
        let ability = Ability::new("フレア".to_string(), 50, AbilityType::Attack, 20);

        let character = Character::new(
            CharacterId(1),
            "亜紀".to_string(),
            Some(17),
            Race::Human,
            Gender::Male,
            vec![ability],
            "由紀とともに旅をする".to_string(),
        );

        let text = character_to_text(&character);
        let store = text_to_store(&text).unwrap();
        let loaded = &store.get_all()[0];

        //全体の比較テスト
        assert_eq!(loaded, &character);

        //各要素それぞれの検証
        assert_eq!(loaded.race.to_string(), character.race.to_string());
        assert_eq!(loaded.gender.to_string(), character.gender.to_string());
        assert_eq!(loaded.story.to_string(), character.story.to_string());
        //能力について能力の各要素の検証
        for index in 0..character.abilities.len() {
            assert_eq!(
                loaded.abilities[index].name,
                character.abilities[index].name
            );
            assert_eq!(
                loaded.abilities[index].power,
                character.abilities[index].power
            );
            assert_eq!(
                loaded.abilities[index].ability_type,
                character.abilities[index].ability_type
            );
            assert_eq!(
                loaded.abilities[index].cost_mp,
                character.abilities[index].cost_mp
            );
        }

        assert_eq!(loaded.name, character.name);
        assert_eq!(loaded.age, character.age);
    }

    #[test]
    fn test_text_to_store_two_characters() {
        let text = concat!(
            "1|亜紀|17|Human|Male|フレア,50,Attack,20;グリーンヒール,20,Heal,15|由紀とともに旅をする\n",
            "2|由紀|16|Human|Female|ブルーオーシャン,35,Buff,20|亜紀の妹"
        );

        let store = text_to_store(text).unwrap();
        let characters = store.get_all();

        assert_eq!(characters.len(), 2);
        assert_eq!(characters[0].name, "亜紀");
        assert_eq!(characters[0].abilities.len(), 2);
        assert_eq!(characters[1].name, "由紀");
        assert_eq!(characters[1].abilities.len(), 1);
    }
}
