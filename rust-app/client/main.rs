use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio;

#[derive(Serialize, Deserialize)]
struct MyData {
    name: String,
    age: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let data = MyData {
        name: "Alice".into(),
        age: 30,
    };

    let res = client.post("http://127.0.0.1:8080/greet")
        .json(&data)
        .send()
        .await?
        .text()
        .await?;

    println!("Response: {}", res);

    Ok(())
}
