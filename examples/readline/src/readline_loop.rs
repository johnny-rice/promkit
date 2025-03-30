use promkit::preset::readline::Readline;

fn main() -> anyhow::Result<()> {
    let mut p = Readline::default().prompt()?;

    loop {
        match p.run() {
            Ok(cmd) => {
                println!("result: {:?}", cmd);
            }
            Err(_) => {
                println!("Bye!");
                break;
            }
        }
    }
    Ok(())
}
