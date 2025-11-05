use pest::Parser;
use anyhow::anyhow;
use complex_num_parser::*;

#[test]
fn test_imaginary_only() -> anyhow::Result<()> {
    let pair = Grammar::parse(Rule::complex, "4i")?.next().ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "4i");
    Ok(())
}

#[test]
fn test_complex_with_plus() -> anyhow::Result<()> {
    let pair = Grammar::parse(Rule::complex, "3+4i")?.next().ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "3+4i");
    Ok(())
}

#[test]
fn test_complex_with_minus() -> anyhow::Result<()> {
    let pair = Grammar::parse(Rule::complex, "5-2i")?.next().ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "5-2i");
    Ok(())
}

#[test]
fn test_invalid_input() -> anyhow::Result<()> {
    let pair = Grammar::parse(Rule::complex, "3+") ;
    assert!(pair.is_err());
    Ok(())
}