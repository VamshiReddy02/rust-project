use std::collections::HashMap;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // Create a HashMap with key-value pairs
    let mut map = HashMap::new();
    map.insert("lang", "rust");
    map.insert("body", "json");

    // Convert the HashMap to a serde_json::Value
    let json_body = json!(&map);

    // Create a reqwest Client
    let client = reqwest::Client::new();

    // Send a POST request with JSON data
    let res = client.post("http://httpbin.org/post")
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(json_body.to_string())
        .send()
        .await?;

    // Print the status code and response body
    println!("Status: {}", res.status());
    let body = res.text().await?;
    println!("Response Body: {}", body);

    Ok(())
}
