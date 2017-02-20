use camera::Camera;
use na::Vector2;
use na;

const player_movement_speed : f64 =  1.0;

pub struct Player {
    pub camera : Camera,
    movement_vector : Vector2<f64>,
    health : f64,
}

impl Player {
    pub fn new(camera : Camera) -> Player {
        let movement_vector = Vector2{ x: 0.0, y: 0.0 };
        Player { camera: camera, movement_vector: movement_vector, health: 1.0, }
    }

    pub fn add_movement_vector(&mut self, vector : Vector2<f64>) {
        self.movement_vector += vector;
    }

    pub fn walk(&mut self) {
        self.camera.position += na::normalize(&self.movement_vector) * player_movement_speed;
        self.movement_vector = Vector2{ x: 0.0, y: 0.0 };
    }
}