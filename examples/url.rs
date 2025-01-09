use endoflife_rs::product::Product;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://endoflife.date/api/python.json")
        .await?
        .json::<Vec<Product>>()
        .await?;

    for p in resp {
        println!("{p:#?}");
        break;
    }

    Ok(())
}
