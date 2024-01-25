use gdnative::prelude::*;

#[derive(NativeClass, Debug, Default)]
#[inherit(Node2D)]
pub struct ControlBar {
    #[property]
    init_pos: Option<Vector2>,
}

impl ControlBar {
    fn new(_base: &Node2D) -> Self {
        Self::default()
    }
}

#[methods]
impl ControlBar {
    #[method]
    fn _ready(&mut self, #[base] base: &Node2D) {
        let Some(vp_ref) = base.get_viewport() else {
            return;
        };
        let vp = unsafe { vp_ref.assume_safe() };
        vp.set_transparent_background(true);
    }
}
