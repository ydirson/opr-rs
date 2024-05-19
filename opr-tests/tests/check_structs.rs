use opr::*;
use rstest::rstest;
use std::fs;
use std::path::PathBuf;

#[rstest]
#[case("ybjR2-7kHUNY")]
#[case("dVlqH2ICxln2")]
#[case("nzTpaov-wlwd")]
#[case("nLBrzTpB1TTJ")]
#[case("2HhzjGpcm5m7")]
#[case("p2KIbSBOYpSB")]
#[case("Mlwpoh1AGLC2")]
#[case("Rrlct39EGuct")]
#[case("IIf1w9UTuaFZ")]
#[case("F0SbXPX_MVfK")]
#[case("vpa-qXmUbXmP")]
fn test_load_parse_armies(#[case] army_id: &str) -> Result<(), String> {
    // locate test data from build.rs info
    let mut data_path = PathBuf::from(env!("OPR_DATA_DIR"));
    data_path.push("armies");
    data_path.push(army_id);

    let json_string = fs::read_to_string(&data_path)
        .expect(format!("data file {data_path:?} should to be readable").as_str());
    let _army_list: Army = serde_json::from_str(json_string.as_str())
        .expect(format!("should parse data as json").as_str());
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
    let _army_list: Vec<SpecialRuleDef> = serde_json::from_str(json_string.as_str())
        .expect(format!("should parse data as json").as_str());
    Ok(())
}
