use indicatif::{ProgressBar, ProgressStyle};
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::ErrorKind;
use std::path::Path;

fn file_exists_message(path: &Path) -> String {
    format!("File exists: {}. Use --overwrite to overwrite.", &path.display())
}

// check upfront (in addition to when opening) for better user experience (fail fast)
pub fn check_exists(path: &Path) -> Result<(), Box<dyn Error>> {
    if path.exists() {
        return Err(file_exists_message(path).into());
    }
    Ok(())
}

pub fn create_csv(path: &Path, overwrite: bool) -> Result<csv::Writer<File>, Box<dyn Error>> {
    let file = create_file(path, overwrite)?;
    Ok(csv::Writer::from_writer(file))
}

pub fn create_file(path: &Path, overwrite: bool) -> Result<File, Box<dyn Error>> {
    OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(overwrite)
        // takes precedence over create and truncate if true
        // https://doc.rust-lang.org/std/fs/struct.OpenOptions.html#method.create_new
        .create_new(!overwrite)
        .open(path)
        .map_err(|e| {
            if e.kind() == ErrorKind::AlreadyExists {
                file_exists_message(path).into()
            } else {
                e.into()
            }
        })
}

pub fn progress_bar(len: u64, message: &'static str, template: &str) -> ProgressBar {
    let bar = ProgressBar::new(len);
    bar.set_style(ProgressStyle::default_bar().template(template));
    bar.set_message(message);
    bar
}
