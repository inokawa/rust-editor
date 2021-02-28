use core::{Error, Input, Key};

pub struct WebInput {}

impl Input for WebInput {
    fn new() -> Result<Self, Error> {
        Ok(WebInput {})
    }

    fn wait_for_key(&self) -> Key {
        Key::Enter
    }
}
