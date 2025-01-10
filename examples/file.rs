use endoflife_rs::product::*;
use std::{error::Error, path::Path};
use tokio::fs::File;
use tokio::io::AsyncReadExt;

const PATH_PYTHON_JSON: &str = "examples/data/python.json";
const PATH_NODE_JSON: &str = "examples/data/node.json";

pub async fn read_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<Cycle>, Box<dyn Error>> {
    let mut file = File::open(path).await?;
    let mut buffer = String::new();

    file.read_to_string(&mut buffer).await?;

    let cycles = serde_json::from_str(&buffer)?;

    Ok(cycles)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let products: Vec<Cycle> = read_from_file(PATH_PYTHON_JSON).await?;
    for product in products {
        println!("{product:#?}");
        break;
    }

    let products: Vec<Cycle> = read_from_file(PATH_NODE_JSON).await?;
    for product in products {
        println!("{product:#?}");
        break;
    }

    Ok(())
}
