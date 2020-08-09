use crate::book::Book;

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
}