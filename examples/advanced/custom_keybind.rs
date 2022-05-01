use std::io;

use crossterm::{
    self,
    event::{Event, KeyCode, KeyEvent, KeyModifiers},
};

use promptio::{
    build::Builder,
    grapheme::Graphemes,
    keybind::KeyBind,
    readline::{self, State},
    EventHandleFn, Result,
};

fn main() -> Result<()> {
    let mut b = KeyBind::default();
    b.assign(vec![(
        Event::Key(KeyEvent {
            code: KeyCode::Char('r'),
            modifiers: KeyModifiers::CONTROL,
        }),
        Box::new(|_, _, _: &mut io::Stdout, state: &mut State| {
            let prev = state.0.editor.clone();
            state.0.editor.replace(&Graphemes::from("REPLCED!!"));
            state.0.input_stream.push((prev, state.0.editor.clone()));
            Ok(None)
        }) as Box<EventHandleFn<State>>,
    )]);
    let mut p = readline::Builder::default().handler(b).build()?;
    loop {
        let (line, exit_code) = p.run()?;
        match exit_code {
            0 => println!("result: {:?}", line),
            1 => return Ok(()),
            _ => (),
        }
    }
}
