mod calc;
fn main()-> anyhow::Result<()> {
    calc::Calc::default().run()?;
    Ok(())
}
