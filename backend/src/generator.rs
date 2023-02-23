use std::os::unix::fs::MetadataExt;
use pathdiff::diff_paths;
use std::path::{PathBuf};
use serde::{de, Deserialize, ser, Serialize};

const DOT_PATH: &str = ".meow_index";

/// Generate thumbnail for a file if absent
pub fn generate_thumb(base: PathBuf, file: PathBuf) -> Result<(), String>
{
    let relative = diff_paths(&file, &base).ok_or("Cannot unwrap path")?;
    let thumb = base.join(DOT_PATH).join(relative)
        .with_extension(format!("thumb.jpg"));

    if thumb.is_file() {
        match (thumb.metadata(), file.metadata()) {
            // Thumbnail already up to date
            (Ok(tm), Ok(fm)) => if tm.mtime() >= fm.mtime() { return Ok(()) },
            _ => {}
#[derive(Serialize, Deserialize)]
pub struct ReturnPath {
    pub(crate) name: String,
    pub(crate) file_type: String,
    pub(crate) mtime: i64
}
        }
    }

    // Generate new thumbnail
    println!("Generating thumbnail for {}\nto {}", file.display(), thumb.display());

    Ok(())
}

fn generate(base: PathBuf, dir: PathBuf)
{
    // Check if already up-to-date
}
