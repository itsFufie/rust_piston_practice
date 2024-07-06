mod game_box;

use game_box::PhysicsBox;
use graphics::{Context, Ellipse, Rectangle, Transformed};
use opengl_graphics::GlGraphics;

pub trait Drawable {
    fn draw(&mut self, c: Context, gl: &mut GlGraphics);
}

pub trait Physics {
    fn update(&mut self);
}

pub enum GameObject {
    Box(PhysicsBox<Rectangle>),
    Circle(PhysicsBox<Ellipse>)
}

impl Drawable for PhysicsBox<Rectangle> {
    fn draw(&mut self, c: Context, gl: &mut GlGraphics) {
        self.render_obj.draw(self.corners, &c.draw_state, c.transform.trans(self.x, self.y), gl);
    }
}

impl Drawable for PhysicsBox<Ellipse> {
    fn draw(&mut self, c: Context, gl: &mut GlGraphics) {
        self.render_obj.draw(self.corners, &c.draw_state, c.transform.trans(self.x, self.y), gl);
    }
}




