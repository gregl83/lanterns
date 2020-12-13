use super::state::State;

trait Screenable {
    fn load() {
        // todo - load screen based on conditions
    }
}

struct Dashboard {}

impl Screenable for Dashboard {}

struct Connection {}

impl Screenable for Connection {}

struct Communicate {}

impl Screenable for Communicate {}