# endoflife-rs

An [https://endoflife.date](endoflife.date) API client written in Rust.

## Installation

TBD

## Usage

### Get All Details

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cycles = get_all_details("python").await?;
    for cycle in cycles {
        println!("{cycle:#?}");
        break;
    }

    Ok(())
}
```

<details>
  <summary>Output</summary>

</details>

### Get Single Cycle Details

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cycle = get_single_cycle_details("python", "3.12").await?;
    println!("{cycle:#?}");

    Ok(())
}
```

<details>
  <summary>Output</summary>

</details>

### Get All Products

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let products = get_all_products().await?;
    println!("{products:#?}");

    Ok(())
}
```

<details>
  <summary>Output</summary>

</details>

## License

- [Apache 2.0](https://choosealicense.com/licenses/apache-2.0/)
- [MIT](https://choosealicense.com/licenses/mit/)
