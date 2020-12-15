pub struct Store {
    connected: bool,
}

impl Store {
    pub fn new() -> Self {
        Store {
            connected: false,
        }
    }
}