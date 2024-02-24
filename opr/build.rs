pub fn main() {
    let out_dir: std::path::PathBuf = std::env::var("OUT_DIR")
        .expect("OUT_DIR should be set").into();
    let data_dir = out_dir.join("data");

    if let Err(e) = opr_test_data::import_data(&data_dir) {
        eprintln!("error: {e:?}");
    }

    println!("cargo:rustc-env=OPR_DATA_DIR={}",
             TryInto::<&str>::try_into(data_dir.as_os_str())
             .expect("OPR_TEST_DATA should be a valid Rust string"));
}
