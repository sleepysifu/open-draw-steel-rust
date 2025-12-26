use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct Ability {
    pub name: String,
    #[serde(rename = "power_roll_1")]
    power_roll_1: PowerRoll,
    #[serde(rename = "power_roll_2")]
    power_roll_2: PowerRoll,
    #[serde(rename = "power_roll_3")]
    power_roll_3: PowerRoll,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct PowerRoll {
    pub damage: i32,
}

impl Ability {
    /// Get the power rolls as an array of exactly 3 elements
    pub fn power_rolls(&self) -> [&PowerRoll; 3] {
        [
            &self.power_roll_1,
            &self.power_roll_2,
            &self.power_roll_3,
        ]
    }
}