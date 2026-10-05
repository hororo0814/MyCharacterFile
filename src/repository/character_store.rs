use crate::domain::character::{Character, CharacterId};
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
        Ok(self.characters.push(character))
    }

    pub fn get_all(&self) -> &[Character] {
        &self.characters
    }

    ///Idからキャラクターを探す
    pub fn find(&self, id: CharacterId) -> Option<&Character> {
        self.characters.iter().find(|character| character.id == id)
    }

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
}
