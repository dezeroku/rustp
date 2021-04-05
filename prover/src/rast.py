import typing

class Base(dict):
    pass

class Bool(Base):
    pass

class Expr(Base):
    pass

class Variable(Base):
    pass

class Type(Base):
    pass

class Opcode(Base):
    pass

class Value(Base):
    pass

class Assignment(Base):
    pass

class Command(Base):
    pass

class Binding(Base):
    pass

class Block(Base):
    pass

class ProveControl(Base):
    pass

class Function(Base):
    def __init__(self, name: str, content: typing.List[Command], input:
                 typing.List[Binding], output: Type, precondition: Bool,
                 postcondition: Bool, return_value: Value):
        dict.__init__(self, name=name, content=content, input=input,
                      output=output, precondition=precondition,
                      postcondition=postcondition, return_value=return_value)

class Program(Base):
    def __init__(self, content: typing.List[Function]):
        dict.__init__(self, content=content)

# TODO: convert all __init__s to use dict.__init__ so it's easily serializable
# and debug readable
class BAnd(Bool):
    def __init__(self, a: Bool, b: Bool):
        dict.__init__(self, a=a, b=b)

class BOr(Bool):
    def __init__(self, a: Bool, b: Bool):
        dict.__init__(self, a=a, b=b)

class BNot(Bool):
    def __init__(self, a: Bool):
        dict.__init__(self, a=a)

class BValue(Bool):
    def __init__(self, a: Value):
        dict.__init__(self, a=a)

class BTrue(Bool):
    def __init__(self):
        dict.__init__(self, a=True)

class BFalse(Bool):
    def __init__(self):
        dict.__init__(self, a=False)

class BEqual(Bool):
    def __init__(self, a: Expr, b: Expr):
        dict.__init__(self, a=a, b=b)

class BGreaterThan(Bool):
    def __init__(self, a: Expr, b: Expr):
        dict.__init__(self, a=a, b=b)

class BLowerThan(Bool):
    def __init__(self, a: Expr, b: Expr):
        dict.__init__(self, a=a, b=b)

class BGreaterEqual(Bool):
    def __init__(self, a: Expr, b: Expr):
        dict.__init__(self, a=a, b=b)

class BLowerEqual(Bool):
    def __init__(self, a: Expr, b: Expr):
        dict.__init__(self, a=a, b=b)

class PAssert(ProveControl):
    def __init__(self, a: Bool):
        dict.__init__(self, a=a)

class PAssume(ProveControl):
    def __init__(self, a: Bool):
        dict.__init__(self, a=a)

class PLoopInvariant(ProveControl):
    def __init__(self, a: Bool):
        dict.__init__(self, a=a)

class TBool(Type):
    pass

class TI32(Type):
    pass

class TTuple(Type):
    def __init__(self, a: typing.List[Type]):
        dict.__init__(self, a=a)

class TReference(Type):
    def __init__(self, a: Type):
        dict.__init__(self, a=a)

class TReferenceMutable(Type):
    def __init__(self, a: Type):
        dict.__init__(self, a=a)

class TArray(Type):
    def __init__(self, a: Type, length: int):
        dict.__init__(self, a=a, length=length)

class TUnit(Type):
    pass

class TUnknown(Type):
    pass

class VExpr(Value):
    def __init__(self, a: Expr):
        dict.__init__(self, a=a)

class VBool(Value):
    def __init__(self, a: Bool):
        dict.__init__(self, a=a)

class VVariable(Value):
    def __init__(self, a: Variable):
        dict.__init__(self, a=a)

class VTuple(Value):
    def __init__(self, a: typing.List[Value]):
        dict.__init__(self, a=a)

class VArray(Value):
    def __init__(self, a: typing.List[Value]):
        dict.__init__(self, a=a)

class VFunctionCall(Value):
    def __init__(self, name: str, args: typing.List[Value]):
        dict.__init__(self, name=name, args=args)

class VDereference(Value):
    def __init__(self, a: Value):
        dict.__init__(self, a=a)

class VReference(Value):
    def __init__(self, a: Value):
        dict.__init__(self, a=a)

class VReferenceMutable(Value):
    def __init__(self, a: Value):
        dict.__init__(self, a=a)

class VUnit(Value):
    pass

class VUnknown(Value):
    pass

class CBinding(Command):
    def __init__(self, a: Binding):
        dict.__init__(self, a=a)

class CAssignment(Command):
    def __init__(self, a: Assignment):
        dict.__init__(self, a=a)

class CProveControl(Command):
    def __init__(self, a: ProveControl):
        dict.__init__(self, a=a)

class CBlock(Command):
    def __init__(self, a: Block):
        dict.__init__(self, a=a)

class CNoop(Command):
    pass

class ATuple(Assignment):
    def __init__(self, a: typing.List[Assignment]):
        dict.__init__(self, a=a)

class ASingle(Assignment):
    def __init__(self, var: Variable, val: Value):
        dict.__init__(self, var=var, val=val)

class BDeclaration(Binding):
    def __init__(self, var: Variable, t: Type, m: bool):
        dict.__init__(self, var=var, t=t, m=m)

class BAssignment(Binding):
    def __init__(self, var: Variable, t: Type, m: bool, v: Value):
        dict.__init__(self, var=var, t=t, m=m, v=v)

class BTuple(Binding):
    def __init__(self, a: typing.List[Command]):
        dict.__init__(self, a=a)

class VNamed(Variable):
    def __init__(self, name: str):
        dict.__init__(self, name=name)

class VEmpty(Variable):
    pass

class VArrayElem(Variable):
    def __init__(self, arr_name: str, index: Value):
        dict.__init__(self, arr_name=arr_name, index=index)

class VTupleElem(Variable):
    def __init__(self, tup_name: str, index: Value):
        dict.__init__(self, tup_name=tup_name, index=index)

class ENumber(Expr):
    def __init__(self, a: int):
        dict.__init__(self, a=a)

class EValue(Expr):
    def __init__(self, a: Value):
        dict.__init__(self, a=a)

class EOp(Expr):
    def __init__(self, a: Expr, op: Opcode, b: Expr):
        dict.__init__(self, a=a, op=op, b=b)

class OMul(Opcode):
    pass

class ODiv(Opcode):
    pass

class OAdd(Opcode):
    pass

class OSub(Opcode):
    pass

class ORem(Opcode):
    pass

class BIf(Block):
    def __init__(self, ifs: typing.List[Bool], blocks:
                 typing.List[typing.List[Command]],
                 el: typing.List[Command]):
        dict.__init__(self, ifs=ifs, blocks=blocks, el=el)

class BForRange(Block):
    def __init__(self, it: Variable, first: Value, last: Value, block:
                 typing.List[Command]):
        dict.__init__(self, it=it, first=first, last=last, block=block)

class BWhile(Block):
    def __init__(self, cond: Bool, block: typing.List[Command]):
        dict.__init__(self, cond=cond, block=block)
