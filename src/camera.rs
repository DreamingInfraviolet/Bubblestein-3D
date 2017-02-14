use na;

pub struct Camera {
    pub position : na::Vector2<f64>,
    pub orientation : f64,
    pub fov : f64,
}