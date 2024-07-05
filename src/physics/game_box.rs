


pub struct PhysicsBox<O> {
    pub x_velocity: f64,
    pub y_velocity: f64,
    // coords
    pub x: f64,
    pub y: f64,
    // if the box has collision
    pub colision: bool,
    // color of the box in RGB-O
    pub render_obj: O,
    pub corners: [f64; 4]
}

impl<O> PhysicsBox<O> {
    pub fn new(colision: bool, render_obj: O, corners: [f64; 4]) -> PhysicsBox<O> {
        PhysicsBox {
            x_velocity: 0.0,
            y_velocity: 0.0,
            x: corners[0],
            y: corners[1],
            colision,
            render_obj,
            corners,
        }
    }


    pub fn update() {
        
    }


}

