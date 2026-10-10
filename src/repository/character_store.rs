use crate::domain::character::{Character, CharacterId, Gender, Race};
use thiserror::Error;

pub struct CharacterStore {
    characters: Vec<Character>,
}

impl CharacterStore {
    pub fn new() -> Self {
        Self {
            characters: Vec::new(),
        }
    }

    pub fn add(&mut self, character: Character) -> Result<(), CharacterStoreErr> {
        if self.find(character.id).is_some() {
            return Err(CharacterStoreErr::DuplicateId(character.id));
        }
        self.characters.push(character);
        Ok(())
    }

    pub fn get_all(&self) -> &[Character] {
        &self.characters
    }

    ///Idからキャラクターを探す
    pub fn find(&self, id: CharacterId) -> Option<&Character> {
        self.characters.iter().find(|character| character.id == id)
    }

    ///Idから編集対象としてキャラクターを探す
    pub fn find_mut(&mut self, id: CharacterId) -> Option<&mut Character> {
        self.characters
            .iter_mut()
            .find(|character| character.id == id)
    }

    pub fn remove(&mut self, id: CharacterId) -> Option<Character> {
        for (index, character) in self.characters.iter().enumerate() {
            if character.id == id {
                return Some(self.characters.remove(index));
            }
        }
        None
    }

    pub fn search_by_race(&self, race: Option<Race>) -> Vec<&Character> {
        self.characters
            .iter()
            .filter(|character| match &race {
                None => true,
                Some(r) => r == &character.race,
            })
            .collect()
    }

    pub fn search_by_gender(&self, gender: Option<Gender>) -> Vec<&Character> {
        self.characters
            .iter()
            .filter(|character| match &gender {
                None => true,
                Some(r) => r == &character.gender,
            })
            .collect()
    }

    pub fn search_by_race_and_gender(
        &self,
        race: Option<Race>,
        gender: Option<Gender>,
    ) -> Vec<&Character> {
        self.characters
            .iter()
            .filter(|character| match &race {
                None => true,
                Some(r) => r == &character.race,
            })
            .filter(|character| match &gender {
                None => true,
                Some(r) => r == &character.gender,
            })
            .collect()
    }
}

impl<'a> IntoIterator for &'a CharacterStore {
    type Item = &'a Character;
    type IntoIter = std::slice::Iter<'a, Character>;

    fn into_iter(self) -> Self::IntoIter {
        self.characters.iter()
    }
}

#[derive(Debug, Error)]
pub enum CharacterStoreErr {
    #[error("Character ID {0}はすでに存在します")]
    DuplicateId(CharacterId),

    //ファイル関連のエラー
    #[error("ファイル操作に失敗しました")]
    FileError(#[from] std::io::Error),

    //storyの文字数上限
    #[error("Storyの文字数は50文字未満に収めてください")]
    WordLenError(),

    //ファイルの形式がおかしい
    #[error("保存ファイルの形式が不正です: {0}")]
    InvalidFormat(String),
}

#[cfg(test)]
mod test {

    use super::CharacterStore;
    use crate::Ability;
    use crate::AbilityType;
    use crate::Character;
    use crate::CharacterId;
    use crate::Gender;
    use crate::Race;
    #[test]
    fn race_match_test() {
        //空のキャラクターを用意
        let mut store = CharacterStore::new();

        //テスト用の能力を宣言
        let ability1 = Ability::new("フレア".to_string(), 50, AbilityType::Attack, 20);
        let ability3 = Ability::new("ブルーオーシャン".to_string(), 35, AbilityType::Buff, 20);

        //テスト用(Race:Human)キャラクターの宣言
        let character1 = Character::new(
            CharacterId(1),
            "亜紀".to_string(),
            Some(17),
            Race::Human,
            Gender::Male,
            vec![ability1],
            "由紀とともに旅をする".to_string(),
        );

        //テスト用(Race:)キャラクターの宣言
        let character3 = Character::new(
            CharacterId(3),
            "アーリヤ".to_string(),
            Some(24),
            Race::Elf,
            Gender::Female,
            vec![ability3],
            "天使とエルフのハーフ".to_string(),
        );

        store.add(character1).unwrap();
        store.add(character3).unwrap();

        //Humanの検索
        let test_race = store.search_by_race(Some(Race::Human));
        assert_eq!(test_race.len(), 1);

        let search_chara_id = match test_race.first() {
            Some(character) => character.id.0,
            None => panic!(),
        };
        assert_eq!(search_chara_id, 1);
    }

    #[test]
    fn race_not_match_test() {
        //テスト用の空のStoreを用意する
        let mut store = CharacterStore::new();

        //テスト用の能力を宣言
        let ability1 = Ability::new("フレア".to_string(), 50, AbilityType::Attack, 20);

        //テスト用(Race:Human)のキャラクターを宣言
        let character1 = Character::new(
            CharacterId(1),
            "亜紀".to_string(),
            Some(17),
            Race::Human,
            Gender::Male,
            vec![ability1],
            "由紀とともに旅をする".to_string(),
        );

        store.add(character1).unwrap();
        //Demonの検索
        let test_race = store.search_by_race(Some(Race::Demon));
        assert!(test_race.is_empty());
    }

