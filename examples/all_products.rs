use endoflife_rs::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let products = get_all_products().await?;
    println!("{products:#?}");

    Ok(())
}
