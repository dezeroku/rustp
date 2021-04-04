import typing

class Program:
    def __init__(self, content):
        self.content = content

class Bool:
    pass

class BAnd(Bool):
    pass

class BOr(Bool):
    pass

class BNot(Bool):
    pass

class BValue(Bool):
    pass

class BTrue(Bool):
    pass

class BFalse(Bool):
    pass

class BEqual(Bool):
    pass

class BGreaterThan(Bool):
    pass

class BLowerThan(Bool):
    pass

class BGreaterEqual(Bool):
    pass

class BLowerEqual(Bool):
    pass

class ProveControl:
    pass

class PAssert(ProveControl):
    pass

class PAssume(ProveControl):
    pass

class PLoopInvariant(ProveControl):
    pass

class Type:
    pass

class TBool(Type):
    pass

class TI32(Type):
    pass

class TTuple(Type):
    pass

class TReference(Type):
    pass

class TReferenceMutable(Type):
    pass

class TArray(Type):
    pass

class TUnit(Type):
    pass

class TUnknown(Type):
    pass

class Value:
    pass

class VExpr(Value):
    pass

class VBool(Value):
    pass

class VVariable(Value):
    pass

class VTuple(Value):
    pass

class VArray(Value):
    pass

class VFunctionCall(Value):
    pass

class VDereference(Value):
    pass

class VReference(Value):
    pass

class VReferenceMutable(Value):
    pass

class VUnit(Value):
    pass

class VUnknown(Value):
    pass

class Command:
    pass

class CBinding(Command):
    pass

class CAssignment(Command):
    pass

class CProveControl(Command):
    pass

class CBlock(Command):
    pass

class CNoop(Command):
    pass

class Assignment:
    pass

class ATuple(Assignment):
    pass

class ASingle(Assignment):
    pass

class Binding:
    pass

class BDeclaration(Binding):
    pass

class BAssignment(Binding):
    pass

class BTuple(Binding):
    pass

class Variable:
    pass

class VNamed(Variable):
    pass

class VEmpty(Variable):
    pass

class VArrayElem(Variable):
    pass

class VTupleElem(Variable):
    pass

class Expr:
    pass

class ENumber(Expr):
    pass

class EValue(Expr):
    pass

class EOp(Expr):
    pass

class Opcode:
    pass

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

class Block:
    pass

class BIf(Block):
    pass

class BForRange(Block):
    pass

class BWhile(Block):
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
