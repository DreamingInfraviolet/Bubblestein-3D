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
mod player;
use scene::Scene;
use camera::Camera;
use texture::Texture;
use retro_window::BubbleEvent;
use std::time;
use player::Player;

fn main() {
    let texture = Texture::new(std::path::Path::new("src/art/world.png")).unwrap();
    let mut window = retro_window::RetroWindow::new("Bubblestein 3D", true).expect("Could not create window");
    let mut camera = Camera{position: na::Vector2{x: 0.0, y: 0.0}, orientation: 0.0, fov: 1.5708};
    let mut player = Player::new(camera);
    let mut scene = Scene{ worldmap: texture, player: player };
    
    loop {
        let start_time = time::SystemTime::now();
        
        let events = window.ennumerate_events();
        let mut done = false;

        for event in events {
            match event {
                BubbleEvent::Quit => done = true,
                BubbleEvent::MoveForward { speed_multiplier } => scene.player.add_movement_vector(na::Vector2{ x: speed_multiplier, y: 0.0 }),
                BubbleEvent::MoveRight { speed_multiplier } => scene.player.add_movement_vector(na::Vector2{ x: 0.0, y: speed_multiplier }),
            };
        }

        if done {
            break;
        }

        scene.player.walk();

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

        let finish_time = time::SystemTime::now();
        
        // Sleep to maintain 60hz
        let desired_delta_time = time::Duration::from_millis((1.0 / 60.0 * 1000.0) as u64);
        let actual_delta_time  = finish_time.duration_since(start_time);
        match actual_delta_time {
            Ok(time) => std::thread::sleep(time),
            _ => () 
        }

    }
}