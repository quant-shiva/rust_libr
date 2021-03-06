mod book;
mod book_item;

use crate::book::Book;
use crate::book_item::BookItem;

use postgres::{Connection, TlsMode};

use std::io::{self};


fn main() -> io::Result<()> {
    println!("\n\n******************************************\n");
    println!("***Welcome to Library Management System***\n\n");

    let conn = Connection::connect("postgresql://postgres:Shivam123@localhost:5432/test", TlsMode::None)
            .unwrap();

    // conn.execute("CREATE TABLE book_item (
    //     id              SERIAL PRIMARY KEY,
    //     bookid               INT,
    //     barcode            VARCHAR NOT NULL,
    //     status            VARCHAR NOT NULL,
    //     subject            VARCHAR NOT NULL,
    //     FOREIGN KEY (bookid) REFERENCES book(id)
    //     )", &[]).unwrap();

    //let b: Book = Book::book_insert(&conn);

    Book::show_book_list(&conn);
    BookItem::show_book_item_list(&conn);
    
    // let mut type_name = String::new();
    // println!("\nEnter the book type: ");
    // io::stdin().read_line(&mut type_name).unwrap();
    // let bt: BookType = BookType::new(type_name.trim_end());
    // conn.execute("INSERT INTO book (isbn, title, subject, publiser, language, author) VALUES ($1, $2, $3, $4, $5, $6)",
    // &[&b.isbn, &b.title, &b.subject, &b.publiser, &b.language, &b.author]).unwrap();
    // let mut barcode = String::new();
    // io::stdin().read_line(&mut barcode).unwrap();
    // let bt_itm: BookItem = BookItem::new(barcode.trim_end(), &b);
    // println!("{:?}", bt);
    // println!("{:?}", b);
    // println!("{:?}",bt_itm);
    Ok(())
}