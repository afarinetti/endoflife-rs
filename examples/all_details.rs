use endoflife_rs::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cycles = get_all_details("python").await?;
    for cycle in cycles {
        println!("{cycle:#?}");
    }

    Ok(())
}
