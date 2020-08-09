mod book;
mod book_type;
mod book_item;

//extern crate  postgres;

use crate::book_type::BookType;
use crate::book::Book;
use crate::book_item::BookItem;

use postgres::{Connection, TlsMode};

use std::io::{self};


fn main() -> io::Result<()> {
    println!("\n\n******************************************\n");
    println!("***Welcome to Library Management System***\n\n");

    let conn = Connection::connect("postgresql://postgres:Shivam123@localhost:5432/test", TlsMode::None)
            .unwrap();

    // conn.execute("CREATE TABLE BOOK (
    //     id              SERIAL PRIMARY KEY,
    //     isbn            VARCHAR NOT NULL,
    //     title            VARCHAR NOT NULL,
    //     subject            VARCHAR NOT NULL,
    //     publiser            VARCHAR NOT NULL,
    //     language            VARCHAR NOT NULL,
    //     author            VARCHAR NOT NULL
    //     )", &[]).unwrap();


    for row in &conn.query("SELECT id, isbn, title, subject, publiser, language, author FROM book", &[]).unwrap() {
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
    
    // let mut type_name = String::new();
    // println!("\nEnter the book type: ");
    // io::stdin().read_line(&mut type_name).unwrap();
    // let bt: BookType = BookType::new(type_name.trim_end());
    // let b: Book = Book::book_init();
    // conn.execute("INSERT INTO book (isbn, title, subject, publiser, language, author) VALUES ($1, $2, $3, $4, $5, $6)",
    // &[&b.isbn, &b.title, &b.subject, &b.publiser, &b.language, &b.author]).unwrap();
    // let mut barcode = String::new();
    // println!("\nEnter the BarCode: ");
    // io::stdin().read_line(&mut barcode).unwrap();
    // let bt_itm: BookItem = BookItem::new(barcode.trim_end(), &b);
    // println!("{:?}", bt);
    // println!("{:?}", b);
    // println!("{:?}",bt_itm);
    Ok(())
}