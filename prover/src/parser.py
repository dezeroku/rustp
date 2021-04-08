import rast
import json

def parse_ast(a: dict) -> rast.Program:
    """Convert the input JSON content into proper AST."""
    # TODO: implement
    return get_root(a)

def get_root(a: dict) -> rast.Program:
    _funcs = a["content"]
    funcs = []
    for _func in _funcs:
        funcs.append(get_func(_func))

    t = rast.Program(funcs)
    return t

def get_func(a: dict) -> rast.Function:
    name = a["name"]
    #content = a["content"]
    content = ""
    #input = a["input"]
    input = ""
    output = get_type(a["output"])
    precondition = a["precondition"]
    postcondition = a["postcondition"]
    return_value = a["return_value"]

    return rast.Function(name, content, input, output, precondition,
                         postcondition, return_value)


def get_type(a) -> rast.Type:
    if a == "Unit":
        return rast.TUnit()
    elif a == "Unknown":
        return rast.TUnknown()
    elif a == "I32":
        return rast.TI32()
    elif a == "Bool":
        return rast.TBool()

    return
    # TODO:
    # Tuple
    # Array
    # Reference
    # ReferenceMut
