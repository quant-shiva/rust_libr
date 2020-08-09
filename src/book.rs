use std::io;

extern crate postgres;

use postgres::Connection;

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

    pub fn book_insert(conn: &Connection) -> Book {
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

        conn.execute("INSERT INTO book (isbn, title, subject, publiser, language, author) VALUES ($1, $2, $3, $4, $5, $6)",
            &[&book.isbn, &book.title, &book.subject, &book.publiser, &book.language, &book.author]).unwrap();
        book
    }

    pub fn show_book_list(conn: &Connection) {
        for row in &conn.query("SELECT id, isbn, title, subject, publiser, language, author FROM book", &[]).unwrap() {
            let id: i32 = row.get(0);
            println!("ID: {}", id);
            let books: Book = Book {
                isbn: row.get(1),
                title: row.get(2),
                subject: row.get(3),
                publiser: row.get(4),
                language: row.get(5),
                author: row.get(6)
            };
            println!("Found book:\n {:#?}", books);
            }
    }

    pub fn get_book_by_id(conn: &Connection, id: &i32) -> Option<Book> {
        for row in &conn.query("SELECT id, isbn, title, subject, publiser, language, author FROM book WHERE id = 1", &[]).unwrap() {
            let id: i32 = row.get(0);
            println!("ID: {}", id);
            let books: Book = Book {
                isbn: row.get(1),
                title: row.get(2),
                subject: row.get(3),
                publiser: row.get(4),
                language: row.get(5),
                author: row.get(6)
            };
            println!("Found book:\n {:#?}", books);
            return Some(books);
        }
        None
    }
}