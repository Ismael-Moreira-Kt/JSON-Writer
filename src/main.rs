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