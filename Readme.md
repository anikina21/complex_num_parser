# complex-num-parser

**Project name:** complex-num-parser

**Brief description:**
A parser and evaluator for arithmetic expressions with complex numbers. The parser recognizes real numbers (integer and float), imaginary numbers (with trailing `i`), the four basic operators `+ - * /`, and parentheses. It produces an AST which is evaluated to a `num_complex::Complex<f64>` result.\

Програма парсить і обчислює вирази з комплексними числами (наприклад, `(3+4i)*(1-2i)`).

## Technical description of parsing process
The parser uses the Pest parsing library to convert an input string (such as `(3+4i)*(1-2i)`) into a structured parse tree according to grammar rules defined in `grammar.pest`.

After parsing, the program builds an Abstract Syntax Tree (AST) from the Pest `Pair` iterator. This AST is then evaluated using the `num-complex` crate to obtain the resulting complex number.

### Example of internal parsing and evaluation process
Input: `(3+4i)*(1-2i)`

1. Parsing phase: The string is matched against grammar rules. Pest produces a parse tree with nested rules.
2. AST construction: The parse tree is transformed into nested `Expr` nodes representing numbers, complex numbers, and binary operations.
3. Evaluation: The AST is recursively evaluated using the appropriate arithmetic operators. The final result is a `Complex<f64>` number.

Result: `11 - 2i`

### CLI usage examples
After building the project (`cargo build`), you can use the command-line interface:

```bash
# Parse and evaluate a single expression
$ cargo run -- parse "(3+4i)*(1-2i)"
AST: BinaryOp { op: '*', left: Complex(3+4i), right: Complex(1-2i) }
Result: 11 + -2i

# Parse and evaluate expressions from a file (each line = one expression)
$ cargo run -- file examples.txt
(3+4i)*(1-2i) -> 11 + -2i
2+3*4 -> 14 + 0i

# Show help
$ cargo run -- help
Complex Number Parser CLI
Usage:
  cargo run -- parse <expression>    - Parse and evaluate a single expression
  cargo run -- file <filename>       - Parse expressions from file (one per line)
  cargo run -- help                  - Show help message
  cargo run -- credits               - Show authors and acknowledgements

```

## Grammar Rules

```rust
// Real numbers
number = @{ ASCII_DIGIT+ ~ ("." ~ ASCII_DIGIT+)? }

// Complex numbers: handles "3+4i", "5i", "-2i", "3-4i"
complex = @{
    number ~ ("+" | "-") ~ number ~ "i" |
    ("+" | "-")? ~ number ~ "i"
}

// Arithmetic rules
sum     = { product ~ ((add_op | sub_op) ~ product)* }
product = { factor ~ ((mul_op | div_op) ~ factor)* }
factor  = { "(" ~ sum ~ ")" | complex | number }

// Operator tokens
add_op = { "+" }
sub_op = { "-" }
mul_op = { "*" }
div_op = { "/" }

// Entry rule
expression = { SOI ~ sum ~ EOI }

WHITESPACE = _{ " " | "\t" | "\n" | "\r" }
```

## How results are used

The parsed expressions are transformed into a numeric representation (`Complex<f64>`), enabling arithmetic operations, scientific calculations, or integration into larger mathematical software.