    #[test]
    fn all_race_match_test() {
        //空のキャラクターを用意
        let mut store = CharacterStore::new();

        //テスト用の能力を宣言
        let ability1 = Ability::new("フレア".to_string(), 50, AbilityType::Attack, 20);
        let ability3 = Ability::new("ブルーオーシャン".to_string(), 35, AbilityType::Buff, 20);

        //テスト用(Race:Human)キャラクターの宣言
        let character1 = Character::new(
            CharacterId(1),
            "亜紀".to_string(),
            Some(17),
            Race::Human,
            Gender::Male,
            vec![ability1],
            "由紀とともに旅をする".to_string(),
        );

        //テスト用(Race:)キャラクターの宣言
        let character3 = Character::new(
            CharacterId(3),
            "アーリヤ".to_string(),
            Some(24),
            Race::Elf,
            Gender::Female,
            vec![ability3],
            "天使とエルフのハーフ".to_string(),
        );

        store.add(character1).unwrap();
        store.add(character3).unwrap();

        //Humanの検索
        let test_race = store.search_by_race(None);
        assert_eq!(test_race.len(), 2);
    }

    ///Gender用のテスト
    #[test]
    fn gender_match_test() {
        //空のキャラクターを用意
        let mut store = CharacterStore::new();

        //テスト用の能力を宣言
        let ability1 = Ability::new("フレア".to_string(), 50, AbilityType::Attack, 20);
        let ability3 = Ability::new("ブルーオーシャン".to_string(), 35, AbilityType::Buff, 20);

        //テスト用(Race:Human)キャラクターの宣言
        let character1 = Character::new(
            CharacterId(1),
            "亜紀".to_string(),
            Some(17),
            Race::Human,
            Gender::Male,
            vec![ability1],
            "由紀とともに旅をする".to_string(),
        );

        //テスト用(Race:)キャラクターの宣言
        let character3 = Character::new(
            CharacterId(3),
            "アーリヤ".to_string(),
            Some(24),
            Race::Elf,
            Gender::Female,
            vec![ability3],
            "天使とエルフのハーフ".to_string(),
        );

        store.add(character1).unwrap();
        store.add(character3).unwrap();

        //Humanの検索
        let test_gender = store.search_by_gender(Some(Gender::Male));
        assert_eq!(test_gender.len(), 1);

        let search_chara_id = match test_gender.first() {
            Some(character) => character.id.0,
            None => panic!(),
        };
        assert_eq!(search_chara_id, 1);
    }

    #[test]
    fn gender_not_match_test() {
        //テスト用の空のStoreを用意する
        let mut store = CharacterStore::new();

        //テスト用の能力を宣言
        let ability1 = Ability::new("フレア".to_string(), 50, AbilityType::Attack, 20);

        //テスト用(Race:Human)のキャラクターを宣言
        let character1 = Character::new(
            CharacterId(1),
            "亜紀".to_string(),
            Some(17),
            Race::Human,
            Gender::Male,
            vec![ability1],
            "由紀とともに旅をする".to_string(),
        );

        store.add(character1).unwrap();
        //Demonの検索
        let test_gender = store.search_by_gender(Some(Gender::Other));
        assert!(test_gender.is_empty());
    }

    #[test]
    fn all_gender_match_test() {
        //空のキャラクターを用意
        let mut store = CharacterStore::new();

        //テスト用の能力を宣言
        let ability1 = Ability::new("フレア".to_string(), 50, AbilityType::Attack, 20);
        let ability3 = Ability::new("ブルーオーシャン".to_string(), 35, AbilityType::Buff, 20);

        //テスト用(Race:Human)キャラクターの宣言
        let character1 = Character::new(
            CharacterId(1),
            "亜紀".to_string(),
            Some(17),
            Race::Human,
            Gender::Male,
            vec![ability1],
            "由紀とともに旅をする".to_string(),
        );

        //テスト用(Race:)キャラクターの宣言
        let character3 = Character::new(
            CharacterId(3),
            "アーリヤ".to_string(),
            Some(24),
            Race::Elf,
            Gender::Female,
            vec![ability3],
            "天使とエルフのハーフ".to_string(),
        );

        store.add(character1).unwrap();
        store.add(character3).unwrap();

        //Humanの検索
        let test_gender = store.search_by_gender(None);
        assert_eq!(test_gender.len(), 2);
    }

