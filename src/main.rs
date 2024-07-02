extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate freetype as ft;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ Filter, GlGraphics, GlyphCache, OpenGL, TextureSettings};
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

        const _GREEN: [f32; 4] = [0.0, 0.6, 0.0, 1.0];
        const GRAY: [f32; 4] = [0.5, 0.5, 0.5, 1.0];
        const _RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const _BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
        const BLACK: [f32; 4] = [0.0, 0.0, 0.1, 1.0];

        let my_input: &str = &self.text;
        let font: &str = "/usr/share/fonts/TTF/DejaVuSansMono.ttf";
        let freetype = ft::Library::init().unwrap();
        let face = freetype.new_face(&font, 0).unwrap();
        let _ = face.set_pixel_sizes(0, 48);

        let texture_settings = TextureSettings::new().filter(Filter::Nearest);
        let ref mut glyphs = GlyphCache::new(font, (), texture_settings)
        .expect(&format!("failed to load font `{}`", font));


        let (x_cords, y_cords) = (20.0, 50.0);

        self.gl.draw(args.viewport(), |c: Context, gl: &mut GlGraphics| {
            // Clear the screen.
            clear(GRAY, gl);
            Text::new_color(BLACK, 32).draw(my_input, glyphs, &c.draw_state, c.transform.trans(x_cords, y_cords), gl).unwrap();
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


// The main function that every Rust application requires!
fn main() {

    // Declarating the OpenGL Graphics
    let opengl = OpenGL::V3_2;

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new("Calculator", [200, 200])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Instanciating the App struct
    let mut app = App {
        gl: GlGraphics::new(opengl),
        rotation: 0.0,
        text: String::from(""),
    };

    // Getting the Events data
    let mut events = Events::new(EventSettings::new().lazy(true));

    // Iterating through the Events: render event, update event, input event, etc
    while let Some(e) = events.next(&mut window) {

        // Render event
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        // Update event
        if let Some(args) = e.update_args() {
            app.update(&args);
        }
        // Text Input Event
        if let Some(text) = e.text_args() {
            app.update_text(&text)
        }
    }
}
