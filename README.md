## rustp (Rust prover)

`rustp` is a tool that can be used to formally verify (prove) code that's written in subset of `Rust` language.
This project was written as part of my master's thesis (take a look at it [here](https://github.com/d0ku/master_thesis) (in Polish)).

### Exit codes

* 1 - rustc check failed
* 2 - parsing failed (parsing error)
* 3 - parsing did not consume whole input (maybe just make it a warning?)
