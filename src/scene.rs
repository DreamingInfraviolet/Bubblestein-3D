use texture::Texture;
use player::Player;
use camera::Camera;

pub struct Scene {
    // textures : Vec<Texture>,
    pub worldmap : Texture,
    pub player   : Player,
}

impl Scene {
    pub fn camera(&self) -> &Camera {
        &self.player.camera
    }
}