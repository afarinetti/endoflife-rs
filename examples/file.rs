use clap::Parser;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{Cell, CellAlignment, Color, ContentArrangement, Table};
use endoflife_rs::*;
use std::{error::Error, fmt, path::Path, path::PathBuf};
use tokio::fs::File;
use tokio::io::AsyncReadExt;

const PATH_EXAMPLE_DATA: &str = "examples/data/";

async fn read_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<Cycle>, Box<dyn Error>> {
    let mut file = File::open(path).await?;
    let mut buffer = String::new();

    file.read_to_string(&mut buffer).await?;

    let cycles = serde_json::from_str(&buffer)?;

    Ok(cycles)
}

fn cycle_to_vec(cycle: Cycle) -> Vec<Cell> {
    let today = chrono::Local::now().date_naive();

    let eol_good = match cycle.eol {
        DateOrBool::Date(d) => d > today,
        DateOrBool::Bool(b) => !b,
    };
    let color_eol = if eol_good { Color::Green } else { Color::Red };

    vec![
        Cell::new(cycle.cycle),
        Cell::new(format!("{}", cycle.release_date)),
        Cell::new(cycle.latest),
        Cell::new(format!("{}", cycle.latest_release_date)),
        Cell::new(format!("{}", cycle.eol)).fg(color_eol),
    ]
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Name of the product to query
    product: String,
}

#[derive(Debug, Clone)]
struct FileNotFound(PathBuf);

impl fmt::Display for FileNotFound {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "File not found: {}", self.0.display())
    }
}

impl Error for FileNotFound {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            Cell::new("Cycle").set_alignment(CellAlignment::Center),
            Cell::new("Release").set_alignment(CellAlignment::Center),
            Cell::new("Latest").set_alignment(CellAlignment::Center),
            Cell::new("Latest Release").set_alignment(CellAlignment::Center),
            Cell::new("EOL").set_alignment(CellAlignment::Center),
        ]);

    for i in vec![1, 3, 4].into_iter() {
        if let Some(col) = table.column_mut(i) {
            col.set_cell_alignment(CellAlignment::Center);
        }
    }

    let path = Path::new(PATH_EXAMPLE_DATA)
        .join(args.product)
        .with_extension("json");

    match path.try_exists() {
        Ok(true) => {
            let cycles: Vec<Cycle> = read_from_file(path).await?;
            for cycle in cycles {
                table.add_row(cycle_to_vec(cycle));
            }

            println!("{table}");

            Ok(())
        }
        Ok(false) => Err(FileNotFound(path).into()),
        Err(err) => Err(err.into()),
    }
}
