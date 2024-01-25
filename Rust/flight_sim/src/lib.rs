mod config;

use gdnative::prelude::*;

fn init(handle: InitHandle) {
    godot_print!("hello world (init)");
    handle.add_class::<config::Config>();
}

godot_init!(init);
