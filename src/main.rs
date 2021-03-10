mod ast;
mod parser;
mod simplifier;

use std::fs;
use std::process::{Command, Stdio};

fn rustc_check(filename: &str) {
    // Is handling Windows required?
    let status = Command::new("rustc")
        .arg(filename)
        .arg("--out-dir")
        .arg("/tmp")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .expect("failed to run rustc check!");

    if !status.success() {
        println!("rustc check failed!");
        println!("make sure that you code is correct Rust");
        std::process::exit(1);
    }
}

fn main() {
    #[cfg(debug_assertions)]
    println!("Running a DEBUG version");

    let filename = "example.rs";
    println!("In file {}:", filename);

    rustc_check(filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let t = parser::program(&contents).expect("Ooops");
    let f = t.clone();
    let simplified = simplifier::simplify(t.1);
    println!("Left: |{}|", t.0);

    //println!("Got: |{:#?}|", simplified);
    println!("Left: |{}|", t.0);

    println!();
    println!("Before: |{}|", f.1);
    println!();
    println!("After: |{}|", simplified);

    //fs::write("./example.z3", val).expect("Unable to write file");
}
