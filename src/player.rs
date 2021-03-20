use gdnative::api::{Camera, InputEventMouseMotion};
use gdnative::prelude::*;

const JUMP_FORCE: f32 = 10.0;
const MOUSE_SENSITIVITY: f32 = 0.3;

#[derive(NativeClass)]
#[inherit(KinematicBody)]
pub struct Player {
    y_vel: f32,
    walking_speed: f32,
}

#[gdnative::methods]
impl Player {
    fn new(_owner: &KinematicBody) -> Self {
        Player {
            y_vel: 0.0,
            walking_speed: 10.0,
        }
    }

    #[export]
    fn _ready(&self, _owner: TRef<KinematicBody>) {
        // hide the cursor
        Input::godot_singleton().set_mouse_mode(2);
    }

    #[export]
    fn _input(&self, owner: TRef<KinematicBody>, event: Ref<InputEvent>) {
        if let Some(mouse_movement) = unsafe { event.assume_safe() }.cast::<InputEventMouseMotion>()
        {
            let camera = unsafe {
                owner
                    .get_node("Head")
                    .unwrap()
                    .assume_safe()
                    .cast::<Camera>()
                    .unwrap()
            };

            let mut rotation = camera.rotation();

            // vertical angle
            let vangle_change =
                -mouse_movement.relative().y as f32 * MOUSE_SENSITIVITY * std::f32::consts::PI
                    / 180.0;
            let current_vangle = camera.rotation().x as f32;
            if current_vangle + vangle_change > -1.39626 && current_vangle + vangle_change < 1.39626
            {
                rotation.x += vangle_change;
            }

            // horizontal
            rotation.y +=
                -mouse_movement.relative().x as f32 * MOUSE_SENSITIVITY * std::f32::consts::PI
                    / 180.0;

            camera.set_rotation(rotation);
        }
    }

    #[export]
    fn _physics_process(&mut self, owner: TRef<KinematicBody>, _delta: f64) {
        let mut movement = Vector3::new(0.0, 0.0, 0.0);

        // apply gravity
        if !owner.is_on_floor() {
            self.y_vel -= 0.5;
        } else if self.y_vel < 0.0 {
            self.y_vel = 0.0;
        }

        // apply player input
        let input = Input::godot_singleton();

        // walking
        if input.is_action_pressed("forward") {
            movement.z -= 1.0;
        }
        if input.is_action_pressed("backward") {
            movement.z += 1.0;
        }
        if input.is_action_pressed("left") {
            movement.x -= 1.0;
        }
        if input.is_action_pressed("right") {
            movement.x += 1.0;
        }

        if movement.length() > 0.0 {
            movement = movement.normalize();

            // apply the camera rotation
            let angle = unsafe {
                owner
                    .get_node("Head")
                    .unwrap()
                    .assume_safe()
                    .cast::<Camera>()
                    .unwrap()
                    .rotation()
                    .y
            };

            let new_z = -movement.x * angle.sin() + movement.z * angle.cos();
            let new_x = movement.x * angle.cos() + movement.z * angle.sin();
            movement.x = new_x;
            movement.z = new_z;

            movement = movement * self.walking_speed;
        }

        // jumping
        if input.is_action_pressed("space") && owner.is_on_floor() {
            self.y_vel += JUMP_FORCE;
            if self.walking_speed < 40.0 {
                self.walking_speed *= 1.05;
            }
        } else if owner.is_on_floor() {
            self.walking_speed = 10.0;
        }

        movement.y += self.y_vel;

        owner.move_and_slide(
            movement,                    // velocity
            Vector3::new(0.0, 1.0, 0.0), // up_direction
            false,                       // stop_on_slope
            4,                           // max_slides
            0.785398,                    // floor_max_angle
            true,                        // infinite_inertia
        );
    }
}
