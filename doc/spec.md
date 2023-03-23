# The staple language specification

## Program
`program -> stmt*`

## Statement
`stmt -> decl | def | (expr ";") | import-stmt | extern-stmt`

`import-stmt -> "import" ... ";"`

`extern-stmt -> extern decl | block`

## Declearation & Definition
`decl -> "let" id (":" type)? def-body? ";"`

`def -> def-lhs ("=" def-rhs)+ ";"`

`def-lhs -> id ("." id)*`

`def-rhs -> expr | fun-literal`

`fun-literal -> param-list "->" type? block ";"`

## Type
`type -> pri-type | ref-type`

`fun-proto -> param-list "->" type`

## Expression
`block -> "{" stmt* "}" `

`expr -> `*ID* `|` *INTLITERAL* `|` *FLOATLITERAL* `|` *BOOLLITERAL* `|` *STRINGLITERAL*

## Parameters
`param-list -> "(" proper-para-list? ")"`