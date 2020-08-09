use std::io;

#[derive(Debug)]
pub struct Book {
    pub isbn: String,
    pub title: String,
    pub subject: String,
    pub publiser: String,
    pub language: String,
    pub author: String
}

impl Book {
    pub fn new(isbn: &str, title: &str, subject: &str, publiser: &str, language: &str, author: &str) -> Book {
        Book {
            isbn: isbn.to_string(),
            title: title.to_string(),
            subject: subject.to_string(),
            publiser: publiser.to_string(),
            language: language.to_string(),
            author: author.to_string()
        }
    }

    pub fn book_init() -> Book {
        let mut isbn = String::new();
        let mut title = String::new();
        let mut subject = String::new();
        let mut publiser = String::new();
        let mut language = String::new();
        let mut author = String::new();

        println!("\nEnter the book ISBN No: ");
        io::stdin().read_line(&mut isbn).unwrap();

        println!("\nEnter the book Title: ");
        io::stdin().read_line(&mut title).unwrap();

        println!("\nEnter the book Subject: ");
        io::stdin().read_line(&mut subject).unwrap();

        println!("\nEnter the book Publiser: ");
        io::stdin().read_line(&mut publiser).unwrap();

        println!("\nEnter the book Language: ");
        io::stdin().read_line(&mut language).unwrap();

        println!("\nEnter the book Author: ");
        io::stdin().read_line(&mut author).unwrap();
        
        let book: Book = Book::new(
            isbn.trim_end(),
            title.trim_end(),
            subject.trim_end(),
            publiser.trim_end(),
            language.trim_end(),
            author.trim_end()
        );
        book
    }
}