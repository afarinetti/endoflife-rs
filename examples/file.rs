use endoflife_rs::product::*;
use std::error::Error;

const PATH_PYTHON_JSON: &str = "examples/data/python.json";
const PATH_NODE_JSON: &str = "examples/data/node.json";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let products: Vec<Product> = read_from_file(PATH_PYTHON_JSON).await?;
    for product in products {
        println!("{product:#?}");
        break;
    }

    let products: Vec<Product> = read_from_file(PATH_NODE_JSON).await?;
    for product in products {
        println!("{product:#?}");
        break;
    }

    Ok(())
}
