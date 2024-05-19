use serde::{Deserialize, Serialize};
use serde_aux::field_attributes::{
    deserialize_number_from_string,
    deserialize_option_number_from_string,
};
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

pub const ARMYFORGE_SHARE_URL: &str = "https://army-forge.onepagerules.com/share";

// structs for deserialization

#[derive(PartialEq, Debug, Deserialize, Serialize)]
#[serde(from = "JsonArmy")]
pub struct Army {
    pub id: Rc<str>,
    pub name: Rc<str>,
    pub game_system: Result<GameSystem, String>,
    pub special_rules: Vec<Rc<SpecialRuleDef>>,
    pub units: Vec<Rc<Unit>>,
}

#[derive(PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct JsonArmy {
    pub id: Rc<str>,
    pub name: Rc<str>,
    pub game_system: String,
    pub special_rules: Vec<Rc<SpecialRuleDef>>,
    pub units: Vec<Rc<Unit>>,
}

impl From<JsonArmy> for Army {
    fn from(json_army: JsonArmy) -> Army {
        Army {
            id: Rc::clone(&json_army.id),
            name: Rc::clone(&json_army.name),
            game_system: GameSystem::try_from(json_army.game_system.as_str()),

            special_rules: json_army.special_rules.clone(),
            units: json_army.units.clone(),
        }
    }
}

#[derive(PartialEq, Debug, Deserialize, Serialize)]
#[serde(from = "JsonUnit")]
pub struct Unit {
    pub id: Rc<str>,
    pub name: Rc<str>,
    pub cost: isize,
    pub full_cost: isize,
    pub custom_name: Option<Rc<str>>,
    pub size: usize,
    pub quality: usize,
    pub defense: usize,
    pub special_rules: Vec<Rc<SpecialRule>>,
    pub loadout: Vec<Rc<UnitLoadout>>,
    pub selected_upgrades: Vec<Rc<SelectedUpgrade>>,
    //
    pub selection_id: Rc<str>,
    pub combined: bool,
    pub join_to_unit: Option<Rc<str>>,
    // FIXME army_id for regrouping
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct JsonUnit {
    pub id: Rc<str>,
    pub name: Rc<str>,
    pub cost: isize,
    #[serde(default)]
    pub custom_name: Option<Rc<str>>,
    pub size: usize,
    pub quality: usize,
    pub defense: usize,
    pub special_rules: Vec<Rc<SpecialRule>>,
    pub loadout: Vec<Rc<UnitLoadout>>,
    pub selected_upgrades: Vec<Rc<SelectedUpgrade>>,
    //
    pub selection_id: Rc<str>,
    pub combined: bool,
    pub join_to_unit: Option<Rc<str>>,
}

impl From<JsonUnit> for Unit {
    fn from(json_unit: JsonUnit) -> Unit {
        let full_cost = json_unit.selected_upgrades.iter()
            .fold(json_unit.cost,
                  |acc, upg| {
                      if upg.option.costs.contains_key(json_unit.id.as_ref()) {
                          acc + upg.option.costs[json_unit.id.as_ref()]
                      } else {
                          // not for this unit instance
                          acc
                      }
                  });
        Unit {
            id: Rc::clone(&json_unit.id),
            name: Rc::clone(&json_unit.name),
            cost: json_unit.cost,
            full_cost,
            custom_name: json_unit.custom_name.clone(),
            size: json_unit.size,
            quality: json_unit.quality,
            defense: json_unit.defense,
            special_rules: json_unit.special_rules.clone(),
            loadout: json_unit.loadout.clone(),
            selected_upgrades: json_unit.selected_upgrades.clone(),

            selection_id: Rc::clone(&json_unit.selection_id),
            combined: json_unit.combined,
            join_to_unit: json_unit.join_to_unit.clone(),
        }
    }
}

#[derive(PartialEq, Debug, Deserialize, Serialize)]
pub struct SpecialRule {
    pub name: Rc<str>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    pub rating: Option<usize>,
}

#[derive(PartialEq, Debug, Deserialize, Serialize)]
pub struct SpecialRuleDef {
    pub name: Rc<str>,
    pub description: Rc<str>,
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
    pub name: Rc<str>,
    #[serde(default)]
    pub range: usize,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub attacks: usize,
    pub count: usize,
    pub special_rules: Vec<Rc<SpecialRule>>,
}

#[derive(PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UnitUpgrade {
    pub name: Rc<str>,
    pub content: Vec<Rc<SpecialRule>>,
}

