use core::cmp::Ordering;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use serde_aux::field_attributes::{
    deserialize_number_from_string,
    deserialize_option_number_from_string,
};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::rc::Rc;

pub const ARMYFORGE_SHARE_URL: &str = "https://army-forge.onepagerules.com/share";
pub const AF_API_SRV: &str = "https://army-forge.onepagerules.com";
pub const AF_API_RELAY: &str = "https://generals-familiar-relay-ydirson-aa7362b7.koyeb.app/opr";

// structs for deserialization

#[derive(PartialEq, Debug, Deserialize, Serialize)]
#[serde(from = "JsonArmy")]
pub struct Army {
    pub id: Rc<str>,
    pub name: Rc<str>,
    pub game_system: Result<GameSystem, String>,
    pub special_rules: Vec<Rc<SpecialRuleDef>>,
    pub unit_groups: Vec<Rc<UnitGroup>>,
    pub armybook_ids: Vec<Rc<str>>,
}

#[derive(PartialEq, Debug, Deserialize, Serialize)]
pub struct UnitGroup {
    pub units: Vec<Rc<Unit>>,
    pub full_cost: isize,
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
        // (likely to be improved, not very rusty, not very efficient)
        // - first, group unit ids in sets (not units, as we may not
        // have the join_to_unit unit indexed yet, and index units
        type SelIdGroup = Rc<RefCell<HashSet<Rc<str>>>>;
        let mut groups_of_selid: HashMap<Rc<str>, SelIdGroup> = Default::default();
        let mut units_by_selid: HashMap<Rc<str>, Rc<Unit>> = Default::default();
        for unit in json_army.units.iter() {
            match (
                groups_of_selid.get(&Rc::clone(&unit.selection_id))
                    .map(Rc::clone),
                unit.join_to_unit.as_ref()
                    .map(|x| (Rc::clone(&x), groups_of_selid.get(&Rc::clone(&x)).map(Rc::clone)))
            ) {
                (Some(set), None) =>
                    assert!(set.borrow().contains(&Rc::clone(&unit.selection_id))),
                (None, None) => {
                    let mut set: HashSet<Rc<str>> = Default::default();
                    set.insert(Rc::clone(&unit.selection_id));
                    groups_of_selid.insert(Rc::clone(&unit.selection_id), Rc::new(RefCell::new(set)));
                },
                (Some(set), Some((join_to_unit, None))) => {
                    groups_of_selid.insert(Rc::clone(&join_to_unit), Rc::clone(&set));
                    { set.borrow_mut().insert(Rc::clone(&join_to_unit)); }
                },
                (None, Some((_join_to_unit, Some(set)))) => {
                    groups_of_selid.insert(Rc::clone(&unit.selection_id), Rc::clone(&set));
                    { set.borrow_mut().insert(Rc::clone(&unit.selection_id)); }
                },
                (Some(_set1), Some((_join_to_unit, Some(_set2)))) =>
                    panic!("unhandled merging"), // FIXME should merge, but should not happen
                (None, Some((join_to_unit, None))) => {
                    let mut set: HashSet<Rc<str>> = Default::default();
                    set.insert(Rc::clone(&join_to_unit));
                    set.insert(Rc::clone(&unit.selection_id));
                    let set = Rc::new(RefCell::new(set));
                    groups_of_selid.insert(Rc::clone(&unit.selection_id), Rc::clone(&set));
                    groups_of_selid.insert(Rc::clone(&join_to_unit), set);
                },
            }

            units_by_selid.insert(Rc::clone(&unit.selection_id), Rc::clone(&unit));
        }

