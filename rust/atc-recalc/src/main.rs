use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    let url = "url here";

    for sub_fund_id in get_sub_fund_ids() {
        let request = Request {
            regulatory_contexts: vec![6],
            fund_ids: vec![],
            sub_fund_ids: vec![sub_fund_id],
            effective_date: "2024-12-31".to_string(),
        };

        match client
            .post(url)
            .header("Cookie", "cookie here")
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
        {
            Ok(response) => {
                println!("Status: {}", response.status());
                println!("Response: {}", response.text().await.unwrap_or_default());
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Request {
    regulatory_contexts: Vec<i32>,
    fund_ids: Vec<String>,
    sub_fund_ids: Vec<String>,
    effective_date: String,
}

fn get_sub_fund_ids() -> Vec<String> {
    vec![
        // sub fund ids here
    ]
}
