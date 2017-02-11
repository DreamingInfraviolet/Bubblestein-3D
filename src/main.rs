extern crate sdl2;
extern crate nalgebra as na;
mod retro_window;
mod util_math;
mod rendering_engine;

fn main() {
    let mut window = retro_window::RetroWindow::new("Bubblestein 3D", true).expect("Could not create window");

    loop {
        window.draw(|buffer: &mut [u8], pitch: usize, width : usize, height : usize| {
            for y in 0..height {
                for x in 0..width {
                    let offset = y*pitch + x*4;
                    buffer[offset + 0] = x as u8;
                    buffer[offset + 1] = y as u8;
                    buffer[offset + 2] = 0;
                }
            }
        });
        window.display();

        if !window.handle_events() {
            break;
        }
    }
}