use crate::domain::ability::Ability;
use std::fmt;
#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum Race {
    Human,
    Elf,
    Beast,
    Demon,
    Other(String),
}

impl std::fmt::Display for Race {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Human => write!(f, "Human"),
            Self::Elf => write!(f, "Elf"),
            Self::Demon => write!(f, "Demon"),
            Self::Beast => write!(f, "Beast"),
            Self::Other(name) => write!(f, "{}", name),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Gender {
    Male,
    Female,
    Other,
}

impl std::fmt::Display for Gender {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Male => write!(f, "Male"),
            Self::Female => write!(f, "Female"),
            Self::Other => write!(f, "Other"),
        }
    }
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
