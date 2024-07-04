mod r#box;

use self::r#box::PhysicsBox;

pub enum GameObject {
    Box(PhysicsBox)
}