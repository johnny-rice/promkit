use std::{
    collections::VecDeque,
    fmt,
    ops::{Deref, DerefMut},
};

use unicode_width::UnicodeWidthChar;

use crate::crossterm::style::{Attribute, ContentStyle};

/// Represents a single grapheme (character) with its display width and optional styling.
///
/// This structure is similar to `Grapheme` but includes styling information directly.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StyledGrapheme {
    ch: char,
    width: usize,
    style: ContentStyle,
}

impl From<char> for StyledGrapheme {
    fn from(ch: char) -> Self {
        Self {
            ch,
            width: UnicodeWidthChar::width(ch).unwrap_or(0),
            style: ContentStyle::default(),
        }
    }
}

impl fmt::Display for StyledGraphemes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for styled_grapheme in self.iter() {
            write!(f, "{}", styled_grapheme.ch)?;
        }
        Ok(())
    }
}

impl StyledGrapheme {
    pub fn new(ch: char, style: ContentStyle) -> Self {
        Self {
            ch,
            width: UnicodeWidthChar::width(ch).unwrap_or(0),
            style,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn apply_style(&mut self, style: ContentStyle) {
        self.style = style;
    }
}

/// A collection of `StyledGrapheme` instances.
///
/// This structure supports operations like calculating the total display width of the collection
/// and generating a display representation that respects the applied styles.
#[derive(Clone, Default, PartialEq, Eq)]
pub struct StyledGraphemes(pub VecDeque<StyledGrapheme>);

impl Deref for StyledGraphemes {
    type Target = VecDeque<StyledGrapheme>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for StyledGraphemes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl FromIterator<StyledGraphemes> for StyledGraphemes {
    fn from_iter<I: IntoIterator<Item = StyledGraphemes>>(iter: I) -> Self {
        let concatenated = iter
            .into_iter()
            .flat_map(|g| g.0.into_iter())
            .collect::<VecDeque<StyledGrapheme>>();
        StyledGraphemes(concatenated)
    }
}

impl<'a> FromIterator<&'a StyledGraphemes> for StyledGraphemes {
    fn from_iter<I: IntoIterator<Item = &'a StyledGraphemes>>(iter: I) -> Self {
        let concatenated = iter
            .into_iter()
            .flat_map(|g| g.0.iter().cloned())
            .collect::<VecDeque<StyledGrapheme>>();
        StyledGraphemes(concatenated)
    }
}

impl FromIterator<StyledGrapheme> for StyledGraphemes {
    fn from_iter<I: IntoIterator<Item = StyledGrapheme>>(iter: I) -> Self {
        let mut g = StyledGraphemes::default();
        for i in iter {
            g.push_back(i);
        }
        g
    }
}

impl<S: AsRef<str>> From<S> for StyledGraphemes {
    fn from(string: S) -> Self {
        Self::from_str(string, ContentStyle::default())
    }
}

impl fmt::Debug for StyledGraphemes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for styled_grapheme in self.iter() {
            write!(f, "{}", styled_grapheme.ch)?;
        }
        Ok(())
    }
}

impl StyledGraphemes {
    pub fn from_str<S: AsRef<str>>(string: S, style: ContentStyle) -> Self {
        string
            .as_ref()
            .chars()
            .map(|ch| StyledGrapheme::new(ch, style))
            .collect()
    }

    /// Returns a `Vec<char>` containing the characters of all `Grapheme` instances in the collection.
    pub fn chars(&self) -> Vec<char> {
        self.0.iter().map(|grapheme| grapheme.ch).collect()
    }

    /// Calculates the total display width of all `Grapheme` instances in the collection.
    pub fn widths(&self) -> usize {
        self.0.iter().map(|grapheme| grapheme.width).sum()
    }

    /// Replaces all occurrences of a substring `from` with another substring `to` within the `StyledGraphemes`.
    pub fn replace<S: AsRef<str>>(mut self, from: S, to: S) -> Self {
        let from_len = from.as_ref().chars().count();
        let to_len = to.as_ref().chars().count();

        let mut offset = 0;
        let diff = from_len.abs_diff(to_len);

        let pos = self.find_all(from);

        for p in pos {
            let adjusted_pos = if to_len > from_len {
                p + offset
            } else {
                p.saturating_sub(offset)
            };
            self.replace_range(adjusted_pos..adjusted_pos + from_len, &to);
            offset += diff;
        }

        self
    }

