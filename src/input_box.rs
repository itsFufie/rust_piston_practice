// use graphics::{color, rectangle::{self, Shape}, Rectangle};

pub struct InputBox {
    pub coords: [f64;4],
    pub selected: bool,
    pub value: String,
    pub color: [f32; 4]
}
