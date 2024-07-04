extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

mod input_box;
mod game;

use game::Game;
use glutin_window::GlutinWindow;
use opengl_graphics::{ Filter, GlGraphics, GlyphCache, OpenGL, TextureSettings };
use piston::event_loop::{ EventSettings, Events };
use piston::input::{ RenderEvent, UpdateEvent };
use piston::window::{ Window, WindowSettings };
use piston::{  EventLoop, MouseCursorEvent, PressEvent, TextEvent };



static BLACK: [f32; 4] = [0.0, 0.0, 0.1, 1.0];
static _GREEN: [f32; 4] = [0.0, 0.6, 0.0, 1.0];
static _GRAY: [f32; 4] = [0.5, 0.5, 0.5, 1.0];
static _RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
static _BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];


// The main function that every Rust application requires!
fn main() {
    // Declarating the OpenGL Graphics
    let graphics = OpenGL::V3_2;

    // Create a Glutin window.
    let mut window: GlutinWindow = WindowSettings::new("Calculator", [200, 200])
        .graphics_api(graphics)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let input_reader_coords = [20.0, 20.0, window.size().width - 40.0, 60.0];
    let input_box = input_box::InputBox {
        coords: input_reader_coords,
        selected: false,
        value: String::from(""),
        color: BLACK,
    };


    let font: &str = "/usr/share/fonts/TTF/DejaVuSansMono.ttf";
    let texture_settings = TextureSettings::new().filter(Filter::Nearest);
    let glyphs = GlyphCache::new(font, (), texture_settings).expect(
        &format!("failed to load font `{}`", font)
    );

    // Instanciating the App struct
    let mut game = Game {
        gl: GlGraphics::new(graphics),
        rotation: 0.0,
        reader: input_box,
        cursor_pos: [0.0, 0.0],
        glyph: glyphs,
    };

    // Getting the Events data
    let mut events = Events::new(EventSettings::new().ups(60));

    // Iterating through the Events: render event, update event, input event, etc
    while let Some(e) = events.next(&mut window) {
        // Render event
        if let Some(args) = e.render_args() {
            game.render(&args);
        }

        // Update event
        if let Some(args) = e.update_args() {
            game.update(&args);
        }

        if let Some(args) = e.press_args() {
            game.handle_press(args);
        }

        if let Some(pos) = e.mouse_cursor_args() {
            game.set_last_cursor_position(pos);
        }
        // Text Input Event
        if let Some(text) = e.text_args() {
            game.update_text(&text);
        }
    }
}
