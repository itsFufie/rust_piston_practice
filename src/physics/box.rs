

pub struct PhysicsBox {
    x_velocity: f64,
    y_velocity: f64,
    // coords
    x: f64,
    y: f64,
    // if the box has collision
    colision: bool,
    // defining corners of the box, used to detect colision
    colision_coords: [f64;4],
    // color of the box in RGB-O
    color: [f64; 4]
}

