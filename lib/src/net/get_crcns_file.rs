use std::collections::HashMap;

use reqwest::Client;
use url::Url;

pub async fn get_crcns_file(filepath: &str) -> reqwest::Response {
    let base_url = "https://portal.nersc.gov/project/crcns/download/index.php";
    let url = Url::parse(base_url).unwrap();
    let url = url.join(filepath).unwrap();

    let username = std::env::var("CRCNS_USERNAME").unwrap();
    let password = std::env::var("CRCNS_PASSWORD").unwrap();

    let mut request_data = HashMap::new();
    request_data.insert("username", username.as_str());
    request_data.insert("password", password.as_str());
    request_data.insert("fn", "hc-3/filelist.txt");
    request_data.insert("submit", "Login");

    let client = Client::new();
    client.post(url).form(&request_data).send().await.unwrap()
}
