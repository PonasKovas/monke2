use gdnative::prelude::*;

mod heightmap;
mod main_menu;
mod player;

fn init(handle: InitHandle) {
    handle.add_class::<main_menu::MainMenuBackground>();
    handle.add_class::<main_menu::MainMenuButton>();

    handle.add_class::<heightmap::HeightMapGenerator>();

    handle.add_class::<player::Player>();
}

godot_init!(init);
