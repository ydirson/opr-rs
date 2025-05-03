use opr::*;
use rstest::rstest;
use std::fs;
use std::path::PathBuf;

#[rstest]
#[case("ybjR2-7kHUNY", 5)]  // GFF: Prime Brothers 200pts
#[case("dVlqH2ICxln2", 6)]  // GFF: Robot Legions 200pts
#[case("nzTpaov-wlwd", 6)]  // GFF: Blessed Sisters 200pts v0
#[case("nLBrzTpB1TTJ", 4)]  // GFF: Alien Hives 200pts - v0
#[case("2HhzjGpcm5m7", 7)]  // AoFS: Haters tribes
#[case("p2KIbSBOYpSB", 24)] // GF: WH Imperium - Imperium
#[case("Mlwpoh1AGLC2", 7)]  // GF: Necrons 2000pts
#[case("Rrlct39EGuct", 18)] // GF: WH Imperium - Necrons
#[case("IIf1w9UTuaFZ", 10)] // GF: Alien Hives 2K
#[case("F0SbXPX_MVfK", 10)] // GF: Blessed Sisters 2K
fn test_load_parse_armies(#[case] army_id: &str, #[case] ngroups: usize) -> Result<(), String> {
    // locate test data from build.rs info
    let mut data_path = PathBuf::from(env!("OPR_DATA_DIR"));
    data_path.push("armies");
    data_path.push(army_id);

    let json_string = fs::read_to_string(&data_path)
        .expect(format!("data file {data_path:?} should to be readable").as_str());
    let army_list: Army = serde_json::from_str(json_string.as_str())
        .expect(format!("should parse data as json").as_str());

    assert_eq!(army_list.unit_groups.len(), ngroups);
    Ok(())
}

#[rstest]
#[case(2)]
#[case(3)]
#[case(4)]
#[case(5)]
#[case(6)]
fn test_load_parse_common_rules(#[case] gs_id: usize) -> Result<(), String> {
    // locate test data from build.rs info
    let mut data_path = PathBuf::from(env!("OPR_DATA_DIR"));
    data_path.push(format!("common-rules-{gs_id}"));

    let json_string = fs::read_to_string(&data_path)
        .expect(format!("data file {data_path:?} should to be readable").as_str());
    let _rules: CommonRules = serde_json::from_str(json_string.as_str())
        .expect(format!("should parse data as json").as_str());
    Ok(())
}
