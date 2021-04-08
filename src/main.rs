mod ast;
mod context;
mod parser;
mod prover;
mod simplifier;
mod validator;

use clap::{App, Arg};
use env_logger::Builder;
use serde_json::Result;
use std::fs;
use std::process::{Command, Stdio};

extern crate log;

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

fn parse(filename: &str) -> ast::Program {
    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let t = parser::program(&content);
    match t {
        Ok((rest, tree)) => {
            if rest == "" {
                tree
            } else {
                println!("Not the whole input was parsed.");
                println!("The part that left is:\n{}", rest);
                std::process::exit(3);
            }
        }
        Err(nom::Err::Error(a)) => {
            println!("Failed to parse!");
            println!("Tag: {:?}", a.code);
            println!("Unmatched input:\n {}", a.input);
            std::process::exit(2);
        }
        Err(nom::Err::Failure(a)) => {
            println!("Failed to parse!");
            println!("Tag: {:?}", a.code);
            println!("Unmatched input:\n {}", a.input);
            std::process::exit(2);
        }
        Err(nom::Err::Incomplete(_)) => {
            // This case shouldn't happen at all, we're working with files, so the whole context is known
            println!("Failed to parse!");
            println!("Not enough data!");
            std::process::exit(2);
        }
    }
}

fn setup_logging(level: i32) {
    let s = match level {
        0 => log::LevelFilter::Off,
        1 => log::LevelFilter::Error,
        2 => log::LevelFilter::Warn,
        3 => log::LevelFilter::Info,
        4 => log::LevelFilter::Debug,
        5 => log::LevelFilter::Trace,
        _ => log::LevelFilter::max(),
    };
    let mut b = Builder::new();
    b.filter_level(s);
    if level < 5 {
        b.filter(Some("z3"), log::LevelFilter::Info);
    }
    b.init();
    log::info!("Debug level: {}", s);
}

fn args() -> (String, i32) {
    let matches = App::new("rustp")
        .version("1.0")
        .author("d0ku <darthtyranus666666@gmail.com>")
        .about("Verify formal corectness of programs written in language based on Rust")
        .arg(
            Arg::new("INPUT")
                .about("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("v")
                .short('v')
                .multiple(true)
                .takes_value(false)
                .about("Sets the level of verbosity"),
        )
        .get_matches();

    let filename = matches.value_of("INPUT").unwrap().to_string();

    let verbosity = match matches.occurrences_of("v") {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        4 => 4,
        5 => 5,
        6 | _ => 6,
    };

    (filename, verbosity)
}

fn validate(input: ast::Program) {
    let t = validator::validate(input);
    if !t {
        println!("Failed validation!");
        std::process::exit(4);
    }
}

fn prove(input: ast::Program) {
    let proved = prover::prove(input);
    if !proved {
        println!("Failed to prove!");
        std::process::exit(5);
    }
    println!("Successfully proved the whole program!");
}

fn main() {
    #[cfg(debug_assertions)]
    println!("Running a DEBUG version");

    let (_filename, verbosity) = args();
    let filename = &_filename.as_str();

    setup_logging(verbosity);

    log::info!("Checking file: {}", filename);

    rustc_check(filename);

    let tree = parse(filename);
    let f = tree.clone();
    let simplified = simplifier::simplify(tree);

    // We assume that at the stage of proving, all the types are already solved.
    // If that's not the case, something is wrong in our inferring
    // TODO: add a check for this
    //if verbosity >= 2 {
    //    println!();
    //    println!("Before: |{}|", f);
    //    println!();
    //    println!("After: |{}|", simplified);
    //}

    // Move this part to python for now
    validate(simplified.clone());

    // Drop the content to a file as JSON
    //let j = serde_json::to_string(&simplified).unwrap();
    //fs::write("./example.json", j).expect("Unable to write temp AST file!");
    //log::debug!("Saved the temp AST data as example.json");

    prove(simplified.clone());
    println!();
    //if verbosity >= 2 {
    //    for func in simplified.content.clone() {
    //        for i in context::get_context_func(func, simplified.clone()) {
    //            println!("{:?}", i);
    //            println!();
    //        }
    //    }
    //}
    //fs::write("./example.z3", val).expect("Unable to write file");
}
