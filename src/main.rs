mod ast;
mod parser;

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
    println!("Left: |{}|", t.0);

    // OR
    println!("Got: |{:#?}|", t.1);
    println!("Left: |{}|", t.0);
    println!("Got: |{:?}|", t.1);

    // OR
    //let copy = t.1.clone();
    //for item in t.1 {
    //    println!("|{}|", item);
    //}

    //let mut temp: String = String::new();
    //for item in copy {
    //    temp += &item.to_string();
    //}

    //temp += &"(check-sat);".to_string();

    //// Only for testing
    //temp += &"(get-model);".to_string();

    //let mut val: String = String::new();
    //for item in temp.split(";") {
    //    val += &item.to_string();
    //    val += &"\n".to_string();
    //}

    //println!("{}", val);
    //fs::write("./example.z3", val).expect("Unable to write file");
}