    /// Replaces the specified range with the given string.
    pub fn replace_range<S: AsRef<str>>(&mut self, range: std::ops::Range<usize>, replacement: S) {
        // Remove the specified range.
        for _ in range.clone() {
            self.0.remove(range.start);
        }

        // Insert the replacement at the start of the range.
        let replacement_graphemes: StyledGraphemes = replacement.as_ref().into();
        for grapheme in replacement_graphemes.0.iter().rev() {
            self.0.insert(range.start, grapheme.clone());
        }
    }

    /// Applies a given style to all `StyledGrapheme` instances within the collection.
    pub fn apply_style(mut self, style: ContentStyle) -> Self {
        for grapheme in &mut self.0 {
            grapheme.apply_style(style);
        }
        self
    }

    /// Applies a given style to a specific `StyledGrapheme` at the specified index.
    pub fn apply_style_at(mut self, idx: usize, style: ContentStyle) -> Self {
        if let Some(grapheme) = self.0.get_mut(idx) {
            grapheme.apply_style(style);
        }
        self
    }

    /// Finds all occurrences of a query string within the StyledGraphemes and returns their start indices.
    pub fn find_all<S: AsRef<str>>(&self, query: S) -> Vec<usize> {
        let query_str = query.as_ref();
        if query_str.is_empty() {
            return Vec::new();
        }

        let mut indices = Vec::new();
        let mut pos = 0;
        let query_chars: Vec<char> = query_str.chars().collect();
        let query_len = query_chars.len();

        // Iterate through each grapheme in self
        while pos + query_len <= self.0.len() {
            let mut match_found = true;
            for (i, query_char) in query_chars.iter().enumerate() {
                if self.0[pos + i].ch != *query_char {
                    match_found = false;
                    break;
                }
            }
            if match_found {
                indices.push(pos);
                pos += 1; // Move to the next position even after a match
            } else {
                pos += 1; // Check the next position
            }
        }

        indices
    }

    /// Highlights all occurrences of a specified query string
    /// within the `StyledGraphemes` collection by applying a given style.
    ///
    /// # Returns
    /// An `Option<Self>` which is:
    /// - `Some(Self)`:
    ///     - with the style applied to all occurrences of the query if the query is found.
    ///     - unchanged if the query string is empty.
    /// - `None`: if the query string is not found in the collection.

    pub fn highlight<S: AsRef<str>>(mut self, query: S, style: ContentStyle) -> Option<Self> {
        let query_str = query.as_ref();
        if query_str.is_empty() {
            return Some(self);
        }

        let indices = self.find_all(query_str);
        if indices.is_empty() {
            return None;
        }

        let query_len = query_str.chars().count();

        for &start_index in &indices {
            for i in start_index..start_index + query_len {
                if let Some(grapheme) = self.0.get_mut(i) {
                    grapheme.apply_style(style);
                }
            }
        }

        Some(self)
    }

    /// Applies a given attribute to all `StyledGrapheme` instances within the collection.
    pub fn apply_attribute(mut self, attr: Attribute) -> Self {
        for styled_grapheme in &mut self.0 {
            styled_grapheme.style.attributes.set(attr);
        }
        self
    }

    /// Returns a displayable format of the styled graphemes.
    pub fn styled_display(&self) -> StyledGraphemesDisplay<'_> {
        StyledGraphemesDisplay {
            styled_graphemes: self,
        }
    }

    /// Organizes the `StyledGraphemes` into a matrix format based on specified width and height,
    /// considering an offset for pagination or scrolling.
    pub fn matrixify(
        &self,
        width: usize,
        height: usize,
        offset: usize,
    ) -> (Vec<StyledGraphemes>, usize) {
        let mut all = VecDeque::new();
        let mut row = StyledGraphemes::default();
        for styled in self.iter() {
            let width_with_next_char = row.iter().fold(0, |mut layout, g| {
                layout += g.width;
                layout
            }) + styled.width;
            if !row.is_empty() && width < width_with_next_char {
                all.push_back(row);
                row = StyledGraphemes::default();
            }
            if width >= styled.width {
                row.push_back(styled.clone());
            }
        }
        if !row.is_empty() {
            all.push_back(row);
        }

        if all.is_empty() {
            return (vec![], 0);
        }

        let mut offset = std::cmp::min(offset, all.len().saturating_sub(1));

        // Adjust the start and end rows based on the offset and height
        while all.len() > height && offset < all.len() {
            if offset > 0 {
                all.pop_front();
                offset -= 1;
            } else {
                all.pop_back();
            }
        }

        (Vec::from(all), offset)
    }
}