#[derive(PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SelectedUpgrade {
    pub option: UnitUpgradeOption,
}
#[derive(PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UnitUpgradeOption {
    // pub label: String,
    #[serde(with = "unit_upgrade_option")]
    pub costs: HashMap<String, isize>,
}

mod unit_upgrade_option {
    use std::collections::HashMap;
    use serde::ser::Serializer;
    use serde::de::{Deserialize, Deserializer};

    #[derive(serde::Deserialize, serde::Serialize)]
    #[serde(rename_all = "camelCase")]
    struct Cost {
        cost: isize,
        unit_id: String,
    }

    pub fn serialize<S>(map: &HashMap<String, isize>, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer
    {
        serializer.collect_seq(map.iter().map(|(unit_id, cost)|
                                              Cost{unit_id: unit_id.to_owned(),
                                                   cost: cost.to_owned()}))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<HashMap<String, isize>, D::Error>
    where D: Deserializer<'de>
    {
        let mut map = HashMap::new();
        for item in Vec::<Cost>::deserialize(deserializer)? {
            map.insert(item.unit_id, item.cost);
        }
        Ok(map)
    }
}

// higher-level than deserialization

impl Unit {
    pub fn formatted_name(&self) -> String {
        let Unit{ref name, ref custom_name, size, ..} = *self;
        let name = match custom_name {
            Some(custom_name) if custom_name.len() > 0
                => format!("{custom_name} ({name})"),
            _ => name.to_string()
        };
        if size > 1 {
            format!("{name} [{size}]")
        } else {
            name
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
#[non_exhaustive]
pub enum GameSystem {
    GF,
    GFF,
    AoF,
    AoFS,
    AoFR,
}

impl TryFrom<&str> for GameSystem {
    type Error = String;

    fn try_from(input: &str) -> Result<GameSystem, Self::Error> {
        match input {
            "GF"   | "gf"   => Ok(GameSystem::GF),
            "GFF"  | "gff"  => Ok(GameSystem::GFF),
            "AoF"  | "aof"  => Ok(GameSystem::AoF),
            "AoFS" | "aofs" => Ok(GameSystem::AoFS),
            "AoFR" | "aofr" => Ok(GameSystem::AoFR),
            _ => Err(format!(r#"cannot find GameSystem for "{input}""#)),
        }
    }
}

impl fmt::Display for GameSystem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            GameSystem::GF   => "GF",
            GameSystem::GFF  => "GFF",
            GameSystem::AoF  => "AoF",
            GameSystem::AoFS => "AoFS",
            GameSystem::AoFR => "AoFR",
        })
    }
}

impl Into<usize> for GameSystem {
    fn into(self) -> usize {
        match self {
            GameSystem::GF   => 2,
            GameSystem::GFF  => 3,
            GameSystem::AoF  => 4,
            GameSystem::AoFS => 5,
            GameSystem::AoFR => 6,
        }
    }
}

pub fn get_army_url(army_id: &str) -> String {
    cfg_if::cfg_if! {
        if #[cfg(feature = "local-files")] {
            let url = format!("/data/armies/{army_id}");
        } else {
            let url = format!("https://army-forge.onepagerules.com/api/tts?id={army_id}");
        }
    }
    url
}

pub fn get_common_rules_url(game_system: GameSystem) -> String {
    let gs_id:usize = game_system.into();
    cfg_if::cfg_if! {
        if #[cfg(feature = "local-files")] {
            format!("/data/common-rules-{gs_id}")
        } else {
            format!("https://army-forge.onepagerules.com/api/rules/common/{gs_id}")
        }
    }
}
