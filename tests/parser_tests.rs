use complex_num_parser::*;
use num_complex::Complex;
use pest::Parser;

#[test]
fn test_parse_number() {
    let expr = build_ast(
        Grammar::parse(Rule::expression, "42")
            .unwrap()
            .next()
            .unwrap(),
    )
    .unwrap();
    let result = evaluate(&expr).unwrap();
    assert_eq!(result, Complex::new(42.0, 0.0));

    let expr = build_ast(
        Grammar::parse(Rule::expression, "3.14")
            .unwrap()
            .next()
            .unwrap(),
    )
    .unwrap();
    let result = evaluate(&expr).unwrap();
    assert_eq!(result, Complex::new(3.14, 0.0));
}

#[test]
fn test_parse_complex_numbers() {
    let expr = build_ast(
        Grammar::parse(Rule::expression, "3+4i")
            .unwrap()
            .next()
            .unwrap(),
    )
    .unwrap();
    let result = evaluate(&expr).unwrap();
    assert_eq!(result, Complex::new(3.0, 4.0));

    let expr = build_ast(
        Grammar::parse(Rule::expression, "-2i")
            .unwrap()
            .next()
            .unwrap(),
    )
    .unwrap();
    let result = evaluate(&expr).unwrap();
    assert_eq!(result, Complex::new(0.0, -2.0));

    let expr = build_ast(
        Grammar::parse(Rule::expression, "5i")
            .unwrap()
            .next()
            .unwrap(),
    )
    .unwrap();
    let result = evaluate(&expr).unwrap();
    assert_eq!(result, Complex::new(0.0, 5.0));

    let expr = build_ast(
        Grammar::parse(Rule::expression, "0+1i")
            .unwrap()
            .next()
            .unwrap(),
    )
    .unwrap();
    let result = evaluate(&expr).unwrap();
    assert_eq!(result, Complex::new(0.0, 1.0));
}

#[test]
fn test_arithmetic_expressions() {
    let expr = build_ast(
        Grammar::parse(Rule::expression, "3 + 4i + 1 - 2i")
            .unwrap()
            .next()
            .unwrap(),
    )
    .unwrap();
    let result = evaluate(&expr).unwrap();
    assert_eq!(result, Complex::new(4.0, 2.0));

    let expr = build_ast(
        Grammar::parse(Rule::expression, "2 * 3 + 4i")
            .unwrap()
            .next()
            .unwrap(),
    )
    .unwrap();
    let result = evaluate(&expr).unwrap();
    assert_eq!(result, Complex::new(6.0, 4.0));

    let expr = build_ast(
        Grammar::parse(Rule::expression, "(1+2i) * (3-4i)")
            .unwrap()
            .next()
            .unwrap(),
    )
    .unwrap();
    let result = evaluate(&expr).unwrap();
    assert_eq!(result, Complex::new(11.0, 2.0));
}

#[test]
fn test_division_and_errors() {
    let expr = build_ast(
        Grammar::parse(Rule::expression, "4 / 2")
            .unwrap()
            .next()
            .unwrap(),
    )
    .unwrap();
    let result = evaluate(&expr).unwrap();
    assert_eq!(result, Complex::new(2.0, 0.0));

    let expr = build_ast(
        Grammar::parse(Rule::expression, "1 / 0")
            .unwrap()
            .next()
            .unwrap(),
    )
    .unwrap();
    assert!(evaluate(&expr).is_err());
}

#[test]
fn test_invalid_complex_format() {
    let parse_result = Grammar::parse(Rule::expression, "3 + 4j");
    assert!(parse_result.is_err());

    let parse_result = Grammar::parse(Rule::expression, "i+3");
    assert!(parse_result.is_err());
}
