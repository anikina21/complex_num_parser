use anyhow::{Context, Result};
use complex_num_parser::*;
use pest::Parser;
use std::fs;

fn parse_and_eval(input: &str) -> Result<()> {
    println!("Input: {}", input);

    let pairs = Grammar::parse(Rule::expression, input).context("Failed to parse expression")?;

    let pair = pairs
        .into_iter()
        .next()
        .ok_or_else(|| anyhow::anyhow!("No parse result"))?;

    let ast = build_ast(pair)?;
    println!("AST: {:#?}", ast);

    let result = evaluate(&ast)?;
    println!("Result: {} + {}i\n", result.re, result.im);

    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage:");
        println!("  cargo parse <expression>    - Parse and evaluate a single expression");
        println!("  cargo file <filename>       - Parse expressions from file (one per line)");
        return Ok(());
    }

    match args[1].as_str() {
        "parse" => {
            if args.len() < 3 {
                return Err(anyhow::anyhow!("Please provide an expression to parse"));
            }
            parse_and_eval(&args[2])?;
        }
        "file" => {
            if args.len() < 3 {
                return Err(anyhow::anyhow!("Please provide a filename"));
            }
            let content = fs::read_to_string(&args[2]).context("Failed to read file")?;

            for line in content.lines() {
                let line = line.trim();
                if line.is_empty() || line.starts_with('#') {
                    continue;
                }

                match parse_and_eval(line) {
                    Ok(_) => {}
                    Err(e) => eprintln!("Error parsing '{}': {}", line, e),
                }
            }
        }
        _ => {
            println!("Unknown command: {}", args[1]);
            println!("Use 'parse' or 'file'");
        }
    }

    Ok(())
}
