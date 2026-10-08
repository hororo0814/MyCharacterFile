use std::io::{Chain, Write};
use std::{char, fs};

use crate::domain::character::{self, Character};
use crate::repository::character_store::{CharacterStore, CharacterStoreErr};

//保存用文字列をファイルに保存する．
pub fn save_text(character: &str) {
    let mut file = fs::File::create("file.txt").unwrap();
    file.write_all(character.as_bytes()).unwrap();
}

///キャラクターの情報を保存用の文字列に変換する．
pub fn character_to_text(character: &Character) -> String {
    let age = match character.age {
        Some(age) => age.to_string(),
        None => String::new(),
    };
    format!(
        "{}|{}|{}|{}|{}",
        character.id, character.name, age, character.race, character.gender
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
pub fn load_text() -> Result<String, CharacterStoreErr> {
    let load_text = fs::read_to_string("file.txt")?;
    for line in load_text.lines() {
        let fields = line.split('|');

        for field in fields {
            println!("{}", field);
        }
    }
    Ok(load_text)
}
