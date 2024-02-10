#[cfg(test)]
mod tests;

use serde::{Deserialize, Serialize};
use serde_aux::field_attributes::{deserialize_number_from_string,
                                  deserialize_string_from_number,
};
use std::rc::Rc;

pub const GET_ARMY_BASE_URL: &str = "https://army-forge.onepagerules.com/api/tts";

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Army {
    pub id: String,
    pub name: String,
    pub game_system: String,
    pub points: usize,
    pub points_limit: usize,
    pub special_rules: Vec<Rc<SpecialRuleDef>>,
    pub units: Vec<Rc<Unit>>,
}

#[derive(PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Unit {
    pub id: String,
    pub name: String,
    pub cost: usize,
    #[serde(default)]
    pub custom_name: String,
    pub size: usize,
    pub quality: usize,
    pub defense: usize,
    pub special_rules: Vec<Rc<SpecialRule>>,
    pub loadout: Vec<Rc<UnitLoadout>>,
    // FIXME army_id for regrouping
}

#[derive(PartialEq, Debug, Deserialize, Serialize)]
pub struct SpecialRule {
    pub name: String,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_string_from_number")]
    pub rating: String,
}

#[derive(PartialEq, Debug, Deserialize, Serialize)]
pub struct SpecialRuleDef {
    pub name: String,
    pub description: String,
}

#[derive(PartialEq, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum UnitLoadout {
    Equipment(Equipment),
    Upgrade(UnitUpgrade),
}

#[derive(PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Equipment {
    pub name: String,
    #[serde(default)]
    pub range: usize,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub attacks: usize,
    pub count: usize,
    pub special_rules: Vec<Rc<SpecialRule>>,
}

#[derive(PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UnitUpgrade{
    pub name: String,
    pub content: Vec<Rc<SpecialRule>>,
}

impl Unit {
    pub fn formatted_name(&self) -> String {
        let Unit{ref name, ref custom_name, size, ..} = *self;
        let name = if custom_name.len() > 0 {
            format!("{custom_name} ({name})")
        } else {
            name.to_string()
        };
        if size > 1 {
            format!("{name} [{size}]")
        } else {
            name
        }
    }
}
