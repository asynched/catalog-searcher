use crate::search;
use crate::search::core::data::Product;
use crate::utils::io::input;

use std::collections::HashMap;
use std::env::Args;
use std::time::Instant;

/// Executes the demo application
///
/// # Example
///
/// ```rs
/// let mut args = std::env::args();
/// demo::main::execute(&mut args);
/// ```
pub fn execute(args: &mut Args) {
    let filename = args
        .nth(1)
        .expect("A filename must be provided in order to run the program");

    let now = Instant::now();
    let products = search::core::main::index_products(&filename);
    let elapsed = now.elapsed().as_micros();

    println!("[INFO]: Rust took {:?}us to parse the source file", elapsed);

    main_loop(products);
}

/// Executes the main loop, asking for maximum and minimum price for the products
///
/// # Example
///
/// ```rs
/// demo::main::main_loop(HashMap::new::<String, Product>());
/// ```
fn main_loop(products: HashMap<String, Product>) {
    loop {
        let min = input("Type in the minimum price: ");
        let max = input("Type in the maximum price: ");

        let min = min.parse::<f64>().expect(
            "Couldn't parse minimum price to a float, make sure that you're passing a valid value",
        );
        let max = max.parse::<f64>().expect(
            "Couldn't parse maximum price to a float, make sure that you're passing a valid value",
        );

        let now = Instant::now();
        let found = search::core::filters::filter_by_price(&products, min, max);
        let elapsed = now.elapsed().as_micros();

        found.iter().for_each(|product| {
            println!("#{} - {} - {}", product.id, product.name, product.price);
        });

        println!("[INFO]: Query took {:?}us to execute\n", elapsed);
    }
}
