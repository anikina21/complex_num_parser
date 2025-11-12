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
        print_help();
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

        "help" => {
            print_help();
        }

        "credits" => {
            print_credits();
        }

        _ => {
            println!("Unknown command: '{}'\n", args[1]);
            print_help();
        }
    }

    Ok(())
}

fn print_help() {
    println!("Complex Number Parser CLI");
    println!("Usage:");
    println!("  cargo run -- parse <expression>     - Parse and evaluate a single expression");
    println!("  cargo run -- file <filename>        - Parse expressions from a file (one per line)");
    println!("  cargo run -- help                   - Show help message");
    println!("  cargo run -- credits                - Show authors and acknowledgements");
    println!();
    println!("Examples:");
    println!("  cargo run -- parse \"(2+3i)*(4-5i)\"");
    println!("  cargo run -- file examples.txt");
}

fn print_credits() {
    println!("--Complex Number Parser--");
    println!("Developed by Anikina Anastasiia");
    println!("Based on the pest parser and complex number evaluation module.");
    println!("(c)2025");
}