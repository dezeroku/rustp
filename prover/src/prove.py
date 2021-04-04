import json
import logging
import rast

#import ast
from z3 import Int, Solver, IntSort, IntVector, Array, sat
from z3 import *

# Read the json data
def parse_json(filename):
    with open(filename, 'r') as data:
        a = json.loads(data.read())
    return a

def main():
    a = parse_json("example.json")
    #print(a)
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
