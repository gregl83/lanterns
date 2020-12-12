pub struct State {
    connected: bool
}

impl State {
    pub fn new() -> Self {
        State {
            connected: false,
        }
    }
}