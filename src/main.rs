use pest::Parser;
use complex_num_parser::*;
use anyhow::anyhow;

fn main() -> anyhow::Result<()> {
    let pair = Grammar::parse(Rule::complex, "5-2i")?.next().ok_or_else(|| anyhow!("no pair"))?;

    //println!("{}", pair.as_str());
    println!("{:?}", pair);
    Ok(())
}
