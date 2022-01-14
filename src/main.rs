pub mod demo;
pub mod search;
pub mod utils;

fn main() {
    let mut args = std::env::args();

    demo::main::execute(&mut args);
}
