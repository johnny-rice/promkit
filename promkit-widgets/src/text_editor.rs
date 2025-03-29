use std::collections::HashSet;

use promkit_core::grapheme::{StyledGrapheme, StyledGraphemes};

use crate::cursor::Cursor;

mod history;
pub use history::History;
mod state;
pub use state::State;

/// Edit mode.
#[derive(Clone, Default)]
pub enum Mode {
    #[default]
    /// Insert a char at the current position.
    Insert,
    /// Overwrite a char at the current position.
    Overwrite,
}

/// A text editor that supports basic editing operations
/// such as insert, delete, and overwrite.
/// It utilizes a cursor to navigate and manipulate the text.
#[derive(Clone)]
pub struct TextEditor(Cursor<StyledGraphemes>);

impl Default for TextEditor {
    fn default() -> Self {
        Self(Cursor::new(
            // Set cursor
            StyledGraphemes::from(" "),
            0,
            false,
        ))
    }
}

impl TextEditor {
    pub fn new<S: AsRef<str>>(s: S) -> Self {
        let mut buf = s.as_ref().to_owned();
        buf.push(' ');
        let pos = buf.len() - 1;
        Self(Cursor::new(StyledGraphemes::from(buf), pos, false))
    }

    /// Returns the current text including the cursor.
    pub fn text(&self) -> StyledGraphemes {
        self.0.contents().clone()
    }

    /// Returns the text without the cursor.
    pub fn text_without_cursor(&self) -> StyledGraphemes {
        let mut ret = self.text();
        ret.pop_back();
        ret
    }

    /// Returns the current position of the cursor within the text.
    pub fn position(&self) -> usize {
        self.0.position()
    }

    /// Masks all characters except the cursor with the specified mask character.
    pub fn masking(&self, mask: char) -> StyledGraphemes {
        self.text()
            .chars()
            .into_iter()
            .enumerate()
            .map(|(i, c)| StyledGrapheme::from(if i == self.text().len() - 1 { c } else { mask }))
            .collect::<StyledGraphemes>()
    }

    /// Replaces the current text with new text and positions the cursor at the end.
    pub fn replace(&mut self, new: &str) {
        let mut buf = new.to_owned();
        buf.push(' ');
        let pos = buf.len() - 1;
        *self = Self(Cursor::new(StyledGraphemes::from(buf), pos, false));
    }

    /// Inserts a character at the current cursor position.
    pub fn insert(&mut self, ch: char) {
        let pos = self.position();
        self.0.contents_mut().insert(pos, StyledGrapheme::from(ch));
        self.forward();
    }

    pub fn insert_chars(&mut self, vch: &Vec<char>) {
        for ch in vch {
            self.insert(*ch);
        }
    }

    /// Overwrites the character at the current cursor position with the specified character.
    pub fn overwrite(&mut self, ch: char) {
        if self.0.is_tail() {
            self.insert(ch)
        } else {
            let pos = self.position();
            self.0
                .contents_mut()
                .replace_range(pos..pos + 1, ch.to_string());
            self.forward();
        }
    }

    pub fn overwrite_chars(&mut self, vch: &Vec<char>) {
        for ch in vch {
            self.overwrite(*ch);
        }
    }

    /// Erases the character before the cursor position.
    pub fn erase(&mut self) {
        if !self.0.is_head() {
            self.backward();
            let pos = self.position();
            self.0.contents_mut().drain(pos..pos + 1);
        }
    }

    /// Clears all text and resets the editor to its default state.
    pub fn erase_all(&mut self) {
        *self = Self::default();
    }

    /// Erases the text from the current cursor position to the specified position,
    /// considering whether pos is greater or smaller than the current position.
    fn erase_to_position(&mut self, pos: usize) {
        let current_pos = self.position();
        if pos > current_pos {
            self.0.contents_mut().drain(current_pos..pos);
        } else {
            self.0.contents_mut().drain(pos..current_pos);
            self.0.move_to(pos);
        }
    }

    /// Finds the nearest previous index of any character in `word_break_chars` from the cursor position.
    fn find_previous_nearest_index(&self, word_break_chars: &HashSet<char>) -> usize {
        let current_position = self.position();
        self.text()
            .chars()
            .iter()
            .enumerate()
            .filter(|&(i, _)| i < current_position.saturating_sub(1))
            .rev()
            .find(|&(_, c)| word_break_chars.contains(c))
            .map(|(i, _)| i + 1)
            .unwrap_or(0)
    }

