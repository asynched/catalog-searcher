use super::data::Product;
use std::collections::HashMap;

pub fn filter_by_price(products: &HashMap<String, Product>, min: f64, max: f64) -> Vec<&Product> {
    products
        .iter()
        .filter(|(_, product)| {
            let price = product.price.parse::<f64>().unwrap();
            price >= min && price <= max
        })
        .map(|(_, product)| product)
        .collect::<Vec<&Product>>()
}
