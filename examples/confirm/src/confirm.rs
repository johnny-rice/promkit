use promkit::preset::confirm::Confirm;

fn main() -> anyhow::Result<()> {
    let mut p = Confirm::new("Do you have a pet?").prompt()?;
    println!("result: {:?}", p.run()?);
    Ok(())
}