    /// Erases the text from the current cursor position to the nearest previous character in `word_break_chars`.
    pub fn erase_to_previous_nearest(&mut self, word_break_chars: &HashSet<char>) {
        let pos = self.find_previous_nearest_index(word_break_chars);
        self.erase_to_position(pos);
    }

    /// Moves the cursor to the nearest previous character in `word_break_chars`.
    pub fn move_to_previous_nearest(&mut self, word_break_chars: &HashSet<char>) {
        let pos = self.find_previous_nearest_index(word_break_chars);
        self.0.move_to(pos);
    }

    /// Finds the nearest next index of any character in `word_break_chars` from the cursor position.
    fn find_next_nearest_index(&self, word_break_chars: &HashSet<char>) -> usize {
        let current_position = self.position();
        self.text()
            .chars()
            .iter()
            .enumerate()
            .filter(|&(i, _)| i > current_position)
            .find(|&(_, c)| word_break_chars.contains(c))
            .map(|(i, _)| {
                if i < self.0.contents().len() - 1 {
                    i + 1
                } else {
                    self.0.contents().len() - 1
                }
            })
            .unwrap_or(self.0.contents().len() - 1)
    }

    /// Erases the text from the current cursor position to the nearest next character in `word_break_chars`.
    pub fn erase_to_next_nearest(&mut self, word_break_chars: &HashSet<char>) {
        let pos = self.find_next_nearest_index(word_break_chars);
        self.erase_to_position(pos);
    }

    /// Moves the cursor to the nearest next character in `word_break_chars`.
    pub fn move_to_next_nearest(&mut self, word_break_chars: &HashSet<char>) {
        let pos = self.find_next_nearest_index(word_break_chars);
        self.0.move_to(pos);
    }

    /// Moves the cursor to the beginning of the text.
    pub fn move_to_head(&mut self) {
        self.0.move_to_head()
    }

    /// Moves the cursor to the end of the text.
    pub fn move_to_tail(&mut self) {
        self.0.move_to_tail()
    }

    pub fn shift(&mut self, backward: usize, forward: usize) -> bool {
        self.0.shift(backward, forward)
    }

    /// Moves the cursor one position backward, if possible.
    pub fn backward(&mut self) -> bool {
        self.0.backward()
    }

