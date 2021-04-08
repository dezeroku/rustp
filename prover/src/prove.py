import json
import logging

from z3 import Int, Solver, IntSort, IntVector, Array, sat
from z3 import *

import rast
import parser

# Read the json data
def parse_json(filename):
    with open(filename, 'r') as data:
        a = json.loads(data.read())
    code = parser.parse_ast(a)
    return code

def main():
    code = parse_json("example.json")
    print(code)
    #root = ast.Program(a["content"])
    #a = a["content"]
    #print(a)


if __name__ == "__main__":
    main()

    x = Int('x')
    y = Int('y')
    s = Solver()

    I = IntSort()
    A = IntVector('A', 3)
    A[0] = 13
    A[1] = 13

    #s.add(x > 2, y < 10, x + 2*y == 7)
    #s.add(Select(A,0) == Select(A,1))
    s.add(not(A[0] == A[1]))

    result = s.check()
    logging.debug(result)
    if result == sat:
        print(s.model())
        print("Failed to prove, found counter example")
    else:
        print("Proved")