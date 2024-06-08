
use std::env::consts::ARCH;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Paragraph{
    name: String
}

#[derive(Serialize, Deserialize)]
struct Article{
    article: String,
    author: String,
    paragraph: Vec<Paragraph>
}

fn main(){
    let article: Article = Article{
        article: String::from("how to work with json in rust"),
        author: String::from("mn3m"),
        paragraph: vec![
            Paragraph{
                name: String::from("first sentence")
            },
            Paragraph{
                name: String::from("middle sentence")
            },
            Paragraph{
                name: String::from("last sentence")
            }
        ]
    };

    let json = serde_json::to_string(&article).unwrap();
    println!("the json is {}", json);
}