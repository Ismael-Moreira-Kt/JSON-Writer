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

    loop {
        println!("Enter a paragraph (or 0 to finish):");
        let mut paragraph = String::new();
        io::stdin().read_line(&mut paragraph).expect("Failed to read line");

        if paragraph.trim() == "0" {
            break;
        }

        article.paragraphs.push(Paragraph {
            name: paragraph.trim().to_string(),
        });
    }

    let article_json = serde_json::to_string(&article).expect("Failed to serialize article");
    let json_filename = "article.json";

    let mut file = File::create(json_filename).expect("Failed to create file");
    file.write_all(article_json.as_bytes()).expect("Failed to write to file");
}