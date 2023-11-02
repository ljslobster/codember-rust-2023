use std::collections::HashMap;
use reqwest::Error;

async fn get_text(url: &str) -> Result<String, Error> {
    let response = reqwest::get(url).await?;
    let text = response.text().await?;
    Ok(text)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let url = "https://codember.dev/data/message_01.txt";
    let text = get_text(url).await?;

    let mut map = HashMap::new();
    let mut order = Vec::new();

    for word in text.split_whitespace() {
        if !map.contains_key(word) {
            order.push(word);
        }
        let counter = map.entry(word).or_insert(0);
        *counter += 1;
    }

    let mut result: String = String::new();
    for &key in &order {
        if let Some(&value) = map.get(key) {
            result += &format!("{}{}", key, value);
        }
    }

    println!("{}", result);
    
    Ok(())
}