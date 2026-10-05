use crate::domain::ability::Ability;
use std::fmt;
#[derive(Debug)]

pub struct Character {
    pub id: CharacterId,
    pub name: String,
    pub age: Option<u32>,
    pub race: Race,
    pub gender: Gender,
    pub abilities: Vec<Ability>,
    pub story: String,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct CharacterId(pub u64);

impl fmt::Display for CharacterId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
pub enum Race {
    Human,
    Elf,
    Beast,
    Demon,
    Other(String),
}

#[derive(Debug)]
pub enum Gender {
    Male,
    Female,
    Other,
}

impl Character {
    ///新しいキャラクターを定義
    pub fn new(
        id: CharacterId,
        name: String,
        age: Option<u32>,
        race: Race,
        gender: Gender,
        abilities: Vec<Ability>,
        story: String,
    ) -> Self {
        Self {
            id,
            name,
            age,
            race,
            gender,
            abilities,
            story,
        }
    }

    pub fn set_age(&mut self, age: u32) {
        self.age = Some(age)
    }

    pub fn display_name(&self) -> &str {
        &self.name
    }
}
