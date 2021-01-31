Format of the data that should be output by parser.

High level:
* function
* import


definition is the reference to a variable, there should be only one definition per variable

function:
* name
* list of commands
    * command type
        * assignment
            * name
            * value
        * definition (`binding` in Rust's doc)
            * name
            * mutable
            * value (does not have to be provided)
            * type (pointer, address, value (has a specific type too))
        * algebraic
            * type
                * addition
                * subtraction
                * multiplication
                * division
                * modulo
                * value (points to a definition)
            * arguments
                * first algebraic
                * second algebraic
        * special
            * break (does not make sense outside of a loop)
            * continue (can it be proven at all?)
        * function call
            * function name
            * list of arguments (definitions)
        * block
            * condition
                * logic expression
                * block of commands true
                * block of commands false
            * loop
                * for
                    * logic expression
                    * block of commands
                * while
                    * logic expression
                    * block of commands
                * loop (basically a `while true` equivalent)
                    * block of commands
