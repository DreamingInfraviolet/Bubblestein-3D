extern crate sdl2;
extern crate nalgebra as na;
mod retro_window;
mod util_math;

fn main() {
    let mut window = retro_window::RetroWindow::new("Bubblestein 3D", true).expect("Could not create window");

    loop {
        window.draw();
        window.display();
    }
}