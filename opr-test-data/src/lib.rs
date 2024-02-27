use include_dir::{include_dir, Dir};
use std::io;
use std::path::Path;

static DATA_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/src/data");

/// Install test data into `target_dir`.  Since the actual location
/// depends on the location of the data files to serve it is the
/// responsibility of the app to get those files installed in the
/// proper location.
///
/// For `trunk serve` for example, data files have to end up being
/// installed as dist/data/, but need to go through an intermediate
/// location from which Trunk will take them.
///
/// target_dir will be deleted and recreated
pub fn import_data(target_dir: &Path) -> io::Result<()> {
    if target_dir.exists() {
        std::fs::remove_dir_all(&target_dir)?;
    }
    std::fs::create_dir_all(&target_dir)?;

    DATA_DIR.extract(&target_dir)
}
