extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::{GlutinWindow, OpenGL};
//use input_box::InputBox;
use opengl_graphics::{Filter, GlGraphics, GlyphCache, TextureSettings};
use piston::input::{ RenderArgs, UpdateArgs };
use piston::{  Button, EventLoop, EventSettings, Events, MouseCursorEvent, PressEvent, RenderEvent, UpdateEvent, WindowSettings };

use game_objects::GameObject;

use crate::game_box::PhysicsBox;
use crate::game_objects::Drawable;


static BLACK: [f32; 4] = [0.0, 0.0, 0.1, 1.0];
static _GREEN: [f32; 4] = [0.0, 0.6, 0.0, 1.0];
static GRAY: [f32; 4] = [0.5, 0.5, 0.5, 1.0];
static _RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
static _BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];



pub struct Game<'a> {
    pub gl: GlGraphics, // OpenGL drawing backend.
    pub cursor_pos: [f64; 2],
    pub glyph: GlyphCache<'a>,
    pub window: GlutinWindow,
    pub game_objects:  Vec<GameObject>,
}

impl Game<'_> {

    pub fn new() -> Game<'static> {
        
        let graphics = OpenGL::V3_2;

        let window: GlutinWindow = WindowSettings::new("Calculator", [200, 200])
        .graphics_api(graphics)
        .exit_on_esc(true)
        .build()    
        .unwrap();

        // font delcaring
        let font: &str = "/usr/share/fonts/TTF/DejaVuSansMono.ttf";
        let texture_settings = TextureSettings::new().filter(Filter::Linear);
        let glyphs = GlyphCache::new(font, (), texture_settings).expect(
            &format!("failed to load font `{}`", font)
        );

        Game {
            gl: GlGraphics::new(graphics),
            cursor_pos: [0.0, 0.0],
            glyph: glyphs,
            window,
            game_objects: vec!(),
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;
        let pos = self.cursor_pos;
        let objects = &mut self.game_objects;

        self.gl.draw(args.viewport(), |c: Context, gl: &mut GlGraphics| {
            // Clear the screen.
            clear(GRAY, gl);
            
            for object in objects {
                match object {
                    GameObject::Box(object) => todo!(),
                    GameObject::Circle(object) => object.draw(c, gl),
                }
            }

            /* Rectangle::new_border(BLACK, 1.0)
                .draw([0.0,0.0, 10.0, 10.0], &c.draw_state, c.transform.trans(pos[0],pos[1]), gl); */
        });
    }

    pub fn start_objects(&mut self) {
        let circle = graphics::Ellipse::new(BLACK);
        let physics_box = GameObject::Circle(PhysicsBox::new(true, circle, [0.0, 0.0, 20.0, 20.0]));
        self.game_objects.push(physics_box);
    }

    pub fn game_loop(&mut self) {

        self.start_objects();
        let mut events = Events::new(EventSettings::new().ups(60));
         // Iterating through the Events: render event, update event, input event, etc
        while let Some(e) = events.next(&mut self.window) {

            // Render event
            if let Some(args) = e.render_args() {
                self.render(&args);
            }

            // Update event
            if let Some(args) = e.update_args() {
                self.update(&args);
            }

            if let Some(args) = e.press_args() {
                self.handle_press(args);
            }

            if let Some(pos) = e.mouse_cursor_args() {
                self.set_last_cursor_position(pos);
            }

        }
    }


    pub fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        println!("{:?}", args);
    }

    pub fn handle_press(&mut self, args: Button) {
        println!("{:?}", args);
    }

    pub fn set_last_cursor_position(&mut self, pos: [f64; 2]) {
        self.cursor_pos = pos;
    }
}
