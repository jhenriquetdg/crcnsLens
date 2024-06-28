use url as urllib;

pub async fn get_url_html(url: urllib::Url) -> String {
    let site_future = reqwest::get(url.clone());
    let site_result = site_future.await;
    let site_response = match site_result {
        Ok(response) => response,
        Err(e) => {
            println!("Unable to get {url}: {e}");
            panic!("Failed.")
        }
    };

    let site_html_future = site_response.text();
    let site_html_result = site_html_future.await;
    match site_html_result {
        Ok(html) => html,
        Err(e) => {
            println!("Unabled to get {url} html: {e}");
            panic!("Failed");
        }
    }
}
