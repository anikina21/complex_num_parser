use anyhow::anyhow;
use complex_num_parser::*;
use pest::Parser;
#[test]
fn test_number_integer() -> anyhow::Result<()> {
    let pair = Grammar::parse(Rule::number, "42")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "42");
    Ok(())
}

#[test]
fn test_number_float() -> anyhow::Result<()> {
    let pair = Grammar::parse(Rule::number, "3.14")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "3.14");
    Ok(())
}

#[test]
fn test_complex_imaginary_only() -> anyhow::Result<()> {
    let pair = Grammar::parse(Rule::complex, "4i")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "4i");
    Ok(())
}

#[test]
fn test_complex_with_plus() -> anyhow::Result<()> {
    let pair = Grammar::parse(Rule::complex, "3+4i")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "3+4i");
    Ok(())
}

#[test]
fn test_complex_with_minus() -> anyhow::Result<()> {
    let pair = Grammar::parse(Rule::complex, "5-2i")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "5-2i");
    Ok(())
}

#[test]
fn test_invalid_complex() -> anyhow::Result<()> {
    let pair = Grammar::parse(Rule::complex, "3+");
    assert!(pair.is_err());
    Ok(())
}

#[test]
fn test_sum_simple() -> anyhow::Result<()> {
    let pair = Grammar::parse(Rule::sum, "3+5")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "3+5");
    Ok(())
}

#[test]
fn test_product_simple() -> anyhow::Result<()> {
    let pair = Grammar::parse(Rule::product, "2*3")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "2*3");
    Ok(())
}

#[test]
fn test_factor_number() -> anyhow::Result<()> {
    let pair = Grammar::parse(Rule::factor, "7")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "7");
    Ok(())
}

#[test]
fn test_factor_complex() -> anyhow::Result<()> {
    let pair = Grammar::parse(Rule::factor, "1+2i")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "1+2i");
    Ok(())
}

#[test]
fn test_factor_with_parentheses() -> anyhow::Result<()> {
    let pair = Grammar::parse(Rule::factor, "(3+4i)")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "(3+4i)");
    Ok(())
}

#[test]
fn test_expression_full() -> anyhow::Result<()> {
    let expr = "(3+4i)*(1-2i)+5";
    let pair = Grammar::parse(Rule::expression, expr)?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), expr);
    Ok(())
}
