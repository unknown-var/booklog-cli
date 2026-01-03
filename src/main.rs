use chrono::Local;
use std::env;
use std::io::{self, Write};

mod config;
mod isbn;
mod types;
use types::Book;
mod review_writer;

mod file_loading;
fn ask(question: &str) -> String {
    print!("{}", question);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn add_new_book() {
    let isbn = loop {
        let answer = ask("Please input the ISBN number of your book: ");
        let answer = answer.replace("-", "");
        let answer = answer.trim().to_string();
        if isbn::check_isbn(&answer) {
            break answer;
        }
        println!("wrong ISBN format");
    };
    let title: String;
    let author: String;
    println!("Requesting book information from openlibrary.org");
    if let Ok((_title, _author)) = isbn::api::request_book_with_isbn(&isbn) {
        author = _author;
        title = _title;
    } else {
        println!("Failed to call isbn api");
        title = ask("Please input the books title: ");
        author = ask("Please input the authors name: ");
    }
    let rating = loop {
        let answer = ask("Please rate the book from 0 to 10: ");
        if let Ok(rating) = answer.parse::<u8>() {
            if rating <= 10 {
                break rating;
            }
        }
        println!("Wrong format for rating");
    };
    let has_review = loop {
        let answer = ask("do you want to write a review [y/n]: ").to_ascii_lowercase();
        if answer.starts_with("y") {
            match review_writer::create_review(&isbn) {
                Ok(()) => (),
                Err(e) => {
                    println!("failed to write review because: {}", e);
                    return;
                }
            }
            break true;
        } else if answer.starts_with("n") {
            break false;
        }
    };
    let book = Book {
        isbn: isbn,
        title: title,
        author: author,
        rating: rating,
        reading_date: Local::now().date_naive(),
        has_review: has_review,
    };
    println!("this book is added to the file book: \n{}", book);
    let _ = file_loading::save_book(book);
}

fn list_all_books() {
    let books = match file_loading::retrieve_books() {
        Ok(v) => v,
        Err(e) => {
            println!("failed to read book file because: {}", e);
            return;
        }
    };
    for book in books {
        println!("\n{}", book);
    }
}

fn delete_entry() {
    println!("\n#### ALL TITLES ####\n");

    let books = match file_loading::retrieve_books() {
        Ok(v) => v,
        Err(e) => {
            println!("failed to read book file because: {}", e);
            return;
        }
    };
    let mut i = 0;
    for book in books {
        i += 1;
        println!("{}. {}", i, book.title);
    }

    let book_to_delete = loop {
        let answer = ask("\nput in the index of the title you want to delete: ");
        if answer.to_ascii_lowercase().starts_with("q") {
            return;
        }
        if let Ok(index) = answer.parse::<usize>() {
            if index == 0 {
                println!("index 0 dose not exist");
                continue;
            } else if index <= i {
                break index - 1;
            } else {
                println!("index to high");
                continue;
            }
        }
        println!("Wrong format for index");
    };
    let _ = file_loading::delete_book(book_to_delete);
}

fn run_option(option: &str) {
    let first_letter = match option.chars().next() {
        Some(v) => v,
        None => {
            println!("option string is empty");
            return;
        }
    };
    match first_letter {
        'n' => add_new_book(),
        'l' => list_all_books(),
        'd' => delete_entry(),
        _ => println!("{} is not a valid option", first_letter),
    }
}

fn menu() {
    let msg = r#"
#### MENU ####
type option then enter:
[n] new book entry
[l] list old book entrys
[d] delete entry
[q] quit
"#;
    loop {
        println!("{}", msg);
        let input = ask("# ");
        if input.starts_with('q') {
            break;
        }
        run_option(&input);
    }
}

fn main() {
    config::init_config();
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        menu();
        return;
    }
    if (&args[1]).starts_with('-') {
        run_option(&(&args[1])[1..]);
    } else {
        println!("{} could not be read as an option", &args[1]);
    }
}
