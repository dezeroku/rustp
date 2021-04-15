mod ast;
mod parser;
mod prover;
mod simplifier;
mod validator;

use clap::{App, Arg};
use env_logger::Builder;
use std::fs;
use std::process::{Command, Stdio};

extern crate log;

fn rustc_check(filename: &str) {
    let status = Command::new("rustc")
        .arg(filename)
        .arg("--out-dir")
        .arg(std::env::temp_dir())
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

fn setup_logging(level: i32, z3_debug: bool) {
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
    if !z3_debug {
        b.filter(Some("z3"), log::LevelFilter::Info);
    }
    b.init();
    log::info!("Debug level: {}", s);
}

fn args() -> (String, i32, bool, Vec<String>, bool) {
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
        .arg(
            Arg::new("z3-debug")
                .long("z3-debug")
                .takes_value(false)
                .about("Sets Z3 module logging verbosity to DEBUG"),
        )
        .arg(
            Arg::new("no-rustc-check")
                .long("no-rustc-check")
                .takes_value(false)
                .about("Don't check if the code provided is correct Rust code (highly dangerous option, use with caution)"),
        )

        .arg(
            Arg::new("function")
                .about("the function to prove (does not check for the existence)")
                .takes_value(true)
                .short('f')
                .long("function")
                .multiple(true),
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

    let z3_debug = matches.is_present("z3-debug");
    let no_rustc_check = matches.is_present("no-rustc-check");

    let mut functions = Vec::new();
    if let Some(to_prove) = matches.values_of("function") {
        for f in to_prove {
            functions.push(f.to_string());
        }
    }
    (filename, verbosity, z3_debug, functions, no_rustc_check)
}

fn validate(input: ast::Program) {
    let t = validator::validate(input);
    if !t {
        println!("Failed validation!");
        std::process::exit(4);
    }
}

fn prove(input: ast::Program, list: Vec<String>) {
    let proved = prover::prove(input, list);
    if !proved {
        println!("Failed to prove!");
        std::process::exit(5);
    }
    println!("Successfully proved the whole program!");
}

fn main() {
    #[cfg(debug_assertions)]
    println!("Running a DEBUG version");

    let (_filename, verbosity, z3_debug, functions, no_rustc_check) = args();
    let filename = &_filename.as_str();

    setup_logging(verbosity, z3_debug);

    log::info!("Checking file: {}", filename);

    if no_rustc_check {
        log::info!("Rustc check explicitly disabled via --no-rustc-check");
    } else {
        log::info!("Checking if the code is valid Rust via rustc");
        rustc_check(filename);
    }

    let tree = parse(filename);
    let simplified = simplifier::simplify(tree);

    validate(simplified.clone());
    prove(simplified.clone(), functions);
    println!();
}
