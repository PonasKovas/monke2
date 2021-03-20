use gdnative::api::{CollisionShape, HeightMapShape};
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(CollisionShape)]
pub struct HeightMapGenerator {
    #[property]
    heightmap: Option<Ref<Texture>>,
}

#[gdnative::methods]
impl HeightMapGenerator {
    fn new(_owner: &CollisionShape) -> Self {
        HeightMapGenerator { heightmap: None }
    }

    #[export]
    fn _ready(&self, owner: TRef<CollisionShape>) {
        // generate the heightmap

        // gotta read the raw image data
        if let Some(heightmap) = &self.heightmap {
            let image = unsafe { heightmap.assume_safe().get_data() };
            if let Some(image) = image {
                let image = unsafe { image.assume_safe() };
                let width = image.get_width();
                let height = image.get_height();
                let data = image.get_data();
                let data = data.read();

                // create a HeightMapShape
                let heightmapshape = HeightMapShape::new();
                heightmapshape.set_map_width(width);
                heightmapshape.set_map_depth(height);

                let mut heightmap = heightmapshape.map_data();
                let mut heightmap_write = heightmap.write();

                for i in 0..width * height {
                    heightmap_write.as_mut_slice()[i as usize] =
                        data.as_slice()[i as usize] as f32 / 255.0;
                }
                drop(heightmap_write);

                heightmapshape.set_map_data(heightmap);

                owner.set_shape(heightmapshape);
            }
        } else {
            godot_print!("No heightmap specified!");
        }
    }
}
