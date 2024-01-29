mod static_data;

use super::*;

#[test]
fn test_load_parse() {
    for (i, json_string) in static_data::JSON_DATA.iter().enumerate() {
        let _army_list: Rc<Army> = serde_json::from_str(json_string)
            .expect(format!("should parse data {i} as json").as_str());
    }
}
