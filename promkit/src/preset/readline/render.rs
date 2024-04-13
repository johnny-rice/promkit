use crate::{
    impl_as_any, impl_cast, keymap::KeymapManager, listbox, pane::Pane, snapshot::Snapshot,
    suggest::Suggest, text, text_editor, validate::ValidatorManager, PaneFactory,
};

/// A `Renderer` for the readline preset, responsible for managing the rendering process.
/// It holds references to various components and their states, facilitating the rendering of the readline interface.
pub struct Renderer {
    /// Manages key bindings and their associated actions within the readline interface.
    pub keymap: KeymapManager<Self>,
    /// Holds a snapshot of the title's renderer state, used for rendering the title section.
    pub title_snapshot: Snapshot<text::State>,
    /// Holds a snapshot of the text editor's renderer state, used for rendering the text input area.
    pub text_editor_snapshot: Snapshot<text_editor::State>,
    /// Optional suggest component for autocomplete functionality.
    pub suggest: Option<Suggest>,
    /// Holds a snapshot of the suggest box's renderer state, used when rendering suggestions for autocomplete.
    pub suggest_snapshot: Snapshot<listbox::State>,
    /// Optional validator manager for input validation.
    pub validator: Option<ValidatorManager<str>>,
    /// Holds a snapshot of the error message's renderer state, used for rendering error messages.
    pub error_message_snapshot: Snapshot<text::State>,
}

impl_as_any!(Renderer);
impl_cast!(Renderer);

impl crate::Renderer for Renderer {
    fn create_panes(&self, width: u16) -> Vec<Pane> {
        vec![
            self.title_snapshot.create_pane(width),
            self.error_message_snapshot.create_pane(width),
            self.text_editor_snapshot.create_pane(width),
            self.suggest_snapshot.create_pane(width),
        ]
    }
}