pub struct StyledGraphemesDisplay<'a> {
    styled_graphemes: &'a StyledGraphemes,
}

impl fmt::Display for StyledGraphemesDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for styled_grapheme in self.styled_graphemes.iter() {
            write!(f, "{}", styled_grapheme.style.apply(styled_grapheme.ch))?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod from_str {
        use super::*;

        #[test]
        fn test() {
            let style = ContentStyle::default();
            let graphemes = StyledGraphemes::from_str("abc", style.clone());
            assert_eq!(3, graphemes.0.len());
            assert!(graphemes.0.iter().all(|g| g.style == style));
        }
    }

    mod chars {
        use super::*;

        #[test]
        fn test() {
            let graphemes = StyledGraphemes::from("abc");
            let chars = graphemes.chars();
            assert_eq!(vec!['a', 'b', 'c'], chars);
        }
    }

    mod widths {
        use super::*;

        #[test]
        fn test() {
            let graphemes = StyledGraphemes::from("a b");
            assert_eq!(3, graphemes.widths()); // 'a' and 'b' are each 1 width, and space is 1 width
        }
    }

    mod replace_char {
        use super::*;

        #[test]
        fn test() {
            let graphemes = StyledGraphemes::from("banana");
            assert_eq!("bonono", graphemes.replace("a", "o").to_string());
        }

        #[test]
        fn test_with_nonexistent_character() {
            let graphemes = StyledGraphemes::from("Hello World");
            assert_eq!("Hello World", graphemes.replace("x", "o").to_string());
        }

        #[test]
        fn test_with_empty_string() {
            let graphemes = StyledGraphemes::from("Hello World");
            assert_eq!("Hell Wrld", graphemes.replace("o", "").to_string());
        }

        #[test]
        fn test_with_multiple_characters() {
            let graphemes = StyledGraphemes::from("Hello World");
            assert_eq!("Hellabc Wabcrld", graphemes.replace("o", "abc").to_string());
        }
    }

    mod replace_range {
        use super::*;

        #[test]
        fn test() {
            let mut graphemes = StyledGraphemes::from("Hello");
            graphemes.replace_range(1..5, "i");
            assert_eq!("Hi", graphemes.to_string());
        }
    }

    mod apply_style {
        use crate::{crossterm::style::Color, style::StyleBuilder};

        use super::*;

        #[test]
        fn test() {
            let mut graphemes = StyledGraphemes::from("abc");
            let new_style = StyleBuilder::new().fgc(Color::Green).build();
            graphemes = graphemes.apply_style(new_style.clone());
            assert!(graphemes.iter().all(|g| g.style == new_style));
        }
    }

    mod apply_style_at {
        use crate::{crossterm::style::Color, style::StyleBuilder};

        use super::*;

        #[test]
        fn test_apply_style_at_specific_index() {
            let mut graphemes = StyledGraphemes::from("abc");
            let new_style = StyleBuilder::new().fgc(Color::Green).build();
            graphemes = graphemes.apply_style_at(1, new_style.clone());
            assert_eq!(graphemes.0[1].style, new_style);
            assert_ne!(graphemes.0[0].style, new_style);
            assert_ne!(graphemes.0[2].style, new_style);
        }

        #[test]
        fn test_apply_style_at_out_of_bounds_index() {
            let mut graphemes = StyledGraphemes::from("abc");
            let new_style = StyleBuilder::new().fgc(Color::Green).build();
            graphemes = graphemes.apply_style_at(5, new_style.clone()); // Out of bounds
            assert_eq!(graphemes.0.len(), 3); // Ensure no changes in length
        }
    }

    mod find_all {
        use super::*;

        #[test]
        fn test_with_empty_query() {
            let graphemes = StyledGraphemes::from("Hello, world!");
            let indices = graphemes.find_all("");
            assert!(
                indices.is_empty(),
                "Should return an empty vector for an empty query string"
            );
        }

        #[test]
        fn test_with_repeated_substring() {
            let graphemes = StyledGraphemes::from("Hello, world! Hello, universe!");
            let indices = graphemes.find_all("Hello");
            assert_eq!(
                indices,
                vec![0, 14],
                "Should find all starting indices of 'Hello'"
            );
        }

