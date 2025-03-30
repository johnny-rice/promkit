use promkit::{crossterm::style::Color, preset::form::Form, style::StyleBuilder, promkit_widgets::text_editor};

fn main() -> anyhow::Result<()> {
    let mut p = Form::new([
        text_editor::State {
            texteditor: Default::default(),
            history: Default::default(),
            prefix: String::from("❯❯ "),
            mask: Default::default(),
            prefix_style: StyleBuilder::new().fgc(Color::DarkRed).build(),
            active_char_style: StyleBuilder::new().bgc(Color::DarkCyan).build(),
            inactive_char_style: StyleBuilder::new().build(),
            edit_mode: Default::default(),
            word_break_chars: Default::default(),
            lines: Default::default(),
        },
        text_editor::State {
            texteditor: Default::default(),
            history: Default::default(),
            prefix: String::from("❯❯ "),
            mask: Default::default(),
            prefix_style: StyleBuilder::new().fgc(Color::DarkGreen).build(),
            active_char_style: StyleBuilder::new().bgc(Color::DarkCyan).build(),
            inactive_char_style: StyleBuilder::new().build(),
            edit_mode: Default::default(),
            word_break_chars: Default::default(),
            lines: Default::default(),
        },
        text_editor::State {
            texteditor: Default::default(),
            history: Default::default(),
            prefix: String::from("❯❯ "),
            mask: Default::default(),
            prefix_style: StyleBuilder::new().fgc(Color::DarkBlue).build(),
            active_char_style: StyleBuilder::new().bgc(Color::DarkCyan).build(),
            inactive_char_style: StyleBuilder::new().build(),
            edit_mode: Default::default(),
            word_break_chars: Default::default(),
            lines: Default::default(),
        },
    ])
    .prompt()?;
    println!("result: {:?}", p.run()?);
    Ok(())
}
