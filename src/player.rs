use camera::Camera;
use na::Vector2;
use na;
use na::AbsoluteRotate;

const player_movement_speed : f64 =  1.0;
const player_turn_speed : f64 = 0.01;

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
        let zero = na::Vector2{x: 0.0, y:0.0};
        if !na::approx_eq(&self.movement_vector, &zero) {
            let velocity_vector = na::normalize(&self.movement_vector) * player_movement_speed;
            let rotation = na::Rotation2::new(na::Vector1{x: self.camera.orientation});
            let rotation = na::to_rotation_matrix(&rotation);
            self.camera.position += na::rotate(&rotation, &velocity_vector);
            self.movement_vector = Vector2{ x: 0.0, y: 0.0 };
        }
    }

    pub fn turn_right(&mut self, speed : f64) {
        self.camera.orientation += player_turn_speed * speed;
    }
}