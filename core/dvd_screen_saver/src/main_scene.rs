use gdnative::api::{Node2D, InputEventMouseMotion};
use gdnative::prelude::*;

#[derive(NativeClass, Debug, Default)]
#[inherit(Node2D)]
pub struct GameControl {
    #[property(default = 5)]
    cursor_move_threshold: u16,
    cursor_movement: u16,
    #[property(default = 1.0)]
    cursor_movement_timeout: f64,
}

impl GameControl {
    fn new(_base: &Node2D) -> Self {
        GameControl {
            cursor_move_threshold: 5,
            cursor_movement_timeout: 1.0,
            ..Default::default()
        }
    }
}

#[methods]
impl GameControl {
    #[method]
    fn _ready(&mut self, #[base] _base: &Node2D) {
        // Hide cursor
        let input = Input::godot_singleton();
        input.set_mouse_mode(Input::MOUSE_MODE_HIDDEN);
    }
    
    #[method]
    fn _input(&mut self, #[base] base: &Node2D, event: Ref<InputEvent>) {
        let ev = unsafe { event.assume_safe() };
        let input_timer = unsafe {
            base
                .get_node_as::<Timer>("InputTimer")
                .expect("no `InputTimer` node was found under `Scene` node")
        };
        let exit_ = || {
            input_timer.stop();
            std::process::exit(0);
        };

        if ev.cast::<InputEventMouseMotion>().is_some() {
            if input_timer.is_stopped() {
                input_timer.start(self.cursor_movement_timeout);
                self.cursor_movement = 0;
            } else {
                self.cursor_movement = self.cursor_movement.saturating_add(1);
                if self.cursor_movement > self.cursor_move_threshold {
                    exit_();
                }
            }
        } else {
            exit_();
        }
    }
}
