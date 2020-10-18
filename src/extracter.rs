use regex::Regex;
use reqwest::Url;

pub fn extract_image_urls(html: &str) -> Result<Vec<Url>, Box<dyn std::error::Error>> {
    let re = Regex::new(r#"<img .*?src="(.*?)".*?>"#)?;
    let urls: Vec<Url> = re
        .captures_iter(html)
        .flat_map(|x| Url::parse(&x[1] ))
        .collect();
    Ok(urls)
}