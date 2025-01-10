[![Rust](https://github.com/afarinetti/endoflife-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/afarinetti/endoflife-rs/actions/workflows/rust.yml) - [![rust-clippy analyze](https://github.com/afarinetti/endoflife-rs/actions/workflows/rust-clippy.yml/badge.svg)](https://github.com/afarinetti/endoflife-rs/actions/workflows/rust-clippy.yml)

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
  <pre>
    Cycle {
      cycle: "3.13",
      release_date: 2024-10-07,
      eol: Date(
        2029-10-31,
      ),
      latest: "3.13.1",
      link: None,
      lts: Bool(
        false,
      ),
      support: Date(
        2026-10-01,
      ),
      discontinued: None,
    }
    Cycle {
      cycle: "3.12",
      release_date: 2023-10-02,
      eol: Date(
        2028-10-31,
      ),
      latest: "3.12.8",
      link: None,
      lts: Bool(
        false,
      ),
      support: Date(
        2025-04-02,
      ),
      discontinued: None,
    }

    // << snip >>

    Cycle {
      cycle: "3.0",
      release_date: 2008-12-03,
      eol: Date(
        2009-06-27,
      ),
      latest: "3.0.1",
      link: None,
      lts: Bool(
        false,
      ),
      support: Bool(
        false,
      ),
      discontinued: None,
    }
    Cycle {
      cycle: "2.6",
      release_date: 2008-10-01,
      eol: Date(
        2013-10-29,
      ),
      latest: "2.6.9",
      link: None,
      lts: Bool(
        false,
      ),
      support: Bool(
        false,
      ),
      discontinued: None,
    }
  </pre>
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
  <summary markdown="span">Output</summary>
  <pre>
    Cycle {
      cycle: "3.12",
      release_date: 2023-10-02,
      eol: Date(
        2028-10-31,
      ),
      latest: "3.12.8",
      link: None,
      lts: Bool(
        false,
      ),
      support: Date(
        2025-04-02,
      ),
      discontinued: None,
    }
  </pre>
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
  <pre>
    [
      "akeneo-pim",
      "alibaba-dragonwell",
      "almalinux",
      "alpine",
      "amazon-cdk",
      "amazon-corretto",

      // << snip >>

      "xcp-ng",
      "yarn",
      "yocto",
      "zabbix",
      "zentyal",
      "zerto",
      "zookeeper",
    ]
  </pre>
</details>

## License

- [Apache 2.0](https://choosealicense.com/licenses/apache-2.0/)
- [MIT](https://choosealicense.com/licenses/mit/)