    /// Moves the cursor one position forward, if possible.
    pub fn forward(&mut self) -> bool {
        self.0.forward()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn new_with_position(s: String, p: usize) -> TextEditor {
        TextEditor(Cursor::new(StyledGraphemes::from(s), p, false))
    }

    mod masking {
        use super::*;

        #[test]
        fn test() {
            let txt = new_with_position(String::from("abcde "), 0);
            assert_eq!(StyledGraphemes::from("***** "), txt.masking('*'))
        }
    }

    mod erase {
        use super::*;

        #[test]
        fn test_for_empty() {
            let txt = TextEditor::default();
            assert_eq!(StyledGraphemes::from(" "), txt.text());
            assert_eq!(0, txt.position());
        }

        #[test]
        fn test_at_non_edge() {
            let mut txt = new_with_position(
                String::from("abc "),
                1, // indicate `b`.
            );
            let new = new_with_position(
                String::from("bc "),
                0, // indicate `b`.
            );
            txt.erase();
            assert_eq!(new.text(), txt.text());
            assert_eq!(new.position(), txt.position());
        }

        #[test]
        fn test_at_tail() {
            let mut txt = new_with_position(
                String::from("abc "),
                3, // indicate tail.
            );
            let new = new_with_position(
                String::from("ab "),
                2, // indicate tail.
            );
            txt.erase();
            assert_eq!(new.text(), txt.text());
            assert_eq!(new.position(), txt.position());
        }

        #[test]
        fn test_at_head() {
            let txt = new_with_position(
                String::from("abc "),
                0, // indicate `a`.
            );
            assert_eq!(StyledGraphemes::from("abc "), txt.text());
            assert_eq!(0, txt.position());
        }
    }

    mod find_previous_nearest_index {
        use super::*;

        use std::collections::HashSet;

        #[test]
        fn test() {
            let mut txt = new_with_position(String::from("koko momo jojo "), 11); // indicate `o`.
            assert_eq!(10, txt.find_previous_nearest_index(&HashSet::from([' '])));
            txt.0.move_to(10);
            assert_eq!(5, txt.find_previous_nearest_index(&HashSet::from([' '])));
        }

        #[test]
        fn test_with_no_target() {
            let txt = new_with_position(String::from("koko momo jojo "), 7); // indicate `m`.
            assert_eq!(0, txt.find_previous_nearest_index(&HashSet::from(['z'])));
        }
    }

    mod find_next_nearest_index {
        use super::*;

        use std::collections::HashSet;

        #[test]
        fn test() {
            let mut txt = new_with_position(String::from("koko momo jojo "), 7); // indicate `m`.
            assert_eq!(10, txt.find_next_nearest_index(&HashSet::from([' '])));
            txt.0.move_to(10);
            assert_eq!(14, txt.find_next_nearest_index(&HashSet::from([' '])));
        }

        #[test]
        fn test_with_no_target() {
            let txt = new_with_position(String::from("koko momo jojo "), 7); // indicate `m`.
            assert_eq!(14, txt.find_next_nearest_index(&HashSet::from(['z'])));
        }
    }

    mod insert {
        use super::*;

        #[test]
        fn test_for_empty() {
            let mut txt = TextEditor::default();
            let new = new_with_position(
                String::from("d "),
                1, // indicate tail.
            );
            txt.insert('d');
            assert_eq!(new.text(), txt.text());
            assert_eq!(new.position(), txt.position());
        }

        #[test]
        fn test_at_non_edge() {
            let mut txt = new_with_position(
                String::from("abc "),
                1, // indicate `b`.
            );
            let new = new_with_position(
                String::from("adbc "),
                2, // indicate `b`.
            );
            txt.insert('d');
            assert_eq!(new.text(), txt.text());
            assert_eq!(new.position(), txt.position());
        }

        #[test]
        fn test_at_tail() {
            let mut txt = new_with_position(
                String::from("abc "),
                3, // indicate tail.
            );
            let new = new_with_position(
                String::from("abcd "),
                4, // indicate tail.
            );
            txt.insert('d');
            assert_eq!(new.text(), txt.text());
            assert_eq!(new.position(), txt.position());
        }

        #[test]
        fn test_at_head() {
            let mut txt = new_with_position(
                String::from("abc "),
                0, // indicate `a`.
            );
            let new = new_with_position(
                String::from("dabc "),
                1, // indicate `a`.
            );
            txt.insert('d');
            assert_eq!(new.text(), txt.text());
            assert_eq!(new.position(), txt.position());
        }
    }

    mod overwrite {
        use super::*;

        #[test]
        fn test_for_empty() {
            let mut txt = TextEditor::default();
            let new = new_with_position(
                String::from("d "),
                1, // indicate tail.
            );
            txt.overwrite('d');
            assert_eq!(new.text(), txt.text());
            assert_eq!(new.position(), txt.position());
        }

        #[test]
        fn test_at_non_edge() {
            let mut txt = new_with_position(
                String::from("abc "),
                1, // indicate `b`.
            );
            let new = new_with_position(
                String::from("adc "),
                2, // indicate `c`.
            );
            txt.overwrite('d');
            assert_eq!(new.text(), txt.text());
            assert_eq!(new.position(), txt.position());
        }

        #[test]
        fn test_at_tail() {
            let mut txt = new_with_position(
                String::from("abc "),
                3, // indicate tail.
            );
            let new = new_with_position(
                String::from("abcd "),
                4, // indicate tail.
            );
            txt.overwrite('d');
            assert_eq!(new.text(), txt.text());
            assert_eq!(new.position(), txt.position());
        }

        #[test]
        fn test_at_head() {
            let mut txt = new_with_position(
                String::from("abc "),
                0, // indicate `a`.
            );
            let new = new_with_position(
                String::from("dbc "),
                1, // indicate `b`.
            );
            txt.overwrite('d');
            assert_eq!(new.text(), txt.text());
            assert_eq!(new.position(), txt.position());
        }
    }

    mod backward {
        use super::*;

        #[test]
        fn test_for_empty() {
            let mut txt = TextEditor::default();
            txt.backward();
            assert_eq!(StyledGraphemes::from(" "), txt.text());
            assert_eq!(0, txt.position());
        }

        #[test]
        fn test_at_non_edge() {
            let mut txt = new_with_position(
                String::from("abc "),
                1, // indicate `b`.
            );
            let new = new_with_position(
                String::from("abc "),
                0, // indicate `a`.
            );
            txt.backward();
            assert_eq!(new.text(), txt.text());
            assert_eq!(new.position(), txt.position());
        }

        #[test]
        fn test_at_tail() {
            let mut txt = new_with_position(
                String::from("abc "),
                3, // indicate tail.
            );
            let new = new_with_position(
                String::from("abc "),
                2, // indicate `c`.
            );
            txt.backward();
            assert_eq!(new.text(), txt.text());
            assert_eq!(new.position(), txt.position());
        }

        #[test]
        fn test_at_head() {
            let mut txt = new_with_position(
                String::from("abc "),
                0, // indicate `a`.
            );
            txt.backward();
            assert_eq!(StyledGraphemes::from("abc "), txt.text());
            assert_eq!(0, txt.position());
        }
    }

    mod forward {
        use super::*;

        #[test]
        fn test_for_empty() {
            let mut txt = TextEditor::default();
            txt.forward();
            assert_eq!(StyledGraphemes::from(" "), txt.text());
            assert_eq!(0, txt.position());
        }

        #[test]
        fn test_at_non_edge() {
            let mut txt = new_with_position(
                String::from("abc "),
                1, // indicate `b`.
            );
            let new = new_with_position(
                String::from("abc "),
                2, // indicate `c`.
            );
            txt.forward();
            assert_eq!(new.text(), txt.text());
            assert_eq!(new.position(), txt.position());
        }

        #[test]
        fn test_at_tail() {
            let mut txt = new_with_position(
                String::from("abc "),
                3, // indicate tail.
            );
            txt.forward();
            assert_eq!(StyledGraphemes::from("abc "), txt.text());
            assert_eq!(3, txt.position());
        }

        #[test]
        fn test_at_head() {
            let mut txt = new_with_position(
                String::from("abc "),
                0, // indicate `a`.
            );
            let new = new_with_position(
                String::from("abc "),
                1, // indicate `b`.
            );
            txt.forward();
            assert_eq!(new.text(), txt.text());
            assert_eq!(new.position(), txt.position());
        }
    }

    mod to_head {
        use super::*;

        #[test]
        fn test_for_empty() {
            let mut txt = TextEditor::default();
            txt.move_to_head();
            assert_eq!(StyledGraphemes::from(" "), txt.text());
            assert_eq!(0, txt.position());
        }

        #[test]
        fn test_at_non_edge() {
            let mut txt = new_with_position(
                String::from("abc "),
                1, // indicate `b`.
            );
            let new = new_with_position(
                String::from("abc "),
                0, // indicate `a`.
            );
            txt.move_to_head();
            assert_eq!(new.text(), txt.text());
            assert_eq!(new.position(), txt.position());
        }

        #[test]
        fn test_at_tail() {
            let mut txt = new_with_position(
                String::from("abc "),
                3, // indicate tail.
            );
            let new = new_with_position(
                String::from("abc "),
                0, // indicate `a`.
            );
            txt.move_to_head();
            assert_eq!(new.text(), txt.text());
            assert_eq!(new.position(), txt.position());
        }

        #[test]
        fn test_at_head() {
            let mut txt = new_with_position(
                String::from("abc "),
                0, // indicate `a`.
            );
            txt.move_to_head();
            assert_eq!(StyledGraphemes::from("abc "), txt.text());
            assert_eq!(0, txt.position());
        }
    }

    mod to_tail {
        use super::*;

        #[test]
        fn test_for_empty() {
            let mut txt = TextEditor::default();
            txt.move_to_tail();
            assert_eq!(StyledGraphemes::from(" "), txt.text());
            assert_eq!(0, txt.position());
        }

        #[test]
        fn test_at_non_edge() {
            let mut txt = new_with_position(
                String::from("abc "),
                1, // indicate `b`.
            );
            let new = new_with_position(
                String::from("abc "),
                3, // indicate tail.
            );
            txt.move_to_tail();
            assert_eq!(new.text(), txt.text());
            assert_eq!(new.position(), txt.position());
        }

        #[test]
        fn test_at_tail() {
            let mut txt = new_with_position(
                String::from("abc "),
                3, // indicate tail.
            );
            txt.move_to_tail();
            assert_eq!(StyledGraphemes::from("abc "), txt.text());
            assert_eq!(3, txt.position());
        }

        #[test]
        fn test_at_head() {
            let mut txt = new_with_position(
                String::from("abc "),
                0, // indicate `a`.
            );
            let new = new_with_position(
                String::from("abc "),
                3, // indicate tail.
            );
            txt.move_to_tail();
            assert_eq!(new.text(), txt.text());
            assert_eq!(new.position(), txt.position());
        }
    }
}
