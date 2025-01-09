use endoflife_rs::product::*;

const PATH_PYTHON_JSON: &str = "examples/data/python.json";
const PATH_NODE_JSON: &str = "examples/data/node.json";

fn main() {
    let products: Vec<Product> = read_from_file(PATH_PYTHON_JSON).unwrap();
    for product in products {
        println!("{product:#?}");
        break;
    }

    let products: Vec<Product> = read_from_file(PATH_NODE_JSON).unwrap();
    for product in products {
        println!("{product:#?}");
        break;
    }
}
