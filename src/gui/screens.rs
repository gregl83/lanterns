use super::state::State;

pub trait Screenable {
    fn draw(&mut self) {
        // todo - load screen based on conditions
    }

    fn on_key(&mut self) {
        // todo - something when key is received
    }
}

pub struct Dashboard {}

impl Screenable for Dashboard {}

pub struct Connection {}

impl Screenable for Connection {}

pub struct Communicate {}

impl Screenable for Communicate {}