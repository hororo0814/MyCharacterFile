#[derive(Debug, PartialEq)]
pub struct Ability {
    pub name: String,
    pub power: u32,
    pub ability_type: AbilityType,
    pub cost_mp: u32,
}

#[derive(Debug, PartialEq)]
pub enum AbilityType {
    Attack,
    Heal,
    Buff,
    Debuff,
}

impl std::fmt::Display for AbilityType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Attack => write!(f, "Attack"),
            Self::Heal => write!(f, "Heal"),
            Self::Buff => write!(f, "Buff"),
            Self::Debuff => write!(f, "Debuff"),
        }
    }
}

impl Ability {
    pub fn new(name: String, power: u32, ability_type: AbilityType, cost_mp: u32) -> Self {
        Self {
            name,
            power,
            ability_type,
            cost_mp,
        }
    }
}
