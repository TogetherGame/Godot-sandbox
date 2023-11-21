mod control;

use gdnative::prelude::*;

fn init(handle: InitHandle) {
    handle.add_class::<control::ControlBar>();
}

// Macro that creates the entry-points of the dynamic library.
godot_init!(init);
