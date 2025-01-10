use endoflife_rs::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cycle = get_single_cycle_details("python", "3.12").await?;
    println!("{cycle:#?}");

    Ok(())
}
