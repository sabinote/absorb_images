use reqwest::{ClientBuilder, Client};
use std::path::Path;
use std::io::{BufWriter, Write};
use std::fs::File;


pub fn make_client() -> Client {
    ClientBuilder::new()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/69.0.3497.100")
        .build()
        .unwrap()
}
pub fn save(path: &Path,filename: &str, data: Vec<u8>) -> Result<(), Box<dyn std::error::Error>>{
    let path = path.join(filename);
    let mut f = BufWriter::new(File::create(path)?);
    f.write(&data)?;
    Ok(())
}