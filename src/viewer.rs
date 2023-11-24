use std::{any::Any, cell::RefCell};

use crate::{crossterm::event::Event, pane::Pane};

mod builder;
pub use builder::{
    item_picker::ItemPickerBuilder, text::TextBuilder, text_editor::TextEditorBuilder,
    tree_viewer::TreeViewerBuilder,
};
mod text_editor;
pub use text_editor::{Mode, TextEditor};
mod item_picker;
pub use item_picker::ItemPicker;
mod text;
pub use text::Text;
mod tree_viewer;
pub use tree_viewer::TreeViewer;

pub trait Viewable: AsAny {
    fn make_pane(&self, width: u16) -> Pane;
    fn handle_event(&mut self, event: &Event);
    fn postrun(&mut self);
}

pub trait AsAny {
    fn as_any(&self) -> &dyn Any;
}

pub struct State<V: Viewable> {
    pub init: V,
    pub before: V,
    pub after: RefCell<V>,
}

impl<V: Viewable + Clone> State<V> {
    pub fn new(viewable: V) -> Self {
        Self {
            init: viewable.clone(),
            before: viewable.clone(),
            after: RefCell::new(viewable),
        }
    }
}

impl<V: Clone + Viewable + 'static> Viewable for State<V> {
    fn make_pane(&self, width: u16) -> Pane {
        self.after.borrow().make_pane(width)
    }

    fn handle_event(&mut self, event: &Event) {
        self.before = self.after.borrow().clone();
        self.after.borrow_mut().handle_event(event);
    }

    fn postrun(&mut self) {
        self.after.borrow_mut().postrun();
        self.init = self.after.borrow().clone();
        self.before = self.after.borrow().clone();
    }
}

impl<V: Viewable + 'static> AsAny for State<V> {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
