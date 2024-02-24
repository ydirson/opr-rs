use std::io;
use std::path::Path;
use include_dir::{include_dir, Dir};

static DATA_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/src/data");

/// target_dir will be deleted and recreated
pub fn import_data(target_dir: &Path) -> io::Result<()> {
    if target_dir.exists() {
        std::fs::remove_dir_all(&target_dir)?;
    }
    std::fs::create_dir_all(&target_dir)?;

    DATA_DIR.extract(&target_dir)
}
