use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::{error::Error, path::Path};
use tokio::fs::File;
use tokio::io::AsyncReadExt;

const URL_EOL_API: &str = "https://endoflife.date/api";

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DateOrBool {
    Date(NaiveDate),
    Bool(bool),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Product {
    cycle: Option<String>,
    #[serde(rename = "releaseDate")]
    release_date: NaiveDate,
    eol: DateOrBool,
    latest: String,
    link: Option<String>,
    lts: DateOrBool,
    support: DateOrBool,
    discontinued: Option<String>,
}

pub async fn read_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<Product>, Box<dyn Error>> {
    let mut file = File::open(path).await?;
    let mut buffer = String::new();

    file.read_to_string(&mut buffer).await?;

    let products = serde_json::from_str(&buffer)?;

    Ok(products)
}

pub async fn read_from_url(product_name: &str) -> Result<Vec<Product>, Box<dyn Error>> {
    let url = format!("{}/{}.json", URL_EOL_API, product_name);

    let products = reqwest::get(url).await?.json::<Vec<Product>>().await?;

    Ok(products)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum ProductOrString {
    Product(Product),
    Map(HashMap<String, String>),
}

#[derive(Debug, Clone)]
pub enum ApiError {
    ProductNotFound,
    UnknownError,
}

impl Error for ApiError {}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ApiError::ProductNotFound => write!(f, "Product not found."),
            ApiError::UnknownError => write!(f, "Unknown error."),
        }
    }
}

pub async fn get_single_product_cycle(
    product_name: &str,
    cycle: &str,
) -> Result<Product, Box<dyn Error>> {
    let url = format!("{}/{}/{}.json", URL_EOL_API, product_name, cycle);

    let result = reqwest::get(url).await?.json::<ProductOrString>().await?;

    match result {
        ProductOrString::Product(p) => {
            let result = Product {
                cycle: Some(cycle.to_string()),
                ..p
            };
            Ok(result)
        }
        ProductOrString::Map(m) => {
            let error = if let Some(message) = m.get("message") {
                if "Product not found".eq(message) {
                    ApiError::ProductNotFound
                } else {
                    ApiError::UnknownError
                }
            } else {
                ApiError::UnknownError
            };
            Err(error.into())
        }
    }
}
