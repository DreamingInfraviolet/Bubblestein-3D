use std::path::Path;
use image;

pub struct Texture {
    texture : image::RgbaImage,
}

impl Texture {
    pub fn new(path : &Path) -> Result<Texture, image::ImageError> {
        let texture = try!(image::open(path));
        let texture = texture.to_rgba();
        Ok(Texture{texture: texture})
    }
    
    pub fn at(&self, x : u32, y : u32) -> u8 {
        self.texture.get_pixel(x, y)[0]
    }
}