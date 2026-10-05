#[derive(Debug)]
pub struct Ability {
    pub name: String,
    pub power: u32,
    pub ability_type: AbilityType,
    pub cost_mp: u32,
}

#[derive(Debug)]
pub enum AbilityType {
    Attack,
    Heal,
    Buff,
    Debuff,
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
