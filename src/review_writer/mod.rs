use crate::config;
use crate::file_loading;
use std::error::Error;
use std::fs;
use std::process::Command;
use which::which;

fn check_editor(editor: &str) -> bool {
    match which(editor) {
        Ok(_) => return true,
        Err(_) => return false,
    }
}

pub fn create_review(isbn: &str) -> Result<(), Box<dyn Error>> {
    let path = file_loading::get_review_path(isbn)?;

    fs::File::create(&path)?;
    println!("path: {:?}", &path);

    let command: String = config::get_config().text_editor.clone();

    if !check_editor(&command) {
        return Err(format!("the editor {} does not exist on the system", command).into());
    }

    let status = Command::new(command).arg(path).status()?;

    if status.success() {
        println!("writen review");
    } else {
        eprintln!("failed to write review");
    }
    Ok(())
}

pub fn delete_review(isbn: &str) -> Result<(), Box<dyn Error>> {
    let review_path = file_loading::get_review_path(isbn)?;
    if review_path.exists() {
        fs::remove_file(review_path)?;
    }
    Ok(())
}
