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
    let t = parser::block(&contents).expect("Ooops");
    println!("Left: |{}|", t.0);
    //println!("Got: |{}|", t.1);

    let copy = t.1.clone();
    for item in t.1 {
        println!("|{}|", item);
    }

    let mut temp: String = String::new();
    for item in copy {
        temp += &item.to_string();
    }

    temp += &"(check-sat);".to_string();

    // Only for testing
    temp += &"(get-model);".to_string();

    let mut val: String = String::new();
    for item in temp.split(";") {
        val += &item.to_string();
        val += &"\n".to_string();
    }

    println!("{}", val);
    fs::write("./example.z3", val).expect("Unable to write file");
}
