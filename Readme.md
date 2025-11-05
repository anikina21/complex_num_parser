### Complex numbers parser

A parser for complex numbers; addition, subtraction, multiplication, division of complex numbers
For example(3+4i)*(1-2i)

### Example

```rust
fn main() -> anyhow::Result<()> {
    let pair = Grammar::parse(Rule::complex, "5-2i")?.next().ok_or_else(|| anyhow!("no pair"))?;

    println!("{:?}", pair);
    Ok(())
}

```