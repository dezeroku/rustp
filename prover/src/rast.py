import typing

class Base:
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

class Function:
    def __init__(self, name: str, content: typing.List[Command], input:
                 typing.List[Binding], output: Type, precondition: Bool,
                 postcondition: Bool, return_value: Value):
        self.name = name
        self.content = content
        self.input = input
        self.output = output
        self.precondition = precondition
        self.postcondition = postcondition
        self.return_value = return_value

class Program(Base):
    def __init__(self, content: typing.List[Function]):
        self.content = content

class BAnd(Bool):
    def __init__(self, a: Bool, b: Bool):
        self.a = a
        self.b = b

class BOr(Bool):
    def __init__(self, a: Bool, b: Bool):
        self.a = a
        self.b = b

class BNot(Bool):
    def __init__(self, a: Bool):
        self.a = a

class BValue(Bool):
    def __init__(self, a: Value):
        self.a = a

class BTrue(Bool):
    def __init__(self):
        self.a = True

class BFalse(Bool):
    def __init__(self):
        self.a = False

class BEqual(Bool):
    def __init__(self, a: Expr, b: Expr):
        self.a = a
        self.b = b

class BGreaterThan(Bool):
    def __init__(self, a: Expr, b: Expr):
        self.a = a
        self.b = b

class BLowerThan(Bool):
    def __init__(self, a: Expr, b: Expr):
        self.a = a
        self.b = b

class BGreaterEqual(Bool):
    def __init__(self, a: Expr, b: Expr):
        self.a = a
        self.b = b

class BLowerEqual(Bool):
    def __init__(self, a: Expr, b: Expr):
        self.a = a
        self.b = b

class PAssert(ProveControl):
    def __init__(self, a: Bool):
        self.a = a

class PAssume(ProveControl):
    def __init__(self, a: Bool):
        self.a = a

class PLoopInvariant(ProveControl):
    def __init__(self, a: Bool):
        self.a = a

class TBool(Type):
    pass

class TI32(Type):
    pass

class TTuple(Type):
    def __init__(self, a: typing.List[Type]):
        self.a = a

class TReference(Type):
    def __init__(self, a: Type):
        self.a = a

class TReferenceMutable(Type):
    def __init__(self, a: Type):
        self.a = a

class TArray(Type):
    def __init__(self, a: Type, length: int):
        self.a = a
        self.length = length

class TUnit(Type):
    pass

class TUnknown(Type):
    pass

class VExpr(Value):
    def __init__(self, a: Expr):
        self.a = a

class VBool(Value):
    def __init__(self, a: Bool):
        self.a = a

class VVariable(Value):
    def __init__(self, a: Variable):
        self.a = a

class VTuple(Value):
    def __init__(self, a: typing.List[Value]):
        self.a = a

class VArray(Value):
    def __init__(self, a: typing.List[Value]):
        self.a = a

class VFunctionCall(Value):
    def __init__(self, name: str, args: typing.List[Value]):
        self.name = name
        self.args = args

class VDereference(Value):
    def __init__(self, a: Value):
        self.a = a

class VReference(Value):
    def __init__(self, a: Value):
        self.a = a

class VReferenceMutable(Value):
    def __init__(self, a: Value):
        self.a = a

class VUnit(Value):
    pass

class VUnknown(Value):
    pass

class CBinding(Command):
    def __init__(self, a: Binding):
        self.a = a

class CAssignment(Command):
    def __init__(self, a: Assignment):
        self.a = a

class CProveControl(Command):
    def __init__(self, a: ProveControl):
        self.a = a

class CBlock(Command):
    def __init__(self, a: Block):
        self.a = a

class CNoop(Command):
    pass

class ATuple(Assignment):
    def __init__(self, a: typing.List[Assignment]):
        self.a = a

class ASingle(Assignment):
    def __init__(self, var: Variable, val: Value):
        self.var = var
        self.val = val

class BDeclaration(Binding):
    def __init__(self, var: Variable, t: Type, m: bool):
        self.var = var
        self.t = t
        self.m = m

class BAssignment(Binding):
    def __init__(self, var: Variable, t: Type, m: bool, v: Value):
        self.var = var
        self.t = t
        self.m = m
        self.v = v

class BTuple(Binding):
    def __init__(self, a: typing.List[Command]):
        self.a = a

class VNamed(Variable):
    def __init__(self, name: str):
        self.name = name

class VEmpty(Variable):
    pass

class VArrayElem(Variable):
    def __init__(self, arr_name: str, index: Value):
        self.arr_name = arr_name
        self.index = index

class VTupleElem(Variable):
    def __init__(self, tup_name: str, index: Value):
        self.tup_name = tup_name
        self.index = index

class ENumber(Expr):
    def __init__(self, a: int):
        self.a = a

class EValue(Expr):
    def __init__(self, a: Value):
        self.a = a

class EOp(Expr):
    def __init__(self, a: Expr, op: Opcode, b: Expr):
        self.a = a
        self.op = op
        self.b = b

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
        self.ifs = ifs
        self.blocks = blocks
        self.el = el

class BForRange(Block):
    def __init__(self, it: Variable, first: Value, last: Value, block:
                 typing.List[Command]):
        self.it = it
        self.first = first
        self.last = last
        self.block = block

class BWhile(Block):
    def __init__(self, cond: Bool, block: typing.List[Command]):
        self.cond = cond
        self.block = block
