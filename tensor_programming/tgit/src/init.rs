use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::error::TgitError;

pub fn init() -> Result<(), TgitError> {
    let dir = Path::new(".tgit");
    fs::create_dir(dir)?;
    fs::create_dir(dir.join("objects"))?;
    fs::create_dir(dir.join("refs"))?;
    fs::create_dir(dir.join("refs").join("heads"))?;

    let mut head = File::create(dir.join("HEAD"))?;
    head.write_all(b"refs: refs/heads/master")?;
    Ok(())
}
