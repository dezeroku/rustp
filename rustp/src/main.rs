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
    let t = parser::command(&contents).expect("Ooops");
    println!("Left: |{}|", t.0);
    println!("Got: |{}|", t.1);

    //for item in t.1 {
    //    println!("{}", item);
    //}
}
