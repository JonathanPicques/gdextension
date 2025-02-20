use godot::engine::{AnimatedSprite2D, Area2D, CollisionShape2D, PhysicsBody2D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct Player {
    speed: f32,
    screen_size: Vector2,

    #[base]
    base: Base<Area2D>,
}

#[godot_api]
impl Player {
    #[signal]
    fn hit();

    #[func]
    fn on_player_body_entered(&mut self, _body: Gd<PhysicsBody2D>) {
        self.base.hide();
        self.base.emit_signal("hit".into(), &[]);

        let mut collision_shape = self
            .base
            .get_node_as::<CollisionShape2D>("CollisionShape2D");

        collision_shape.set_deferred("disabled".into(), true.to_variant());
    }

    #[func]
    pub fn start(&mut self, pos: Vector2) {
        self.base.set_global_position(pos);
        self.base.show();

        let mut collision_shape = self
            .base
            .get_node_as::<CollisionShape2D>("CollisionShape2D");

        collision_shape.set_disabled(false);
    }
}

#[godot_api]
impl GodotExt for Player {
    fn init(base: Base<Area2D>) -> Self {
        Player {
            speed: 400.0,
            screen_size: Vector2::new(0.0, 0.0),
            base,
        }
    }

    fn ready(&mut self) {
        let viewport = self.base.get_viewport_rect();
        self.screen_size = viewport.size();
        self.base.hide();
    }

    fn process(&mut self, delta: f64) {
        let mut animated_sprite = self
            .base
            .get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");

        let mut velocity = Vector2::new(0.0, 0.0).inner();

        // Note: exact=false by default, in Rust we have to provide it explicitly
        let input = Input::singleton();
        if input.is_action_pressed("ui_right".into(), false) {
            velocity.x += 1.0;
        }
        if input.is_action_pressed("ui_left".into(), false) {
            velocity.x -= 1.0;
        }
        if input.is_action_pressed("ui_down".into(), false) {
            velocity.y += 1.0;
        }
        if input.is_action_pressed("ui_up".into(), false) {
            velocity.y -= 1.0;
        }

        if velocity.length() > 0.0 {
            velocity = velocity.normalize() * self.speed;

            let animation;

            if velocity.x != 0.0 {
                animation = "right";

                animated_sprite.set_flip_v(false);
                animated_sprite.set_flip_h(velocity.x < 0.0)
            } else {
                animation = "up";

                animated_sprite.set_flip_v(velocity.y > 0.0)
            }

            animated_sprite.play(animation.into(), false);
        } else {
            animated_sprite.stop();
        }

        let change = velocity * delta as f32;
        let position = self.base.get_global_position().inner() + change;
        let position = Vector2::new(
            position.x.max(0.0).min(self.screen_size.inner().x),
            position.y.max(0.0).min(self.screen_size.inner().y),
        );
        self.base.set_global_position(position);
    }
}
