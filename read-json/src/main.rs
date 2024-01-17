use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Paragraph {
    name: String
}
#[derive(Serialize, Deserialize)]
struct Article {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>
}

fn main() {
    let json = r#"
    {
        "article": "how to work with json",
        "author": "Vamshi reddy",
        "paragraph": [
            {
                "name": "asdfghjkl"
            },
            {
                "name": "qwertyuiop"
            }
        ]
    }
    "#;  

    let parsed: Article = read_from_json(json);
    println!("{}",parsed.paragraph[0].name);
}

fn read_from_json(raw_json: &str) -> Article {
    let parsed: Article = serde_json::from_str(raw_json).unwrap();
    return parsed;
}
