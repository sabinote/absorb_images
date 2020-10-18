use reqwest::{Url, Client};

pub async fn download_html(client: &Client, url: &str) -> Result<String, Box<dyn std::error::Error>>{
    let url = Url::parse(url)?;
    let res = client.get(url)
        .send()
        .await?
        .text()
        .await?;
    Ok(res)
}

pub async fn download_image(client: &Client, url: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let url = Url::parse(url)?;
    let res = client.get(url)
        .send()
        .await?
        .bytes()
        .await?;
    Ok(res.to_vec())
}
