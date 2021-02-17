mod ast;
mod parser;

use std::fs;
fn main() {
    // Open a file
    // Read its content
    // Write it down in prefix notation
    let filename = "example";
    println!("In file {}:", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    parser::testo();
    //println!("With text:\n{}", contents);

    //println!("Hello, world!");
}
