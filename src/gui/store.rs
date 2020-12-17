pub struct Store {
    pub welcomed: bool,
    pub connected: bool,
}

impl Store {
    pub fn new() -> Self {
        Store {
            welcomed: false,
            connected: false,
        }
    }
}