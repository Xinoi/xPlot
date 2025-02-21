# Project Specifications
## Mathematical Operators:
- [+, -, *, /, ^, (, )] 
- planned for Future: [sqrt, cos, sin, exp, log, ln, Sum, Product]

## Converting Input to Graph
### Input to Vector
>Input -> String -> Vector 

### Input-Vector to Equation-Tree
>Lexer -> Parser


## Lexer (Brackets not included!)

#### Types 
PLUS: + 
MINUS: -
TIMES: *
FRAC: /
BLEFT: (
BRIGHT: )
VAR: x 
VAL: { 1 | ... | 9 }

#### EBNF
Equation :: ({Component ['+' | '-']})

Component :: [Faktor] Variable

Variable :: (x | x^(Equation) | Operation | '')

Operation :: ['*' | '/'] Variable

Faktor :: { Number }

Number :: 1 | 2 | ... | 9 | 0

## Parser
-> user *reverse polish notation (rpn)* to convert to binary tree

>[!note] RPN 
> Algorithm to convert a mathematical expression into a postfix format. 