        #[test]
        fn test_with_nonexistent_substring() {
            let graphemes = StyledGraphemes::from("Hello, world!");
            let indices = graphemes.find_all("xyz");
            assert!(
                indices.is_empty(),
                "Should return an empty vector for a non-existent substring"
            );
        }

        #[test]
        fn test_with_special_character() {
            let graphemes = StyledGraphemes::from("µs µs µs");
            let indices = graphemes.find_all("s");
            assert_eq!(
                indices,
                vec![1, 4, 7],
                "Should correctly find indices of substring 'µs'"
            );
        }

        #[test]
        fn test_with_single_character() {
            let graphemes = StyledGraphemes::from("abcabcabc");
            let indices = graphemes.find_all("b");
            assert_eq!(
                indices,
                vec![1, 4, 7],
                "Should find all indices of character 'b'"
            );
        }

        #[test]
        fn test_with_full_match() {
            let graphemes = StyledGraphemes::from("Hello");
            let indices = graphemes.find_all("Hello");
            assert_eq!(indices, vec![0], "Should match the entire string");
        }

        #[test]
        fn test_with_partial_overlap() {
            let graphemes = StyledGraphemes::from("ababa");
            let indices = graphemes.find_all("aba");
            assert_eq!(
                indices,
                vec![0, 2],
                "Should handle overlapping matches correctly"
            );
        }
    }

    mod highlight {
        use super::*;

        #[test]
        fn test_with_empty_query() {
            let graphemes = StyledGraphemes::from("Hello, world!");
            let expected = graphemes.clone();
            let highlighted = graphemes.highlight("", ContentStyle::default());
            assert_eq!(highlighted.unwrap(), expected);
        }
    }

    mod apply_attribute {
        use super::*;

        #[test]
        fn test() {
            let mut graphemes = StyledGraphemes::from("abc");
            graphemes = graphemes.apply_attribute(Attribute::Bold);
            assert!(graphemes
                .iter()
                .all(|g| g.style.attributes.has(Attribute::Bold)));
        }
    }

    mod styled_display {
        use super::*;

        #[test]
        fn test() {
            let graphemes = StyledGraphemes::from("abc");
            let display = graphemes.styled_display();
            assert_eq!(format!("{}", display), "abc"); // Assuming default styles do not alter appearance
        }
    }

    #[cfg(test)]
    mod matrixify {
        use super::*;

        #[test]
        fn test_with_empty_input() {
            let input = StyledGraphemes::default();
            let (matrix, offset) = input.matrixify(10, 2, 0);
            assert_eq!(matrix.len(), 0);
            assert_eq!(offset, 0);
        }

        #[test]
        fn test_with_exact_width_fit() {
            let input = StyledGraphemes::from("1234567890");
            let (matrix, offset) = input.matrixify(10, 1, 0);
            assert_eq!(matrix.len(), 1);
            assert_eq!("1234567890", matrix[0].to_string());
            assert_eq!(offset, 0);
        }

        #[test]
        fn test_with_narrow_width() {
            let input = StyledGraphemes::from("1234567890");
            let (matrix, offset) = input.matrixify(5, 2, 0);
            assert_eq!(matrix.len(), 2);
            assert_eq!("12345", matrix[0].to_string());
            assert_eq!("67890", matrix[1].to_string());
            assert_eq!(offset, 0);
        }

        #[test]
        fn test_with_offset() {
            let input = StyledGraphemes::from("1234567890");
            let (matrix, offset) = input.matrixify(2, 2, 1);
            assert_eq!(matrix.len(), 2);
            assert_eq!("34", matrix[0].to_string());
            assert_eq!("56", matrix[1].to_string());
            assert_eq!(offset, 0);
        }

        #[test]
        fn test_with_padding() {
            let input = StyledGraphemes::from("1234567890");
            let (matrix, offset) = input.matrixify(2, 100, 1);
            assert_eq!(matrix.len(), 5);
            assert_eq!("12", matrix[0].to_string());
            assert_eq!("34", matrix[1].to_string());
            assert_eq!("56", matrix[2].to_string());
            assert_eq!("78", matrix[3].to_string());
            assert_eq!("90", matrix[4].to_string());
            assert_eq!(offset, 1);
        }

        #[test]
        fn test_with_large_offset() {
            let input = StyledGraphemes::from("1234567890");
            let (matrix, offset) = input.matrixify(10, 2, 100); // Offset beyond content
            assert_eq!(matrix.len(), 1);
            assert_eq!("1234567890", matrix[0].to_string());
            assert_eq!(offset, 0);
        }
    }
}
