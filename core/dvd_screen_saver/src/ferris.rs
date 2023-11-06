use gdnative::api::{AnimatedSprite, Area2D};
use gdnative::prelude::*;
use rand::{thread_rng, Rng};

#[derive(NativeClass, Debug, Default)]
#[inherit(Area2D)]
pub struct Ferris {
    #[property(default = 200.0)]
    speed: f32,
    screen_size: Vector2,
    /// Use random initial velocity if none was provided
    #[property]
    initial_velocity: Option<Vector2>,
    velocity: Vector2,
    /// The size of `ferris` sprite.
    /// Technically, this should be fetched by script, but I'm dumb and don't know how!
    #[property]
    sprite_size: Vector2,
}

impl Ferris {
    fn new(_base: &Area2D) -> Self {
        Ferris {
            speed: 200.0,
            sprite_size: Vector2::new(460., 307.),
            ..Default::default()
        }
    }
}

#[methods]
impl Ferris {
    #[method]
    fn _ready(&mut self, #[base] base: &Area2D) {
        let Some(vp_ref) = base.get_viewport() else {
            return;
        };
        let vp = unsafe { vp_ref.assume_safe() };
        vp.set_transparent_background(true);
        self.screen_size = vp.get_visible_rect().size;

        let animated_sprite = unsafe {
            base.get_node_as::<AnimatedSprite>("AnimatedSprite")
                .expect("No `AnimatedSprite` node attached to ferris")
        };
        animated_sprite.play("default", false);

        let anim_sprite_scale = animated_sprite.scale();
        self.sprite_size.x *= anim_sprite_scale.x;
        self.sprite_size.y *= anim_sprite_scale.y;

        if let Some(init_v) = self.initial_velocity {
            self.velocity = init_v;
        } else {
            self.velocity = Vector2::new(
                gen_non_zero_direction_float(),
                gen_non_zero_direction_float(),
            );
        }
    }

    #[method]
    fn _process(&mut self, #[base] base: &Area2D, delta: f32) {
        let mut velocity = self.velocity.normalized() * self.speed;

        let position = base.global_position() + velocity * delta;
        base.set_global_position(position);

        if position.y - (self.sprite_size.y / 2.0) <= 0.0
            || position.y + (self.sprite_size.y / 2.0) >= self.screen_size.y
        {
            velocity.y *= -1.0;
        }
        if position.x - (self.sprite_size.x / 2.0) <= 0.0
            || position.x + (self.sprite_size.x / 2.0) >= self.screen_size.x
        {
            velocity.x *= -1.0;
        }
        self.velocity = velocity;
    }
}

/// Generate a float from [-1.0, 1.0] (but except 0) with precision of 1.
///
/// Keep the precisio of 1 to keep a really small value being generated, such as
/// 0.0000001, which will make the main object bounce between two edges for a really long time.
fn gen_non_zero_direction_float() -> f32 {
    let mut rng = thread_rng();

    let mut raw_int: i32 = rng.gen_range(0..10);
    if rng.gen_bool(0.5) {
        raw_int *= -1;
    }
    raw_int as f32 / 10.0
}
