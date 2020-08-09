use postgres::Connection;
use crate::book::Book;
use std::io;

#[derive(Debug)]
pub enum BookStatus {
    Available,
    // Reserved,
    // Loaned,
    // Lost
}

#[derive(Debug)]
pub struct BookItem  {
    barcode: String,
    status: BookStatus,
    book: Book
}

impl BookItem {
    pub fn new(barcode: &str, book: &Book) -> BookItem {
        BookItem {
            barcode: barcode.to_string(),
            status: BookStatus::Available,
            book: Book {
                isbn: book.isbn.to_string(),
                title: book.title.to_string(),
                subject: book.subject.to_string(),
                publiser: book.publiser.to_string(),
                language: book.language.to_string(),
                author: book.author.to_string()
            }
        }
    }

    pub fn insert_book_item(conn: &Connection) {
        let mut barcode: String = String::new();
        let mut status: String = String::new();
        let mut bookid: String = String::new();

        println!("\nEnter the BarCode: ");
        io::stdin().read_line(&mut barcode).unwrap();
        println!("\nEnter the Book Status: ");
        println!("1. Available  2. Reserved  3. Loaned  4. Lost");
        io::stdin().read_line(&mut status).unwrap();
        println!("\nEnter the Book Id: ");
        io::stdin().read_line(&mut bookid).unwrap();

        barcode = String::from(barcode.trim_end());
        status = String::from(status.trim_end());
        bookid = String::from(bookid.trim_end());

        conn.execute("INSERT INTO book_item (bookid, barcode, status) VALUES ($1, $2, $3)",
            &[&bookid.parse::<i32>().unwrap(), &barcode, &status]).unwrap();
    }

    pub fn show_book_item_list(conn: &Connection) {
        for row in &conn.query("SELECT id, bookid, barcode, status FROM book_item", &[]).unwrap() {
            let id: i32 = row.get(1);
            
            let bk: Book = Book::get_book_by_id(&conn, &id).unwrap();

            let book_i: BookItem = BookItem {
                barcode: row.get(2),
                status: BookStatus::Available,
                book: bk
            };
            println!("Found book:\n {:#?}", book_i);
            }
    }
}