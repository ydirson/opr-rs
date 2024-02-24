use rstest::rstest;
use std::fs;
use std::path::{Path, PathBuf};
use opr::*;

#[rstest]
#[case("ybjR2-7kHUNY")]
#[case("dVlqH2ICxln2")]
#[case("nzTpaov-wlwd")]
#[case("nLBrzTpB1TTJ")]
#[case("2HhzjGpcm5m7")]
#[case("p2KIbSBOYpSB")]
#[case("Mlwpoh1AGLC2")]
#[case("Rrlct39EGuct")]
fn test_load_parse_armies(#[case] army_id: &str) -> Result<(), String> {
    // locate test data starting from source file
    let topdir = Path::new("..");
    let mut data_path = PathBuf::from(fs::canonicalize(topdir.join(file!()))
                                      .expect("test source file should have a valid path")
                                      .parent()
                                      .expect("test source file should have a parent"));
    eprintln!("data_path: {data_path:?}");
    data_path.push("data/armies");
    data_path.push(army_id);

    let json_string = fs::read_to_string(&data_path)
        .expect(format!("data file {data_path:?} should to be readable").as_str());
    let _army_list: Army = serde_json::from_str(json_string.as_str())
        .expect(format!("should parse data as json").as_str());
    Ok(())
}

#[test]
fn test_load_parse_common_rules() -> Result<(), String> {
    // locate test data starting from source file
    let topdir = Path::new("..");
    let mut data_path = PathBuf::from(fs::canonicalize(topdir.join(file!()))
                                      .expect("test source file should have a valid path")
                                      .parent()
                                      .expect("test source file should have a parent"));
    data_path.push("data/common-rules");

    let json_string = fs::read_to_string(&data_path)
        .expect(format!("data file {data_path:?} should to be readable").as_str());
    let _army_list: Vec<SpecialRuleDef> = serde_json::from_str(json_string.as_str())
        .expect(format!("should parse data as json").as_str());
    Ok(())
}