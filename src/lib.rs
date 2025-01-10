use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::error::Error;

const URL_EOL_API: &str = "https://endoflife.date/api";

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DateOrBool {
    Date(NaiveDate),
    Bool(bool),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Cycle {
    #[serde(default)]
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

pub async fn get_all_details(product: &str) -> Result<Vec<Cycle>, Box<dyn Error>> {
    let url = format!("{}/{}.json", URL_EOL_API, product);

    let response = reqwest::get(url).await?;
    match response.error_for_status() {
        Ok(res) => {
            let cycles = res.json::<Vec<Cycle>>().await?;
            Ok(cycles)
        }
        Err(err) => Err(err.into()),
    }
}

pub async fn get_single_cycle_details(product: &str, cycle: &str) -> Result<Cycle, Box<dyn Error>> {
    let url = format!("{}/{}/{}.json", URL_EOL_API, product, cycle);

    let response = reqwest::get(url).await?;
    match response.error_for_status() {
        Ok(res) => {
            let cycle_temp = res.json::<Cycle>().await?;
            let cycle = Cycle {
                cycle: cycle.to_string(),
                ..cycle_temp
            };

            Ok(cycle)
        }
        Err(err) => Err(err.into()),
    }
}

pub async fn get_all_products() -> Result<Vec<String>, Box<dyn Error>> {
    let url = format!("{}/all.json", URL_EOL_API);

    let response = reqwest::get(url).await?;
    match response.error_for_status() {
        Ok(res) => {
            let products = res.json::<Vec<String>>().await?;
            Ok(products)
        }
        Err(err) => Err(err.into()),
    }
}
