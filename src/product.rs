use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::{error::Error, fs::File, io::BufReader, path::Path};

const URL_EOL_API: &str = "https://endoflife.date/api";

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DateOrBool {
    Date(NaiveDate),
    Bool(bool),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    cycle: String,
    #[serde(rename = "releaseDate")]
    release_date: NaiveDate,
    eol: DateOrBool,
    latest: String,
    link: Option<String>,
    lts: DateOrBool,
    support: DateOrBool,
    discontinued: Option<String>,
}

pub fn read_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<Product>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let products = serde_json::from_reader(reader)?;

    Ok(products)
}

pub async fn read_from_url(product_name: &str) -> Result<Vec<Product>, Box<dyn Error>> {
    let url = format!("{}/{}.json", URL_EOL_API, product_name);

    let products = reqwest::get(url).await?.json::<Vec<Product>>().await?;

    Ok(products)
}
