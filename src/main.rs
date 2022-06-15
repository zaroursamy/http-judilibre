use reqwest::header::{ACCEPT, CONTENT_TYPE};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct JudiResult {
    id: String,
    source: String,
    text: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct JudiResponse {
    batch: u8,
    batch_size: u8,
    results: Vec<JudiResult>,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let base_export_url = "https://sandbox-api.piste.gouv.fr/cassation/judilibre/v1.0/export?";
    let date_start = "2022-01-01";
    let date_end = "2022-05-30";
    let batch_size = 2;
    let batch = 0;

    let key_id = "68e791c6-974b-4522-a8f9-dd4bb6a5ea68";
    let url = format!("{base_export_url}batch={batch}&batch_size={batch_size}&date_start={date_start}&date_end={date_end}");

    println!("{:?}", &url);

    let client = Client::new();

    let response: JudiResponse = client
        .get(url)
        .header("KeyId", key_id)
        .header(ACCEPT, "application/json")
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await?
        .json()
        .await?;

    println!("Success! {:#?}", response);

    Ok(())
}
