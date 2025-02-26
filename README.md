Grammer as of chapter 9.
```
program        → declaration* EOF ;

declaration    → varDecl
               | statement ;

statement      → exprStmt
               | ifStmt
               | whileStmt
               | forStmt
               | printStmt
               | block ;

whileStmt     → "while" "(" expression ")" statement ;

forStmt       → "for" "(" 
                    varDeclr | exprStmt | ";"
                    expression? ";"
                    expression? 
                ")" statement ;

ifStmt        → "if" "(" expression ")" statement
               ("else" statement) ? ;

block          → "{" declaration* "}" ;

exprStmt       → expression ";" ;

printStmt      → "print" expression ";" ;

expression     → assignment ;

assignment     → IDENTIFIER "=" assignment
               | logic_or ;

logic_or       → logic_and ( "or" logic_and )* ;

logic_and      → equality ( "and" equality )* ;

equality       → comparison ( ( "!=" | "==" ) comparison )* ;

comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;

term           → factor ( ( "-" | "+" ) factor )* ;

factor         → unary ( ( "/" | "*" ) unary )* ;

unary          → ( "!" | "-" ) unary
               | primary ;

primary        → NUMBER | STRING | "true" | "false" | "nil"
               | "(" expression ")" ;
```
