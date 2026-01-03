use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct SubPart {
    key: String,
}

#[derive(Deserialize, Debug)]
struct Book {
    authors: Vec<SubPart>,
    title: String,
}

#[derive(Deserialize, Debug)]
struct Author {
    name: String,
}

fn request_author(author_link: &str) -> Result<String, Box<dyn std::error::Error>> {
    let request_url = format!("https://openlibrary.org{author_link}.json");
    let client = reqwest::blocking::Client::new();
    let response = client.get(request_url).send()?;

    let author: Author = response.json()?;
    Ok(author.name)
}

pub fn request_book_with_isbn(isbn: &str) -> Result<(String, String), Box<dyn std::error::Error>> {
    let request_url = format!("https://openlibrary.org/isbn/{isbn}.json");
    let client = reqwest::blocking::Client::new();
    let response = client.get(request_url).send()?;

    let book: Book = response.json()?;
    println!("found book");
    let author_tag = &book.authors.get(0).unwrap().key;
    let author_name = request_author(author_tag)?;
    println!("found author");
    Ok((book.title, author_name))
}
