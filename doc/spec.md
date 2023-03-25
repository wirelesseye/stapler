# The staple language specification

## Program
```
program -> stmt*
```

## Statement
```
stmt -> decl
      | def
      | expr
      | import-stmt
      | extern-stmt
import-stmt -> "import" ...
extern-stmt -> "extern" decl | extern-block
extern-block -> "{" decl* "}"
```

## Declearation & Definition
```
decl -> "let" decl-body ("," decl-body)*
decl-body -> IDENT (":" type)? ("=" expr)?
def -> def-lhs ("=" expr)+
def-lhs -> IDENT ("." IDENT)*
fun-literal -> param-list "->" type? block
fun-proto -> param-list "->" type
```

## Type
```
type -> PRIMITIVE_TYPES | ref-type | func-type
func-type -> param-list "->" type
```

## Expression
```
block -> "{" stmt* "}"
expr -> func-call
      | fun-literal
      | IDENT
      | INTLITERAL
      | FLOATLITERAL
      | BOOLLITERAL
      | STRINGLITERAL
func-call -> IDENT arg-list
```

## Parameters
```
param-list -> "(" proper-param-list? ")"
proper-param-list -> param ("," expr)*
param -> IDENT ":" type

arg-list -> "(" proper-arg-list? ")"
proper-arg-list -> expr ("," expr)*
```