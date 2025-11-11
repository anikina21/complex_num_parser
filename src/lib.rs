use anyhow::{Result, anyhow};
use num_complex::Complex;
use pest::iterators::Pair;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct Grammar;

// AST Definition
#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64),
    Complex(Complex<f64>),
    BinaryOp {
        op: char,
        left: Box<Expr>,
        right: Box<Expr>,
    },
}

fn parse_number(s: &str) -> Result<f64> {
    s.parse::<f64>()
        .map_err(|e| anyhow!("Failed to parse number: {}", e))
}

pub fn build_ast(pair: Pair<Rule>) -> Result<Expr> {
    match pair.as_rule() {
        Rule::expression => {
            let inner = pair
                .into_inner()
                .next()
                .ok_or_else(|| anyhow!("Empty expression"))?;
            build_ast(inner)
        }
        Rule::sum => {
            let mut pairs = pair.into_inner();
            let mut expr = build_ast(pairs.next().ok_or_else(|| anyhow!("Expected product"))?)?;

            while let Some(next) = pairs.next() {
                let op = match next.as_rule() {
                    Rule::add_op => '+',
                    Rule::sub_op => '-',
                    _ => return Err(anyhow!("Expected operator, got {:?}", next.as_rule())),
                };
                let right = build_ast(
                    pairs
                        .next()
                        .ok_or_else(|| anyhow!("Expected right operand after {}", op))?,
                )?;
                expr = Expr::BinaryOp {
                    op,
                    left: Box::new(expr),
                    right: Box::new(right),
                };
            }
            Ok(expr)
        }
        Rule::product => {
            let mut pairs = pair.into_inner();
            let mut expr = build_ast(pairs.next().ok_or_else(|| anyhow!("Expected factor"))?)?;

            while let Some(next) = pairs.next() {
                let op = match next.as_rule() {
                    Rule::mul_op => '*',
                    Rule::div_op => '/',
                    _ => return Err(anyhow!("Expected operator, got {:?}", next.as_rule())),
                };
                let right = build_ast(
                    pairs
                        .next()
                        .ok_or_else(|| anyhow!("Expected right operand after {}", op))?,
                )?;
                expr = Expr::BinaryOp {
                    op,
                    left: Box::new(expr),
                    right: Box::new(right),
                };
            }
            Ok(expr)
        }
        Rule::factor => {
            let inner = pair
                .into_inner()
                .next()
                .ok_or_else(|| anyhow!("Empty factor"))?;
            build_ast(inner)
        }
        Rule::number => {
            let num = parse_number(pair.as_str())?;
            Ok(Expr::Number(num))
        }
        Rule::complex => parse_complex(pair.as_str()),
        _ => Err(anyhow!("Unexpected rule: {:?}", pair.as_rule())),
    }
}

fn parse_complex(s: &str) -> Result<Expr> {
    let s = s.trim();

    if s.ends_with('i') && !s.contains('+') {
        let minus_count = s.chars().filter(|&c| c == '-').count();
        if minus_count == 0 || (minus_count == 1 && s.starts_with('-')) {
            let num_part = &s[..s.len() - 1];
            if num_part.is_empty() || num_part == "+" {
                return Ok(Expr::Complex(Complex::new(0.0, 1.0)));
            }
            if num_part == "-" {
                return Ok(Expr::Complex(Complex::new(0.0, -1.0)));
            }
            let im = parse_number(num_part)?;
            return Ok(Expr::Complex(Complex::new(0.0, im)));
        }
    }

    if !s.ends_with('i') {
        return Err(anyhow!("Complex number must end with 'i': {}", s));
    }

    let without_i = &s[..s.len() - 1];
    let chars: Vec<char> = without_i.chars().collect();

    let mut split_pos = None;
    for i in (1..chars.len()).rev() {
        if chars[i] == '+' || chars[i] == '-' {
            split_pos = Some(i);
            break;
        }
    }

    if let Some(pos) = split_pos {
        let real_part = &without_i[..pos];
        let imag_part_with_sign = &without_i[pos..];

        let re = if real_part.is_empty() {
            0.0
        } else {
            parse_number(real_part)?
        };

        let im = if imag_part_with_sign == "+" {
            1.0
        } else if imag_part_with_sign == "-" {
            -1.0
        } else {
            let sign = if imag_part_with_sign.starts_with('-') {
                -1.0
            } else {
                1.0
            };
            let digits = &imag_part_with_sign[1..];
            if digits.is_empty() {
                sign
            } else {
                sign * parse_number(digits)?
            }
        };

        Ok(Expr::Complex(Complex::new(re, im)))
    } else {
        Err(anyhow!("Invalid complex number format: {}", s))
    }
}

pub fn evaluate(expr: &Expr) -> Result<Complex<f64>> {
    match expr {
        Expr::Number(n) => Ok(Complex::new(*n, 0.0)),
        Expr::Complex(c) => Ok(*c),
        Expr::BinaryOp { op, left, right } => {
            let left_val = evaluate(left)?;
            let right_val = evaluate(right)?;

            match op {
                '+' => Ok(left_val + right_val),
                '-' => Ok(left_val - right_val),
                '*' => Ok(left_val * right_val),
                '/' => {
                    if right_val.norm() < 1e-10 {
                        Err(anyhow!("Division by zero"))
                    } else {
                        Ok(left_val / right_val)
                    }
                }
                _ => Err(anyhow!("Unknown operator: {}", op)),
            }
        }
    }
}
