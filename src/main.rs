extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate freetype as ft;

mod input_box;

use glutin_window::GlutinWindow;
use input_box::InputBox;
use opengl_graphics::{ Filter, GlGraphics, GlyphCache, OpenGL, TextureSettings};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::{Window, WindowSettings};
use piston::{Button, ButtonEvent, EventLoop, MouseButton, MouseCursorEvent, PressEvent, TextEvent};

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    rotation: f64, // Rotation for the square.
    text: String,
    reader: InputBox,
    cursor_pos: [f64; 2],
}

static BLACK: [f32; 4] = [0.0, 0.0, 0.1, 1.0];

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const _GREEN: [f32; 4] = [0.0, 0.6, 0.0, 1.0];
        const GRAY: [f32; 4] = [0.5, 0.5, 0.5, 1.0];
        const _RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const _BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

        let font: &str = "/usr/share/fonts/TTF/DejaVuSansMono.ttf";
        let freetype = ft::Library::init().unwrap();
        let face = freetype.new_face(&font, 0).unwrap();
        let _ = face.set_pixel_sizes(0, 48);

        let texture_settings = TextureSettings::new().filter(Filter::Nearest);
        let ref mut glyphs = GlyphCache::new(font, (), texture_settings)
        .expect(&format!("failed to load font `{}`", font));
        let reader = &self.reader;
        let input_reader_coords = reader.coords;
        let (x_cords, y_cords) = (20.0, 50.0);

        self.gl.draw(args.viewport(), |c: Context, gl: &mut GlGraphics| {
            // Clear the screen.
            clear(GRAY, gl);
            if reader.selected {
                Text::new_color(BLACK, 32).draw(&reader.value, glyphs, &c.draw_state, c.transform.trans(x_cords, y_cords), gl).unwrap();
            }
            Rectangle::new_border(reader.color, 1.0)
                .draw(input_reader_coords, &c.draw_state, c.transform.trans(0.0, 0.0), gl);
        });
    }

    fn update_text(&mut self, text: &String) {
        if self.reader.selected {
            self.reader.value.push_str(text);
        }
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt;
    }

    fn handle_press(&mut self, args: Button) {
        if args == Button::Mouse(MouseButton::Left) {
            let x = self.cursor_pos[0];
            let y = self.cursor_pos[1];
            if x >= self.reader.coords[0] && x <= self.reader.coords[2] && y >= self.reader.coords[1] && y <= self.reader.coords[3] {
                self.reader.selected = !self.reader.selected;
        }

    }}

    fn set_last_cursor_position(&mut self, pos: [f64; 2]) {
            self.cursor_pos = pos;
    }

}


// The main function that every Rust application requires!
fn main() {

    // Declarating the OpenGL Graphics
    let opengl = OpenGL::V3_2;

    // Create a Glutin window.
    let mut window: GlutinWindow = WindowSettings::new("Calculator", [200, 200])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();
    
    let input_reader_coords = [20.0, 20.0, window.size().width - 40.0, 60.0];
    let mut input_box = input_box::InputBox {
        coords: input_reader_coords,
        selected: false,
        value: String::from(""),
        color: BLACK,
    };


    // Instanciating the App struct
    let mut app = App {
        gl: GlGraphics::new(opengl),
        rotation: 0.0,
        text: String::from(""),
        reader: input_box,
        cursor_pos: [0.0, 0.0],
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

        if let Some(args) = e.press_args() {
            app.handle_press(args);
        }

        if let Some(pos) = e.mouse_cursor_args() {
            app.set_last_cursor_position(pos);
        }
        // Text Input Event
        if let Some(text) = e.text_args() {
            app.update_text(&text)
        }
    }
}
