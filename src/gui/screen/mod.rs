pub mod dashboard;
pub mod connection;
pub mod communicate;

use std::io::Stdout;

use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::ListState,
    Frame,
};

use crate::gui::application::Application;
use crate::gui::store::Store;
use crate::io::adapters::bluetooth::Device;

pub trait Screenable {
    fn draw(&mut self, f: &mut Frame<CrosstermBackend<Stdout>>);

    fn on_key(&mut self, c: char) { }
}