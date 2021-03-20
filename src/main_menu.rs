use gdnative::api::{AudioStreamPlayer, Label, VideoPlayer};
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(VideoPlayer)]
pub struct MainMenuBackground {}

#[gdnative::methods]
impl MainMenuBackground {
    fn new(_owner: &VideoPlayer) -> Self {
        MainMenuBackground {}
    }

    #[export]
    fn _ready(&self, owner: TRef<VideoPlayer>) {
        owner
            .connect(
                "finished",
                owner,
                "_on_finished",
                VariantArray::new_shared(),
                0,
            )
            .unwrap();
    }

    #[export]
    fn _on_finished(&self, owner: &VideoPlayer) {
        owner.play();
    }
}

#[derive(NativeClass)]
#[inherit(Label)]
pub struct MainMenuButton {
    #[property]
    click_sound_player: Option<NodePath>,
    #[property]
    music_player: Option<NodePath>,
    #[property]
    video_player: Option<NodePath>,
    #[property]
    play_scene: Option<Ref<PackedScene>>,
    hovering: bool,
    pressed: bool,
    exit_time: f64,
}

#[gdnative::methods]
impl MainMenuButton {
    fn new(_owner: &Label) -> Self {
        MainMenuButton {
            click_sound_player: None,
            music_player: None,
            video_player: None,
            play_scene: None,
            hovering: false,
            pressed: false,
            exit_time: -1.0,
        }
    }

    #[export]
    fn _ready(&self, owner: TRef<Label>) {
        owner
            .connect(
                "mouse_entered",
                owner,
                "_on_mouse_entered",
                VariantArray::new_shared(),
                0,
            )
            .unwrap();
        owner
            .connect(
                "mouse_exited",
                owner,
                "_on_mouse_exited",
                VariantArray::new_shared(),
                0,
            )
            .unwrap();
    }

    #[export]
    fn _on_mouse_entered(&mut self, owner: TRef<Label>) {
        if let Some(click_sound_player) = self.click_sound_player.as_ref() {
            let node = unsafe {
                owner
                    .get_node(click_sound_player.to_godot_string())
                    .unwrap()
                    .assume_safe()
            };

            node.cast::<AudioStreamPlayer>().unwrap().play(0.0);

            owner.add_color_override("font_color", Color::rgb(0.8, 0.8, 0.8));
        }

        self.hovering = true;
    }

    #[export]
    fn _on_mouse_exited(&mut self, owner: TRef<Label>) {
        owner.add_color_override("font_color", Color::rgb(1.0, 1.0, 1.0));

        self.hovering = false;
    }

    #[export]
    fn _process(&mut self, owner: TRef<Label>, delta: f64) {
        if self.exit_time > -1.0 {
            if self.exit_time > 1.0 {
                unsafe {
                    owner.get_tree().unwrap().assume_safe().quit(0);
                }
            }

            self.exit_time += delta;

            // audio distortion
            if let Some(music_player) = self.music_player.as_ref() {
                let node = unsafe {
                    owner
                        .get_node(music_player.to_godot_string())
                        .unwrap()
                        .assume_safe()
                };

                node.cast::<AudioStreamPlayer>()
                    .unwrap()
                    .set_pitch_scale(0.9 - 0.2 * self.exit_time);
            }
            // fade out video
            if let Some(video_player) = self.video_player.as_ref() {
                let node = unsafe {
                    owner
                        .get_node(video_player.to_godot_string())
                        .unwrap()
                        .assume_safe()
                };

                node.cast::<VideoPlayer>().unwrap().set_modulate(Color::rgb(
                    1.0 - self.exit_time as f32,
                    1.0 - self.exit_time as f32,
                    1.0 - self.exit_time as f32,
                ));
            }
        }

        let input = Input::godot_singleton();

        if self.hovering && input.is_mouse_button_pressed(1) {
            if !self.pressed {
                let text = format!("{}", owner.text());
                if &text == "Exit" && self.exit_time == -1.0 {
                    self.exit_time = 0.0;
                }
                if &text == "Play" && self.exit_time == -1.0 {
                    // change scene
                    if let Some(scene) = &self.play_scene {
                        unsafe {
                            owner
                                .get_tree()
                                .unwrap()
                                .assume_safe()
                                .change_scene_to(scene)
                                .unwrap();
                        }
                    }
                }
            }
            self.pressed = true;
        } else {
            self.pressed = false;
        }
    }
}
