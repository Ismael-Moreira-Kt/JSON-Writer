use std::io;
use serde::{Serialize, Deserialize};
use serde_json;
use std::fs::File;
use std::io::Write;



#[derive(Debug, Serialize, Deserialize)]
struct Paragraph {
    name: String,
}


#[derive(Debug, Serialize, Deserialize)]
struct Article {
    article: String,
    author: String,
    paragraphs: Vec<Paragraph>, // Corrected field name
}



fn main() {
    let mut article = Article {
        article: String::new(),
        author: String::new(),
        paragraphs: Vec::new(),
    };

    println!("Enter the article:");
    io::stdin().read_line(&mut article.article).expect("Failed to read line");

    println!("Enter the author:");
    io::stdin().read_line(&mut article.author).expect("Failed to read line");
}