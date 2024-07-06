extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

use glutin_window::{GlutinWindow, OpenGL};
//use input_box::InputBox;
use opengl_graphics::{Filter, GlGraphics, GlyphCache, Texture, TextureSettings};
use piston::input::{ RenderArgs, UpdateArgs };
use piston::{ Button, EventLoop, EventSettings, Events, MouseCursorEvent, PressEvent, RenderEvent, UpdateEvent, WindowSettings };

use game_objects::GameObject;
use self::rand::Rng;

use crate::game_box::PhysicsBox;
use crate::tile::Tile;


static BLACK: [f32; 4] = [0.0, 0.0, 0.1, 1.0];
static _GREEN: [f32; 4] = [0.0, 0.6, 0.0, 1.0];
static GRAY: [f32; 4] = [0.5, 0.5, 0.5, 1.0];
static RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
static BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];



pub struct Game<'a> {
    pub gl: GlGraphics, // OpenGL drawing backend.
    pub cursor_pos: [f64; 2],
    pub glyph: GlyphCache<'a>,
    pub window: GlutinWindow,
    pub grid: graphics::grid::Grid,
    pub bombs: Vec<(u32, u32)>,
    pub tiles: Vec<Tile>
}

impl Game<'_> {

    pub fn new() -> Game<'static> {
        
        let graphics = OpenGL::V3_2;

        let window: GlutinWindow = WindowSettings::new("Calculator", [800, 600])
        .graphics_api(graphics)
        .exit_on_esc(true)
        .build()    
        .unwrap();

        // font delcaring
        let font: &str = "/usr/share/fonts/TTF/DejaVuSansMono.ttf";
        let texture_settings = TextureSettings::new().filter(Filter::Nearest);
        let glyphs = GlyphCache::new(font, (), texture_settings).expect(
            &format!("failed to load font `{}`", font)
        );

        Game {
            gl: GlGraphics::new(graphics),
            cursor_pos: [0.0, 0.0],
            glyph: glyphs,
            window,
            grid: graphics::grid::Grid {
                cols: 16,
                rows: 16,
                units: 32.0
            }, 
            bombs: vec!(),
            tiles: vec!()
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;
        let pos = self.cursor_pos;
        let grid = self.grid;
        let bombs = &self.bombs;
        let tiles = &self.tiles;
        let ref mut glyphs = self.glyph;

        self.gl.draw(args.viewport(), |c: Context, gl: &mut GlGraphics| {
            // Clear the screen.
            clear(GRAY, gl);

            grid.draw(&Line::new(BLACK, 1.0), &c.draw_state, c.transform.trans(40.0, 40.0), gl);
            
            for tile in tiles {
                if tile.clicked {
                    let pos = grid.cell_position(tile.pos);
                    Text::new_color(BLUE, 24).draw(&tile.bombs_around.to_string(), glyphs, &c.draw_state, c.transform.trans(pos[0] + 44.0, pos[1] + 28.0 + 40.0),gl).unwrap();
                }
            }

            for bomb in bombs {
                let square = Rectangle::new_border(RED, 1.0);
                let cell_x = grid.x_pos(*bomb);
                let cell_y = grid.y_pos(*bomb);
                let rect = [cell_x, cell_y, 32.0, 32.0];          
                square.draw(rect, &c.draw_state, c.transform.trans(40.0, 40.0), gl);
            }  
        })
    }

    pub fn start_objects(&mut self) {
        let circle = graphics::Ellipse::new(BLACK);
        let physics_box = GameObject::Circle(PhysicsBox::new(true, circle, [0.0, 0.0, 20.0, 20.0]));
        let mut bombs: Vec<(u32, u32)> = vec!();
        while bombs.len() < 24 {
            let num_x = rand::thread_rng().gen_range(0..16) as u32;
            let num_y = rand::thread_rng().gen_range(0..16) as u32;
            if !bombs.contains(&(num_x, num_y)) {
                bombs.push((num_x, num_y));
            }
        }
        for i in 0..16 {
            for j in 0..16 {
                if !bombs.contains(&(i, j)) {
                    let mut tile = Tile {
                        pos: (i, j),
                        bombs_around: 0,
                        clicked: false,
                    };
                    tile.count_bombs(&bombs);
                    self.tiles.push(tile);
                }
            }
        }
        self.bombs = bombs;
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
        //println!("{:?}", args);
    }

    pub fn handle_press(&mut self, args: Button) {
        //println!("{:?}", args);
    }

    pub fn set_last_cursor_position(&mut self, pos: [f64; 2]) {
        self.cursor_pos = pos;
    }
}