    ///RaceとGender両方を検索する用のテスト
    #[test]
    fn race_gender_match_test() {
        //空のキャラクターを用意
        let mut store = CharacterStore::new();

        //テスト用の能力を宣言
        let ability1 = Ability::new("フレア".to_string(), 50, AbilityType::Attack, 20);
        let ability3 = Ability::new("ブルーオーシャン".to_string(), 35, AbilityType::Buff, 20);

        //テスト用(Race:Human)キャラクターの宣言
        let character1 = Character::new(
            CharacterId(1),
            "亜紀".to_string(),
            Some(17),
            Race::Human,
            Gender::Male,
            vec![ability1],
            "由紀とともに旅をする".to_string(),
        );

        //テスト用(Race:)キャラクターの宣言
        let character3 = Character::new(
            CharacterId(3),
            "アーリヤ".to_string(),
            Some(24),
            Race::Elf,
            Gender::Female,
            vec![ability3],
            "天使とエルフのハーフ".to_string(),
        );

        store.add(character1).unwrap();
        store.add(character3).unwrap();

        //Humanの検索
        let test_race_gender =
            store.search_by_race_and_gender(Some(Race::Human), Some(Gender::Male));
        assert_eq!(test_race_gender.len(), 1);

        let search_chara_id = match test_race_gender.first() {
            Some(character) => character.id.0,
            None => panic!(),
        };
        assert_eq!(search_chara_id, 1);
    }

    #[test]
    fn race_gender_not_match_test() {
        //テスト用の空のStoreを用意する
        let mut store = CharacterStore::new();

        //テスト用の能力を宣言
        let ability1 = Ability::new("フレア".to_string(), 50, AbilityType::Attack, 20);

        //テスト用(Race:Human)のキャラクターを宣言
        let character1 = Character::new(
            CharacterId(1),
            "亜紀".to_string(),
            Some(17),
            Race::Human,
            Gender::Male,
            vec![ability1],
            "由紀とともに旅をする".to_string(),
        );

        store.add(character1).unwrap();
        //Demonの検索
        let test_race_gender =
            store.search_by_race_and_gender(Some(Race::Human), Some(Gender::Female));
        assert!(test_race_gender.is_empty());
    }

    #[test]
    fn all_race_gender_match_test() {
        //空のキャラクターを用意
        let mut store = CharacterStore::new();

        //テスト用の能力を宣言
        let ability1 = Ability::new("フレア".to_string(), 50, AbilityType::Attack, 20);
        let ability3 = Ability::new("ブルーオーシャン".to_string(), 35, AbilityType::Buff, 20);

        //テスト用(Race:Human)キャラクターの宣言
        let character1 = Character::new(
            CharacterId(1),
            "亜紀".to_string(),
            Some(17),
            Race::Human,
            Gender::Male,
            vec![ability1],
            "由紀とともに旅をする".to_string(),
        );

        //テスト用(Race:)キャラクターの宣言
        let character3 = Character::new(
            CharacterId(3),
            "アーリヤ".to_string(),
            Some(24),
            Race::Elf,
            Gender::Female,
            vec![ability3],
            "天使とエルフのハーフ".to_string(),
        );

        store.add(character1).unwrap();
        store.add(character3).unwrap();

        //Humanの検索
        let test_race_gender = store.search_by_race_and_gender(None, None);
        assert_eq!(test_race_gender.len(), 2);
    }

    #[test]
    fn find_mut_name_test() {
        let mut store = CharacterStore::new();

        //テスト用の能力を宣言
        let ability1 = Ability::new("フレア".to_string(), 50, AbilityType::Attack, 20);

        //テスト用(Race:Human)キャラクターの宣言
        let character1 = Character::new(
            CharacterId(1),
            "亜紀".to_string(),
            Some(17),
            Race::Human,
            Gender::Male,
            vec![ability1],
            "由紀とともに旅をする".to_string(),
        );

        store.add(character1);
        store.find_mut(CharacterId(1)).unwrap().name = "亜紀太郎".to_string();
        assert_eq!(
            store.find(CharacterId(1)).unwrap().name,
            "亜紀太郎".to_string()
        )
    }

    #[test]
    fn find_mut_age_test() {
        let mut store = CharacterStore::new();

        //テスト用の能力を宣言
        let ability1 = Ability::new("フレア".to_string(), 50, AbilityType::Attack, 20);

        //テスト用(Race:Human)キャラクターの宣言
        let character1 = Character::new(
            CharacterId(1),
            "亜紀".to_string(),
            Some(17),
            Race::Human,
            Gender::Male,
            vec![ability1],
            "由紀とともに旅をする".to_string(),
        );

        store.add(character1);
        store.find_mut(CharacterId(1)).unwrap().age = Some(20);
        assert_eq!(store.find(CharacterId(1)).unwrap().age, Some(20));
    }

    #[test]
    ///年齢を None に変更するテスト
    fn find_mut_no_age_test() {
        let mut store = CharacterStore::new();

        //テスト用の能力を宣言
        let ability1 = Ability::new("フレア".to_string(), 50, AbilityType::Attack, 20);

        //テスト用(Race:Human)キャラクターの宣言
        let character1 = Character::new(
            CharacterId(1),
            "亜紀".to_string(),
            Some(17),
            Race::Human,
            Gender::Male,
            vec![ability1],
            "由紀とともに旅をする".to_string(),
        );

        store.add(character1);
        store.find_mut(CharacterId(1)).unwrap().age = None;
        assert_eq!(store.find(CharacterId(1)).unwrap().age, None);
    }
}
