use endoflife_rs::product::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let products = get_all_details("python").await?;
    for product in products {
        println!("{product:#?}");
        break;
    }

    let product = get_single_cycle_details("python", "3.12").await?;
    println!("{product:#?}");

    let product_names = get_all_products().await?;
    println!("{product_names:#?}");

    Ok(())
}
