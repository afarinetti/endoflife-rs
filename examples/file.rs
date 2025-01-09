use std::{error::Error, fs::File, io::BufReader, path::Path};

use endoflife_rs::product::Product;

fn read_products_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<Product>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let u = serde_json::from_reader(reader)?;

    Ok(u)
}

fn main() {
    let products: Vec<Product> = read_products_from_file("examples/data/python.json").unwrap();
    for product in products {
        println!("{product:#?}");
        break;
    }

    let products: Vec<Product> = read_products_from_file("examples/data/node.json").unwrap();
    for product in products {
        println!("{product:#?}");
        break;
    }
}
