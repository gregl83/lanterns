pub struct Store {
    pub welcomed: bool,
    pub connected: bool,
    pub in_call: bool,
}

impl Store {
    pub fn new() -> Self {
        Store {
            welcomed: false,
            connected: false,
            in_call: false,
        }
    }
}