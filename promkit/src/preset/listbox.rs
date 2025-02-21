use std::{cell::RefCell, fmt::Display, io};

use crate::{
    crossterm::style::{Attribute, Attributes, Color, ContentStyle},
    listbox,
    style::StyleBuilder,
    switch::ActiveKeySwitcher,
    text::{self, Text},
    Prompt,
};

pub mod keymap;
pub mod render;

/// A component for creating and managing a selectable list of options.
pub struct Listbox {
    keymap: ActiveKeySwitcher<keymap::Keymap>,
    /// State for the title displayed above the selectable list.
    title_state: text::State,
    /// State for the selectable list itself.
    listbox_state: listbox::State,
    /// Writer to which promptkit write its contents
    writer: Box<dyn io::Write>,
}

impl Listbox {
    /// Constructs a new `Listbox` instance
    /// with a list of items to be displayed as selectable options.
    ///
    /// # Arguments
    ///
    /// * `items` - An iterator over items
    ///   that implement the `Display` trait, to be used as options.
    pub fn new<T: Display, I: IntoIterator<Item = T>>(items: I) -> Self {
        Self {
            title_state: text::State {
                text: Default::default(),
                style: StyleBuilder::new()
                    .attrs(Attributes::from(Attribute::Bold))
                    .build(),
                lines: None,
            },
            listbox_state: listbox::State {
                listbox: listbox::Listbox::from_displayable(items),
                cursor: String::from("❯ "),
                active_item_style: Some(StyleBuilder::new().fgc(Color::DarkCyan).build()),
                inactive_item_style: Some(StyleBuilder::new().build()),
                lines: Default::default(),
            },
            keymap: ActiveKeySwitcher::new("default", self::keymap::default),
            writer: Box::new(io::stdout()),
        }
    }

    /// Sets the title text displayed above the selectable list.
    pub fn title<T: AsRef<str>>(mut self, text: T) -> Self {
        self.title_state.text = Text::from(text);
        self
    }

    /// Sets the style for the title text.
    pub fn title_style(mut self, style: ContentStyle) -> Self {
        self.title_state.style = style;
        self
    }

    /// Sets the cursor symbol used to indicate the current selection.
    pub fn cursor<T: AsRef<str>>(mut self, cursor: T) -> Self {
        self.listbox_state.cursor = cursor.as_ref().to_string();
        self
    }

    /// Sets the style for active (currently selected) items.
    pub fn active_item_style(mut self, style: ContentStyle) -> Self {
        self.listbox_state.active_item_style = Some(style);
        self
    }

    /// Sets the style for inactive (not currently selected) items.
    pub fn inactive_item_style(mut self, style: ContentStyle) -> Self {
        self.listbox_state.inactive_item_style = Some(style);
        self
    }

    /// Sets the number of lines to be used for displaying the selectable list.
    pub fn listbox_lines(mut self, lines: usize) -> Self {
        self.listbox_state.lines = Some(lines);
        self
    }

    pub fn register_keymap<K: AsRef<str>>(mut self, key: K, handler: keymap::Keymap) -> Self {
        self.keymap = self.keymap.register(key, handler);
        self
    }

    /// Sets writer.
    pub fn writer<W: io::Write + 'static>(mut self, writer: W) -> Self {
        self.writer = Box::new(writer);
        self
    }

    /// Displays the select prompt and waits for user input.
    /// Returns a `Result` containing the `Prompt` result,
    /// which is the selected option.
    pub fn prompt(self) -> anyhow::Result<Prompt<render::Renderer>> {
        Ok(Prompt {
            renderer: render::Renderer {
                keymap: RefCell::new(self.keymap),
                title_state: self.title_state,
                listbox_state: self.listbox_state,
            },
            writer: self.writer,
        })
    }
}
