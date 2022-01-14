use super::data::Product;
use serde_json;
use std::collections::HashMap;
use std::fs;

/// Parses a product from a string to a `Product` struct
///
/// # Panics
///
/// Panics if the product passed to the function is invalid.
///
/// # Example
/// ```rs
/// parse_product("{\"id\":\"1020\",\"name\":\"Test\",\"price\":\"1020\"}")
/// // > Product { id: "1020", "name": "Test", "price": "1020" }
/// ```
fn parse_product(line: String) -> Product {
    let line = line.as_str();
    let product = serde_json::from_str(line).unwrap();

    product
}

/// Indexes a list of products by looking for a given filename
///
/// # Panics
///
/// Panics if the file isn't found in the file sytstem or if one of the contents
/// of the file is invalid (cannot be serialized to a `Product` struct).
///
/// # Example
///
/// ```rs
/// index_product("$SOURCE_FILE");
/// // > HashMap<String, Product>
/// ```
pub fn index_products(filename: &str) -> HashMap<String, Product> {
    let contents = fs::read_to_string(filename).expect("Couldn't read source file");
    let mut map: HashMap<String, Product> = HashMap::new();

    let products = contents
        .split('\n')
        .map(|line| line.to_owned())
        .map(parse_product)
        .collect::<Vec<Product>>();

    for product in products {
        map.insert(product.id.clone(), product);
    }

    map
}
