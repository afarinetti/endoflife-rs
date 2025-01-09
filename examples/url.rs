use endoflife_rs::product::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let products = read_from_url("python").await?;

    for p in products {
        println!("{p:#?}");
        break;
    }

    Ok(())
}
