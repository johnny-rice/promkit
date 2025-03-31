use promkit::preset::listbox::Listbox;

fn main() -> anyhow::Result<()> {
    let mut p = Listbox::new(0..100)
        .title("What number do you like?")
        .prompt()?;
    println!("result: {:?}", p.run()?);
    Ok(())
}
