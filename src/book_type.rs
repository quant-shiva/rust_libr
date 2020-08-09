#[derive(Debug)]
pub struct BookType {
    type_id: String,
    type_name: String,
    available_books: usize
}

impl BookType {
    pub fn new(name: &str) -> BookType {
        //Do some DB stuff
        let type_id: String = String::from("this_is_book_id");
        let available_books: usize = 1;
        let type_name: String = name.to_string();
        BookType {
            type_id,
            type_name,
            available_books
        }
    }
}