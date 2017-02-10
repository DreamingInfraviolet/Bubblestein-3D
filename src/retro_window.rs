use piston_window::*;

pub struct RetroWindow {
    window : PistonWindow,
}

impl RetroWindow {

    pub fn new(title : &str, upsize : bool) -> Result<RetroWindow, String> {
        let window = try!(WindowSettings::new(title, [320, 200]).build());
        Ok(RetroWindow { window:window })
    }

    pub fn draw(&mut self) {
        while let Some(e) = self.window.next() {
            self.window.draw_2d(&e, |c, g| {
                clear([1.0; 4], g);
                rectangle([1.0, 0.0, 0.0, 1.0], // red
                        [0.0, 0.0, 100.0, 100.0],
                        c.transform, g);
            });
        }
    }

}