extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;


use glutin_window::GlutinWindow as Window;
use opengl_graphics::{Filter, GlGraphics, GlyphCache, OpenGL, TextureSettings};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use piston::{EventLoop, TextEvent};


pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    rotation: f64, // Rotation for the square.
    text: String, 
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 0.6, 0.0, 1.0];
        const _RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const _BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
        const BLACK: [f32; 4] = [0.0, 0.0, 0.1, 1.0];

        let my_input: &str = &self.text;
        let font: &str = "/usr/share/fonts/TTF/DejaVuSansMono.ttf";
        let (mut x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

        self.gl.draw(args.viewport(), |c: Context, gl: &mut GlGraphics| {
            // Clear the screen.
            clear(GREEN, gl);

            // Draw a box rotating around the middle of the screen.
            let texture_settings = TextureSettings::new().filter(Filter::Linear);

            let mut glyphs = GlyphCache::new(font, (), texture_settings)
                .expect(&format!("failed to load font `{:?}`", font));
            
            let text_image = Image::new_color(BLACK);

            for ch in my_input.chars() {
                x += 5.0;
                if let Ok(character) = glyphs.character(32, ch) {
                    let text_image = text_image.src_rect([
                        character.atlas_offset[0],
                        character.atlas_offset[1],
                        character.atlas_size[0],
                        character.atlas_size[1],
                    ]);
                    text_image.draw(character.texture, &c.draw_state, c.transform.trans(x,y), gl);
                }
            };
        });
    }

    fn update_text(&mut self, text: &String) {
        self.text.push_str(text);
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt;
    }
}



fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new("Calculator", [200, 200])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        rotation: 0.0,
        text: String::from(""),
    };

    let mut events = Events::new(EventSettings::new().lazy(true));
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }

        if let Some(text) = e.text_args() {
            app.update_text(&text)
        }
    }
}
