mod domain;
mod repository;
use core::error;

use domain::character::{Character, CharacterId, Gender, Race};

use crate::domain::ability::{Ability, AbilityType};
use crate::repository::character_store::CharacterStore;

fn main() {
    let mut store = CharacterStore::new();

    let ability1 = Ability::new("フレア".to_string(), 50, AbilityType::Attack, 20);
    let ability2 = Ability::new("グリーンヒール".to_string(), 20, AbilityType::Heal, 15);

    let character1 = Character::new(
        CharacterId(1),
        "亜紀".to_string(),
        Some(17),
        Race::Human,
        Gender::Male,
        vec![ability1],
        "由紀とともに旅をする".to_string(),
    );
    let character2 = Character::new(
        CharacterId(1),
        "由紀".to_string(),
        Some(16),
        Race::Human,
        Gender::Female,
        vec![ability2],
        "亜紀の妹".to_string(),
    );

    //store.add(character1);

    //if let Some(character) = store.find_mut(CharacterId(1)) {
    //character.set_age(20);
    //}

    //if let Some(character) = store.remove(CharacterId(2)) {
    //println!("削除: {:?}", character);
    //}
    //for character in &store {
    //println!("{}", character.name);
    //}

    //save_character(&character);
    //let text = character_to_text(&character1);
    //println!("{}", text);
    println!("{}", Race::Human);
    println!("{}", Race::Other("Dragon".to_string()));
    println!("{}", Gender::Male);
}
