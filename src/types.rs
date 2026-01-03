use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Book {
    pub isbn: String,
    pub title: String,
    pub author: String,
    pub rating: u8,
    pub reading_date: NaiveDate,
    pub has_review: bool,
}

impl fmt::Display for Book {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ISBN: {}\nTITLE: {}\nAUTHOR: {}\nRATING: {}/10\nREADING DATE: {}\nHAS REVIEW: {}",
            self.isbn,
            self.title,
            self.author,
            self.rating,
            self.reading_date.format("%Y-%m-%d"),
            if self.has_review { "yes" } else { "no" },
        )
    }
}
