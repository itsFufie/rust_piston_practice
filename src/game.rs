extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use input_box::InputBox;
use opengl_graphics::{GlGraphics, GlyphCache};
use piston::input::{ RenderArgs, UpdateArgs };
use piston::{ Button,  MouseButton };

static BLACK: [f32; 4] = [0.0, 0.0, 0.1, 1.0];
static _GREEN: [f32; 4] = [0.0, 0.6, 0.0, 1.0];
static GRAY: [f32; 4] = [0.5, 0.5, 0.5, 1.0];
static _RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
static _BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];


pub struct Game<'a> {
    pub gl: GlGraphics, // OpenGL drawing backend.
    pub rotation: f64, // Rotation for the square.
    pub reader: InputBox,
    pub cursor_pos: [f64; 2],
    pub glyph: GlyphCache<'a>,
}

impl Game<'_> {
    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        let reader = &self.reader;
        let ref mut glyph = self.glyph;
        let input_reader_coords = reader.coords;
        let pos = self.cursor_pos;

        self.gl.draw(args.viewport(), |c: Context, gl: &mut GlGraphics| {
            // Clear the screen.
            clear(GRAY, gl);
            // Draw text
            Text::new_color(BLACK, 32)
                .draw(&reader.value, glyph, &c.draw_state, c.transform.trans(20.0, 50.0), gl)
                .unwrap();

            // Draw the text box
            Rectangle::new_border(reader.color, 0.8).draw(
                input_reader_coords,
                &c.draw_state,
                c.transform.trans(0.0, 0.0),
                gl
            ); 

            Rectangle::new_border(BLACK, 1.0)
                .draw([0.0,0.0, 10.0, 10.0], &c.draw_state, c.transform.trans(pos[0],pos[1]), gl);

        });
    }

    pub fn update_text(&mut self, text: &String) {
        if self.reader.selected {
            self.reader.value.push_str(text);
        }
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt;
    }

    pub fn handle_press(&mut self, args: Button) {
        println!("{:?}", args);

        if args == Button::Mouse(MouseButton::Left) {
            let x = self.cursor_pos[0];
            let y = self.cursor_pos[1];
            if
                x >= self.reader.coords[0] &&
                x <= self.reader.coords[2] &&
                y >= self.reader.coords[1] &&
                y <= self.reader.coords[3]
            {
                self.reader.selected = true;
            } else if self.reader.selected {
                self.reader.selected = false;
            }
        }

        if args == Button::Keyboard(piston::Key::Backspace) {
            self.reader.value.pop();
        }
    }

    pub fn set_last_cursor_position(&mut self, pos: [f64; 2]) {
        self.cursor_pos = pos;
    }
}
