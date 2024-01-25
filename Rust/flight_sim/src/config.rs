use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct Config;

impl Config {
    fn new(_base: &Node) -> Self {
        Self
    }
}

#[methods]
impl Config {
    #[method]
    fn _ready(&mut self, #[base] _base: &Node) {
        let input = Input::godot_singleton();
        input.set_mouse_mode(Input::MOUSE_MODE_HIDDEN);
    }
}
