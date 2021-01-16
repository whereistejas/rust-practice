use std::collections::HashMap;

#[derive(Debug)]
struct BookDetails {
    name: String,
    author: String,
    isbn: u32,
    release_year: u16,
    publisher: String,
    genres: Vec<String>,
}

fn main() {
    // key-> book_id::<i32>
    // value-> details::<bookDetails>
    let mut book_collection = HashMap::new();

    let book_1 = BookDetails{
        name: "The wizard of Oz".to_string(),
        author: "Oz".to_string(),
        isbn: 123456789,
        release_year: 2020,
        publisher: "Random House".to_string(),
        genres: vec!["fantasy".to_string(), "magic".to_string(), "wizardary".to_string()],
    };

    book_collection.insert(1, book_1);

    let book_2 = BookDetails{
        name: "Ikigai".to_string(),
        author: "Generic Person".to_string(),
        isbn: 123456789,
        release_year: 2020,
        publisher: "Random House".to_string(),
        genres: vec!["self-help".to_string(), "life purpose".to_string(), "mentality".to_string()],
    };

    book_collection.insert(2, book_2);

    let book_3 = BookDetails{
        name: "Amnesty".to_string(),
        author: "Aravind Adiga".to_string(),
        isbn: 123456789,
        release_year: 2020,
        publisher: "Random House".to_string(),
        genres: vec!["thriller".to_string(), "fiction".to_string(), "man booker prize".to_string()],
    };

    book_collection.entry(2).or_insert( book_3);

    println!("{:#?}", book_collection);
}
