use std::process;
use reqwest::header::USER_AGENT;
use serde::{Deserialize, Serialize};

pub async fn get(username: String) -> Result<UserData, Box<dyn std::error::Error>> {
    let url = format!("https://api.github.com/users/{}", username);

    // Get the body of the request
    let client = reqwest::Client::new();
    let res = client
        .get(url)
        .header(USER_AGENT, "octofetch cli")
        .send()
        .await?
        .text()
        .await?;
    // The json of the api's body
    let user: UserData = serde_json::from_str(&res)?;

    Ok(user)
}

#[tokio::main]
async fn main() {
    let user = get("cimok".to_string()).await?;
    println!("{}", user.name.to_string());
}
