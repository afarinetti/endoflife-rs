mod product;

use std::{error::Error, fs::File, io::BufReader, path::Path};

use crate::product::Product;

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let resp = reqwest::get("https://endoflife.date/api/python.json")
//         .await?
//         .json::<Vec<Product>>()
//         .await?;

//     for p in resp {
//         println!("{p:#?}");
//         break;
//     }

//     Ok(())
// }
//

fn read_products_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<Product>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let u = serde_json::from_reader(reader)?;

    Ok(u)
}

fn main() {
    let products: Vec<Product> = read_products_from_file("python.json").unwrap();
    for product in products {
        println!("{product:#?}");
        break;
    }

    let products: Vec<Product> = read_products_from_file("node.json").unwrap();
    for product in products {
        println!("{product:#?}");
        break;
    }
}
