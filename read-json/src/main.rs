use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Paragraph {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Article {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>, // Corrected type
}

fn main() {
    let json = r#"
    {
        "article": "adsvd",
        "author": "author",
        "paragraph": [
            {
                "name": "first paragraph" 
            },
            {
                "name": "2nd paragraph"  
            }
        ]
    }"#;
    let parsed: Article = read_json_typed(json);
    println!("\n\n the name of the 1st paragraph is: {}", parsed.paragraph[0].name);
}

fn read_json_typed(raw_json: &str) -> Article {
    let parsed: Article = serde_json::from_str(raw_json).unwrap();
    return parsed;
}
