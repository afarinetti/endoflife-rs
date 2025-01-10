use endoflife_rs::product::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let products = read_from_url("python").await?;

    for product in products {
        println!("{product:#?}");
        break;
    }

    let product = get_single_product_cycle("python", "3.12").await?;

    println!("{product:#?}");

    Ok(())
}
