# Project Specifications
## Mathematical Operators:
- [+, -, *, /, ^, (, )] 
- planned for Future: [sqrt, cos, sin, exp, log, ln, Sum, Product]

## Converting Input to Graph
### Input to Vector
>Input -> String -> Vector 

### Input-Vector to Equation-Tree
- Lexer -> Parser


## Lexer (Brackets not included!)
Equation :: ({Component ['+' | '-']})
Component :: [Faktor] Variable
Variable :: (x | x^(Equation) | Operation | '')
Operation :: ['*' | '/'] Variable
Faktor :: { Number }
Number :: 1 | 2 | ... | 9 | 0
