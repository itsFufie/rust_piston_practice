extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

mod input_box;
mod game;

use game::Game;


// The main function that every Rust application requires!
fn main() {
    // Instanciating the Game struct
    let mut game = Game::new();
    game.game_loop();   
}
