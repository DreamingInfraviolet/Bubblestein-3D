use na;
use texture::Texture;

pub enum Direction { HORIZONTAL, VERTICAL }

pub struct RaycastCollision {
    pub distance : f64,
    pub position : na::Vector2<f64>,
    pub colour_index : u8,
    pub tangent : Direction,
}

pub fn raycast(origin : na::Vector2<f64>, direction : na::Vector2<f64>, worldmap : &Texture) -> Option<RaycastCollision> {
    // @TODO: Placeholder
    Some(RaycastCollision{distance: 1.0, position: na::Vector2{x: 0.0, y: 0.0}, colour_index: 0, tangent: Direction::HORIZONTAL })
}
