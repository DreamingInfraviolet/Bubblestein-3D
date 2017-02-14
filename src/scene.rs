use texture::Texture;
use camera::Camera;

pub struct Scene {
    // textures : Vec<Texture>,
    pub worldmap : Texture,
    pub camera   : Camera,
}

impl Scene {

    pub fn new(texture : Texture, camera: Camera) -> Scene {
        Scene { worldmap: texture, camera: camera}
    }
}