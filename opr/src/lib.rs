use serde::{Deserialize, Serialize};
use serde_aux::field_attributes::{deserialize_number_from_string, deserialize_string_from_number};
use std::fmt;
use std::rc::Rc;
use std::str::FromStr;

pub const ARMYFORGE_SHARE_URL: &str = "https://army-forge.onepagerules.com/share";

cfg_if::cfg_if! {
    if #[cfg(not(feature = "local-files"))] {
        const GET_ARMY_URL: &str =
            "https://army-forge.onepagerules.com/api/tts";
        const GET_COMMON_RULES_URL: &str =
            "https://army-forge.onepagerules.com/api/afs/common-rules";
    }
}

// structs for deserialization

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
    //
    pub selection_id: String,
    pub combined: bool,
    pub join_to_unit: Option<String>,
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
pub struct UnitUpgrade {
    pub name: String,
    pub content: Vec<Rc<SpecialRule>>,
}


// higher-level than deserialization

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

#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub enum GameSystem {
    GF,
    GFF,
    AoF,
    AoFS,
    AoFR,
}

impl FromStr for GameSystem {
    type Err = String;

    fn from_str(input: &str) -> Result<GameSystem, Self::Err> {
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

pub fn get_army_url(army_id: &str) -> String {
    cfg_if::cfg_if! {
        if #[cfg(feature = "local-files")] {
            let url = format!("/data/armies/{army_id}");
        } else {
            let url = format!("{}?id={army_id}", GET_ARMY_URL);
        }
    }
    url
}

pub fn get_common_rules_url(game_system: GameSystem) -> String {
    let query_description = match game_system {
        GameSystem::GF | GameSystem::AoF => None,
        GameSystem::GFF | GameSystem::AoFS => Some("skirmish"),
        GameSystem::AoFR => Some("regiments"),
    };
    cfg_if::cfg_if! {
        if #[cfg(feature = "local-files")] {
            let url = "/data/common-rules";
            match query_description {
                None => url.to_string(),
                Some(query_description) =>
                    format!("{url}-{query_description}"),
            }
        } else {
            match query_description {
                None => GET_COMMON_RULES_URL.to_string(),
                Some(query_description) =>
                    format!("{GET_COMMON_RULES_URL}?description={query_description}"),
            }
        }
    }
}
