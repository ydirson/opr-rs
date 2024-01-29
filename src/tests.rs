use rstest::rstest;
use std::fs;
use std::path::PathBuf;
use super::*;

#[rstest]
#[case("ybjR2-7kHUNY")]
#[case("dVlqH2ICxln2")]
#[case("nzTpaov-wlwd")]
#[case("nLBrzTpB1TTJ")]
#[case("2HhzjGpcm5m7")]
#[case("p2KIbSBOYpSB")]
#[case("Mlwpoh1AGLC2")]
fn test_load_parse(#[case] army_id: &str) -> Result<(), String> {
    // locate test data starting from test exe
    let mut data_path = PathBuf::from(std::env::current_exe()
                                      .expect("test exe should have a path")
                                      .parent()
                                      .expect("test exe path should have a parent"));
    data_path.push("../../../src/test-data");
    data_path.push(army_id);

    let json_string = fs::read_to_string(&data_path)
        .expect(format!("data file {data_path:?} should to be readable").as_str());
    let _army_list: Rc<Army> = serde_json::from_str(json_string.as_str())
        .expect(format!("should parse data as json").as_str());
    Ok(())
}
