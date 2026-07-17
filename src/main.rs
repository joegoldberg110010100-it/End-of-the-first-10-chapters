mod struct1;
use struct1::struct2::Book;

use crate::struct1::struct2::most_interesting_word;

fn main() {
    let book1 = Book {
        title: "Rust Programming",
        quote: "Safety and speed.",
    };
    let book2 = Book {
        title: "The Hobbit",
        quote: "In a hole in the ground there lived a hobbit.",
    };

    println!("=== Books ===");
    println!("Book 1");
    println!("Title: {}", book1.title);
    println!("Quote: {}", book1.quote);
    println!("Longer field: {}", book1.longer_field());

    println!();

    println!("Book 2");
    println!("Title: {}", book2.title);
    println!("Quote: {}", book2.quote);
    println!("Longer field: {}", book2.longer_field());

    println!();

    let text = "The Rust language is fast and memory safe";
    let boring_word = "The";

    let interesting = most_interesting_word(text, boring_word);

    println!("=== Interesting Word ===");
    println!("Text: {}", text);
    println!("Boring word: {}", boring_word);
    println!("Most interesting word: {}", interesting);
    
}
