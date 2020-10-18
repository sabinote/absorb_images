
mod util;
mod downloader;
mod extracter;

use std::path::PathBuf;
use chrono::{Local, DateTime};   
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("対象アドレスのみを指定してください");
        return Ok(());
    } 
    let url = &args[1];
    let client = util::make_client();
    let html = downloader::download_html(&client, url).await?;
    let urls = extracter::extract_image_urls(&html)?;

    println!("画像数:{}", urls.len());
    let local_datetime: DateTime<Local> = Local::now(); 
    let dir = PathBuf::from(format!("./{}", local_datetime.format("%F %H%M%S%6f")));
    if urls.len() == 0 {
        return Ok(());
    }
    else {
        std::fs::create_dir(&dir)?;
    }
  
    for (_i, url) in urls.into_iter().enumerate() {
        println!("対象アドレス:\t{}", url.to_string());
        println!("ダウンロード中...");
        let v = match downloader::download_image(&client, &url.as_ref()).await {
            Ok(x) => x,
            Err(e) => {
                println!("{}", e);
                continue;
            },
        };
        println!("保存中...");
        util::save(&dir,url.path_segments().unwrap().last().unwrap(), v)?;
    }
    Ok(())
}