extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

#[path = "physics/game_objects.rs"] mod game_objects;
#[path = "physics/game_box.rs"] mod game_box;
mod input_box;
mod game;
mod tile;

use game::Game;


// The main function that every Rust application requires!
fn main() {
    // Instanciating the Game struct
    let mut game = Game::new();
    game.game_loop();   
}
