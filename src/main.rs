extern crate piston_window;
mod retro_window;

fn main() {
    let mut window = match retro_window::RetroWindow::new("Bubblestein 3D", true) {
        Ok(window) => window,
        Err(msg)   => {
            println!("{}", msg);
            panic!("Aaaah")
        }
    };

    window.draw();
}