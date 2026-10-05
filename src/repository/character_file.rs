use std::fs;
use std::io::{Chain, Write};

use crate::domain::character::Character;

pub fn save_text(character: &str) {
    let mut file = fs::File::create("characters.txt").unwrap();
    file.write_all(character.as_bytes()).unwrap();
}

///キャラクターの情報を保存用の文字列に変換する
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
