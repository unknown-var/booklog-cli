use crate::config;
use crate::types::Book;
use std::error::Error;
use std::fs::{self, OpenOptions};
use std::path::{Path, PathBuf};

fn check_dir(path: &str) -> Result<(), Box<dyn Error>> {
    let dir_path = Path::new(path);

    if !dir_path.exists() {
        match fs::create_dir(dir_path) {
            Ok(()) => {}
            Err(e) => {
                eprintln!("Failed to create directory at: {}", path);
                return Err(Box::new(e));
            }
        }
    }

    Ok(())
}

fn get_book_path() -> Result<PathBuf, Box<dyn Error>> {
    let mut path: String = config::get_config().data_path.clone();

    // Expanding config path
    let expanded_path = shellexpand::tilde(&path);
    path = expanded_path.to_string();

    path.push_str("/");
    path.push_str(&config::get_config().data_dir_name);
    check_dir(&path)?;
    path.push_str("/");
    path.push_str(&config::get_config().book_file);
    Ok(PathBuf::from(path))
}

pub fn get_review_path(isbn: &str) -> Result<PathBuf, Box<dyn Error>> {
    let mut path: String = config::get_config().data_path.clone();

    // Expanding config path
    let expanded_path = shellexpand::tilde(&path);
    path = expanded_path.to_string();

    path.push_str("/");
    path.push_str(&config::get_config().data_dir_name);
    check_dir(&path)?;
    path.push_str("/");
    path.push_str(&isbn);
    path.push_str("_review.txt");
    Ok(PathBuf::from(path))
}

pub fn save_book(book: Book) -> Result<(), Box<dyn Error>> {
    let path = get_book_path()?;

    let file_exists = path.exists();

    // Open the file in append mode
    let file = OpenOptions::new()
        .append(true)
        .create(true) // create it if it doesn't exist
        .open(path)?;

    // Don't write headers again
    let mut wtr = csv::WriterBuilder::new()
        .has_headers(!file_exists)
        .from_writer(file);

    // Serialize and write the book record
    wtr.serialize(book)?;
    wtr.flush()?;

    Ok(())
}

pub fn retrieve_books() -> Result<Vec<Book>, Box<dyn Error>> {
    let path = get_book_path()?;
    let mut books = Vec::new();
    let mut rdr = csv::Reader::from_path(path)?;
    for result in rdr.deserialize() {
        let book: Book = result?;
        books.push(book);
    }
    Ok(books)
}

pub fn delete_book(index: usize) -> Result<(), Box<dyn Error>> {
    // 1. Retrieve the current list of books
    let mut books = retrieve_books()?;

    // 2. Check if the index is valid to avoid a program crash (panic)
    if index >= books.len() {
        return Err("Index out of bounds: No book found at that position.".into());
    }

    // 3. Optional: Delete the review file associated with this book
    let book_to_remove = &books[index];
    let review_path = get_review_path(&book_to_remove.isbn)?;
    if review_path.exists() {
        fs::remove_file(review_path)?;
    }

    // 4. Remove the book from the vector
    books.remove(index);

    // 5. Overwrite the file with the new list
    let path = get_book_path()?;

    // Using File::create truncates (clears) the file automatically
    let file = fs::File::create(path)?;
    let mut wtr = csv::WriterBuilder::new()
        .has_headers(true)
        .from_writer(file);

    for book in books {
        wtr.serialize(book)?;
    }
    wtr.flush()?;

    Ok(())
}
