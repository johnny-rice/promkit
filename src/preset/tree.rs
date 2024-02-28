use crate::{
    crossterm::style::{Attribute, Attributes, Color, ContentStyle},
    error::Result,
    snapshot::Snapshot,
    style::StyleBuilder,
    text,
    tree::{self, Node},
    Prompt, Renderable,
};

/// Represents a tree component for creating
/// and managing a hierarchical list of options.
pub struct Tree {
    /// Renderer for the title displayed above the tree.
    title_renderer: text::Renderer,
    /// Renderer for the tree itself.
    tree_renderer: tree::Renderer,
}

impl Tree {
    /// Constructs a new `Tree` instance with a specified root node.
    ///
    /// # Arguments
    ///
    /// * `root` - The root node of the tree.
    pub fn new(root: Node) -> Self {
        Self {
            title_renderer: text::Renderer {
                text: Default::default(),
                style: StyleBuilder::new()
                    .attrs(Attributes::from(Attribute::Bold))
                    .build(),
            },
            tree_renderer: tree::Renderer {
                tree: tree::Tree::new(root),
                folded_symbol: String::from("▶︎ "),
                unfolded_symbol: String::from("▼ "),
                active_item_style: StyleBuilder::new().fgc(Color::DarkCyan).build(),
                inactive_item_style: StyleBuilder::new().build(),
                lines: Default::default(),
                indent: 2,
            },
        }
    }

    /// Sets the title text displayed above the tree.
    pub fn title<T: AsRef<str>>(mut self, text: T) -> Self {
        self.title_renderer.text = text.as_ref().to_string();
        self
    }

    /// Sets the style for the title text.
    pub fn title_style(mut self, style: ContentStyle) -> Self {
        self.title_renderer.style = style;
        self
    }

    /// Sets the symbol used to indicate a folded (collapsed) node.
    pub fn folded_symbol<T: AsRef<str>>(mut self, symbol: T) -> Self {
        self.tree_renderer.folded_symbol = symbol.as_ref().to_string();
        self
    }

    /// Sets the symbol used to indicate an unfolded (expanded) node.
    pub fn unfolded_symbol<T: AsRef<str>>(mut self, symbol: T) -> Self {
        self.tree_renderer.unfolded_symbol = symbol.as_ref().to_string();
        self
    }

    /// Sets the style for active (currently selected) items.
    pub fn active_item_style(mut self, style: ContentStyle) -> Self {
        self.tree_renderer.active_item_style = style;
        self
    }

    /// Sets the style for inactive (not currently selected) items.
    pub fn inactive_item_style(mut self, style: ContentStyle) -> Self {
        self.tree_renderer.inactive_item_style = style;
        self
    }

    /// Sets the number of lines to be used for displaying the tree.
    pub fn tree_lines(mut self, lines: usize) -> Self {
        self.tree_renderer.lines = Some(lines);
        self
    }

    /// Sets the indentation level for rendering the tree data.
    pub fn indent(mut self, indent: usize) -> Self {
        self.tree_renderer.indent = indent;
        self
    }

    /// Displays the tree prompt and waits for user input.
    /// Returns a `Result` containing the `Prompt` result,
    /// which is a list of selected options.
    pub fn prompt(self) -> Result<Prompt<Vec<String>>> {
        Prompt::try_new(
            vec![
                Box::new(Snapshot::<text::Renderer>::new(self.title_renderer)),
                Box::new(Snapshot::<tree::Renderer>::new(self.tree_renderer)),
            ],
            |_, _| Ok(true),
            |renderables: &Vec<Box<dyn Renderable + 'static>>| -> Result<Vec<String>> {
                Ok(renderables[1]
                    .as_any()
                    .downcast_ref::<Snapshot<tree::Renderer>>()
                    .unwrap()
                    .after
                    .borrow()
                    .tree
                    .get())
            },
        )
    }
}
