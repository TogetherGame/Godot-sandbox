mod ferris;
mod main_scene;

use ferris::Ferris;
use main_scene::GameControl;
use gdnative::prelude::*;

fn init(handle: InitHandle) {
    handle.add_class::<Ferris>();
    handle.add_class::<GameControl>();
}

// Macro that creates the entry-points of the dynamic library.
godot_init!(init);
