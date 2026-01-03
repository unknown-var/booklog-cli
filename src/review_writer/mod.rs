use crate::config;
use crate::file_loading;
use std::error::Error;
use std::fs;
use std::process::Command;
use which::which;
fn check_editor(editor: &str) -> bool {
    match which("vim") {
        Ok(path) => return true,
        Err(_) => return false,
    }
}

pub fn create_review(isbn: &str) -> Result<(), Box<dyn Error>> {
    let path = file_loading::get_review_path(isbn)?;

    fs::File::create(&path)?;
    println!("path: {:?}", &path);

    let command: String = config::get_config().text_editor.clone();

    let status = Command::new(command).arg(path).status()?;

    if status.success() {
        println!("writen review");
    } else {
        eprintln!("failed to write review");
    }
    Ok(())
}