        // - then create groups for selid groups without dups
        let mut unit_groups: Vec<Rc<UnitGroup>> = Default::default();
        {
            let mut seen_selid: HashSet<Rc<str>> = Default::default();
            for (selid, group) in groups_of_selid.iter() {
                if seen_selid.contains(selid) {
                    continue;
                }
                let group = group.as_ref().borrow();
                for member in group.iter() {
                    seen_selid.insert(Rc::clone(member));
                }
                let units: Vec<Rc<Unit>> = group.iter()
                    .map(|id| Rc::clone(units_by_selid.get(id).unwrap()))
                    .sorted_by(|a, b| match (a.is_hero, b.is_hero) {
                        (true, false) => Ordering::Less,
                        (false, true) => Ordering::Greater,
                        _ => Ordering::Equal,
                    })
                    .collect();
                unit_groups.push(
                    Rc::new(UnitGroup {
                        full_cost: units.iter().fold(0, |cost, unit| cost + unit.full_cost),
                        units,
                    }));
            }
        }

        // collect unique army_id values
        // FIXME looks pretty alloc-wasteful
        let armybook_ids = HashSet::<String>::from_iter(
            json_army.units.iter().map(|unit| unit.army_id.to_string()));

        Army {
            id: Rc::clone(&json_army.id),
            name: Rc::clone(&json_army.name),
            game_system: GameSystem::try_from(json_army.game_system.as_str()),

            special_rules: json_army.special_rules.clone(),
            unit_groups: unit_groups,
            armybook_ids: armybook_ids.into_iter()
                .map(|id| id.into())
                .collect(),
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
    pub is_hero: bool,
    pub loadout: Vec<Rc<UnitLoadout>>,
    pub selected_upgrades: Vec<Rc<SelectedUpgrade>>,
    //
    pub selection_id: Rc<str>,
    pub combined: bool,
    pub join_to_unit: Option<Rc<str>>,
    pub army_id: Rc<str>,
}

#[derive(Deserialize, Serialize)]
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
    pub rules: Vec<Rc<SpecialRule>>,
    pub loadout: Vec<Rc<UnitLoadout>>,
    pub selected_upgrades: Vec<Rc<SelectedUpgrade>>,
    //
    pub selection_id: Rc<str>,
    pub combined: bool,
    pub join_to_unit: Option<Rc<str>>,
    pub army_id: Rc<str>,
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
            special_rules: json_unit.rules.clone(),
            is_hero: json_unit.rules.iter()
                .find(|rule| rule.name.as_ref() == "Hero")
                .is_some(),
            loadout: json_unit.loadout.clone(),
            selected_upgrades: json_unit.selected_upgrades.clone(),

            selection_id: Rc::clone(&json_unit.selection_id),
            combined: json_unit.combined,
            join_to_unit: json_unit.join_to_unit.clone(),
            army_id: Rc::clone(&json_unit.army_id),
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

#[derive(PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommonRules {
    pub rules: Vec<Rc<SpecialRuleDef>>,
    // traits
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArmyBook {
    pub uid: Rc<str>,
    pub name: Rc<str>,
    pub units: Vec<Rc<ArmyBookUnitDef>>,
    pub spells: Vec<Rc<Spell>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArmyBookUnitDef {
    pub id: Rc<str>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Spell {
    pub name: Rc<str>,
    pub threshold: isize,
    pub effect: Rc<str>,
    pub effect_skirmish: Rc<str>,
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

impl UnitGroup {
    pub fn formatted_name(&self) -> String {
        self.units.iter()
            .map(|unit| unit.formatted_name())
            .intersperse(" + ".into())
            .collect()
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
            let url = format!("{AF_API_SRV}/api/tts?id={army_id}");
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
            format!("{AF_API_RELAY}/api/rules/common/{gs_id}")
        }
    }
}

pub fn get_bookinfo_url(book_id: &str, game_system: GameSystem) -> String {
    let gs_id: usize = game_system.into();
    format!("https://army-forge.onepagerules.com/armyInfo?gameSystem={gs_id}&armyId={book_id}")
}

pub fn get_book_url(book_id: &str, game_system: GameSystem) -> String {
    let gs_id: usize = game_system.into();
    cfg_if::cfg_if! {
        if #[cfg(feature = "local-files")] {
            format!("/data/books/{book_id}-{gs_id}")
        } else {
            format!("{AF_API_RELAY}/api/army-books/{book_id}?gameSystem={gs_id}")
        }
    }
}
