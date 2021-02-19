mod ast;
mod parser;

use std::fs;

fn main() {
    #[cfg(debug_assertions)]
    println!("Running a DEBUG version");

    // Open a file
    // Read its content
    // Write it down in prefix notation
    let filename = "example";
    println!("In file {}:", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let t = parser::expr(&contents).expect("Ooops");
    //println!("With text:\n{}", contents);
    println!("Left: |{}|, Got: |{}|", t.0, t.1);
}
