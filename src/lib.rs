#![warn(missing_docs)]

//! # endoflife-rs
//!
//! Rust based [endoflife.date](https://endoflife.date/) API client.
//!

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::{error::Error, fmt};

/// URL to the [endoflife.date](https://endoflife.date/) API.
const URL_EOL_API: &str = "https://endoflife.date/api";

/// Either a date ([chrono::NaiveDate]) or boolean ([bool]).
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DateOrBool {
    /// Date returned by the API.
    Date(NaiveDate),

    /// Boolean returned by the API.
    Bool(bool),
}

impl fmt::Display for DateOrBool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            DateOrBool::Date(d) => format!("{}", d),
            DateOrBool::Bool(b) => format!("{}", b),
        };

        write!(f, "{}", str)
    }
}

/// Struct representing the API's return for a single product cycle.
/// - Source: https://endoflife.date/docs/api
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Cycle {
    /// Cycle version (ex: for `Python 3.12.8`, it would return `3.12`).
    #[serde(default)]
    pub cycle: String,

    /// Release date for the first release in this cycle.
    #[serde(rename = "releaseDate")]
    pub release_date: NaiveDate,

    /// Latest release in this cycle
    pub latest: String,

    /// Release date of the latest release in this cycle.
    #[serde(rename = "latestReleaseDate")]
    pub latest_release_date: NaiveDate,

    /// Whether this release cycle has long-term-support (LTS).
    ///   Can be a [date](chrono::NaiveDate) as well if the release enters LTS status on a given date.
    pub lts: DateOrBool,

    /// End of Life Date for this release cycle (or `false` if it is already end of life).
    pub eol: DateOrBool,

    /// Whether this release cycle has active support.
    ///   Can be a [date](chrono::NaiveDate) as well if the release exits active support on a given date.
    pub support: Option<DateOrBool>,

    /// Whether this release cycle has extended support.
    #[serde(rename = "extendedSupport")]
    pub extended_support: Option<DateOrBool>,

    /// Whether this cycle is now discontinued.
    ///   Can be a [date](chrono::NaiveDate) as well if the release is discontinued on a given date.
    pub discontinued: Option<DateOrBool>,

    /// Link to changelog for the latest release, if available.
    pub link: Option<String>,
}

/// Get all cycles by product name.
///
/// # Example
///
/// ```rust
/// use endoflife_rs::*;
///
// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let cycles = get_all_details("python").await?;
///     for cycle in cycles {
///         println!("{cycle:#?}");
///     }
///
///     Ok(())
/// }
/// ```
///
/// # Errors
///
/// This function will return an error if the product does not exist or if the API HTTP GET fails.
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

/// Get a single cycle by product name and cycle.
///
/// # Example
///
/// ```rust
/// use endoflife_rs::*;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let cycle = get_single_cycle_details("python", "3.12").await?;
///     println!("{cycle:#?}");
///
///     Ok(())
/// }
/// ```
///
/// # Errors
///
/// This function will return an error if the product or cycle does not exist or if the API HTTP GET fails.
pub async fn get_single_cycle_details(
    product_name: &str,
    cycle: &str,
) -> Result<Cycle, Box<dyn Error>> {
    let url = format!("{}/{}/{}.json", URL_EOL_API, product_name, cycle);

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

/// Get all product names that are supported by the API.
///
/// # Example
///
/// ```rust
/// use endoflife_rs::*;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let products = get_all_products().await?;
///     println!("{products:#?}");
///     Ok(())
/// }
/// ```
///
/// # Errors
///
/// This function will return an error if the API HTTP GET fails.
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
