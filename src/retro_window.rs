use sdl2;
use std::error::Error;
use util_math::Size;
use std::collections::HashSet;
use sdl2::keyboard::Keycode;

pub struct RetroWindow<'a> {
    sdl_context : sdl2::Sdl,
    renderer : sdl2::render::Renderer<'a>,
    window_texture : sdl2::render::Texture,
    buffer_size : Size,
}

pub enum BubbleEvent {
    Quit,
    MoveForward { speed_multiplier : f64 },
    MoveRight { speed_multiplier : f64 },
    TurnRight { speed_multiplier : f64 },
    }

/* Multiples the resolution by a positive integer so that it fits within the maximum bounds */
fn upsize_resolution(initial : Size, max : Size) -> Size {
    initial * (max / initial)
}

impl<'a> RetroWindow<'a> {
    pub fn new(title : &str, upsize : bool) -> Result<RetroWindow, String> {
        let buffer_size = Size { w:320, h:200 };

        // Decide on actual window display resolution
        let window_size =   if upsize
                            { upsize_resolution(buffer_size, Size { w : 1900, h : 1200 }) }
                            else
                            { buffer_size };

        // Create SDL things
        let sdl_context  = try!(sdl2::init());
        let video_system = try!(sdl_context.video());
        let window       = match video_system.window(title, window_size.w, window_size.h)
                            .position_centered()
                            .opengl()
                            .build() {
            Ok(window) => window,
            Err(e)     => return Err(e.description().to_string()),
        };

        // For type reasons, this consumes the window.
        // Use Renderer::get_parent() to get the window back.
        let renderer = match window.renderer().build() {
            Ok(renderer) => renderer,
            Err(e)       => return Err(e.description().to_string()),
        };

        // Create texture to draw to
        let pixel_format       = sdl2::pixels::PixelFormatEnum::RGBA8888;
        let window_texture = match renderer.create_texture_streaming(pixel_format, buffer_size.w, buffer_size.h) {
            Ok(texture)        => texture,
            Err(e)             => return Err(e.description().to_string()),
        };

        Ok(RetroWindow {sdl_context:sdl_context,
                        renderer:renderer,
                        window_texture:window_texture,
                        buffer_size:buffer_size})
    }

    pub fn draw<F>(&mut self, draw_fn : F) where F : Fn(&mut [u8], usize, usize, usize) -> () {
        let height = self.buffer_size.h as usize;
        let width  = self.buffer_size.w as usize;
        let meta   = |a : &mut [u8], b : usize| -> () { draw_fn(a, b, width, height) }; 
        self.window_texture.with_lock(None, meta).unwrap();
    }

    pub fn display(&mut self) {
        self.renderer.clear();
        self.renderer.copy(&self.window_texture, None, None).unwrap();
        self.renderer.present();
    }

    pub fn ennumerate_events(&mut self) -> Vec<BubbleEvent> {
        use sdl2::event::Event;
        use sdl2::keyboard::Keycode;
        let mut events = Vec::new();
        
        for event in self.sdl_context.event_pump().unwrap().poll_iter() {
            match event {
                Event::KeyDown{ keycode: Some(keycode), ..} => {
                    if keycode == Keycode::Escape { events.push(BubbleEvent::Quit); }
                }
                Event::Quit{..} => events.push(BubbleEvent::Quit),
                _ => ()
            }
        }

        // Handle continuous events
        let keyboard_state = self.keyboard_state();
        if keyboard_state.contains(&Keycode::W) {
            events.push(BubbleEvent::MoveForward { speed_multiplier: 1.0 });
        }
        if keyboard_state.contains(&Keycode::A) {
            events.push(BubbleEvent::MoveRight { speed_multiplier: -1.0 });
        }
        if keyboard_state.contains(&Keycode::S) {
            events.push(BubbleEvent::MoveForward { speed_multiplier: -1.0 });
        }
        if keyboard_state.contains(&Keycode::D) {
            events.push(BubbleEvent::MoveRight { speed_multiplier: 1.0 });
        }
        if keyboard_state.contains(&Keycode::Right) {
            events.push(BubbleEvent::TurnRight { speed_multiplier: 1.0 });
        }
        if keyboard_state.contains(&Keycode::Left) {
            events.push(BubbleEvent::TurnRight { speed_multiplier: -1.0 });
        }

        events
    }

    fn keyboard_state(&self) -> HashSet<Keycode> {
        self.sdl_context.event_pump().unwrap().keyboard_state().pressed_scancodes().filter_map(Keycode::from_scancode).collect()
     }
}