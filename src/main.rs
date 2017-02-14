extern crate image;
extern crate sdl2;
extern crate nalgebra as na;
mod retro_window;
mod util_math;
mod rendering_engine;
mod scene;
mod camera;
mod texture;    
mod raycast;
use scene::Scene;
use camera::Camera;
use texture::Texture;

fn main() {
    let mut window = retro_window::RetroWindow::new("Bubblestein 3D", true).expect("Could not create window");
    let camera = Camera{position: na::Vector2{x: 0.0, y: 0.0}, orientation: 0.0, fov: 1.5708};
    let texture = Texture::new(std::path::Path::new("src/art/world.png")).unwrap();
    let scene = Scene::new(texture, camera);

    loop {
        window.draw(|buffer: &mut [u8], pitch: usize, width : usize, height : usize| {
            rendering_engine::draw(&scene, buffer, pitch, width, height);
            // for y in 0..height {
            //     for x in 0..width {
            //         let offset = y*pitch + x*4;
            //         buffer[offset + 0] = x as u8;
            //         buffer[offset + 1] = y as u8;
            //         buffer[offset + 2] = 0;
            //     }
            // }
        });
        window.display();

        if !window.handle_events() {
            break;
        }
    }
}