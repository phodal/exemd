mod commands;

use commands::calc;

fn main() {
    println!("Hello, world!");
    let result = calc::add(1, 2);
    println!("{}", result);
}