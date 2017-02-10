use sdl2;
use std::error::Error;
use util_math::Size;

pub struct RetroWindow<'a> {
    sdl_context : sdl2::Sdl,
    video_system : sdl2::VideoSubsystem,
    renderer : sdl2::render::Renderer<'a>,
    window_texture : sdl2::render::Texture,
    buffer_size : Size,
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
                        video_system:video_system,
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

}