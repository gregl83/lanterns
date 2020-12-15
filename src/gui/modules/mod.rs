use tui::widgets::ListItem;

pub mod connection;
pub mod info;
pub mod message;

pub trait Module {
    fn init_store() {
        // todo - init module specific store
    }

    fn generate() -> ListItem {
        // todo - build widgets
    }
}