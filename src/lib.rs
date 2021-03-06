use gdnative::api::VideoPlayer;
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(VideoPlayer)]
struct MainMenuBackground;

#[gdnative::methods]
impl MainMenuBackground {
    fn new(_owner: &VideoPlayer) -> Self {
        MainMenuBackground
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

fn init(handle: InitHandle) {
    handle.add_class::<MainMenuBackground>();
}

godot_init!(init);
