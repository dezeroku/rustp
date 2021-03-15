## rustp (Rust prover)

`rustp` is a tool that can be used to formally verify (prove) code that's written in subset of `Rust` language.
This project was written as part of my master's thesis (take a look at it [here](https://github.com/d0ku/master_thesis) (in Polish)).


### Specs

Currently only assignment bindings are supported, there's still no simplifier that would do the inferring for declarations.

Validation checks:
* if there is a reassignment of already defined function/variable (shadowing)


### Exit codes

* 1 - rustc check failed
* 2 - parsing failed (parsing error)
* 3 - parsing did not consume whole input (maybe just make it a warning?)
* 4 - validation failed
