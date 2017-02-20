use na;
use texture::Texture;

#[derive(PartialEq)]
pub enum Direction { HORIZONTAL, VERTICAL }

pub struct RaycastCollision {
    pub distance : f64,
    pub position : na::Vector2<f64>,
    pub colour_index : u8,
    pub tangent : Direction,
}

pub fn raycast(origin : na::Vector2<f64>, direction : na::Vector2<f64>, worldmap : &Texture) -> Option<RaycastCollision> {

    //length of ray from one x or y-side to next x or y-side
    let delta_dist_x = (1.0 + (direction.y * direction.y) / (direction.x * direction.x)).sqrt();
    let delta_dist_y = (1.0 + (direction.x * direction.x) / (direction.y * direction.y)).sqrt();

    //which box of the map we're in
    let mut map_x = origin.x as i32;
    let mut map_y = origin.y as i32;

    //calculate step and initial sideDist
    //sideDist = length of ray from current position to next x or y-side
    //step     = what direction to step in x or y-direction (either +1 or -1)
    let (stepX, mut side_dist_x) = if direction.x < 0.0 {
        (-1, (origin.x - (map_x as f64)) * delta_dist_x)
    } else {
        (1, ((map_x as f64) + 1.0 - origin.x) * delta_dist_x)
    };

    let (stepY, mut side_dist_y) = if direction.y < 0.0 {
        (-1, (origin.y - (map_y as f64)) * delta_dist_y)
    } else {
        (1, ((map_y as f64) + 1.0 - origin.y) * delta_dist_y)
    };

    let mut edirection = Direction::VERTICAL;
    
    loop {
        //jump to next map square, OR in x-direction, OR in y-direction
        if side_dist_x < side_dist_y {
          side_dist_x += delta_dist_x;
          map_x += stepX;
          edirection = Direction::VERTICAL;
        } else {
          side_dist_y += delta_dist_y;
          map_y += stepY;
          edirection = Direction::HORIZONTAL;
        }

        // Out of bounds. Found nothing.
        if map_x < 0 || map_y < 0 || map_x >= worldmap.width() || map_y >= worldmap.height() {
            return None;
        }

        //Check if ray has hit a wall
        if worldmap.at(map_x as u32, map_y as u32) > 0 {
            break;
        }
    }

    let distance = if edirection == Direction::VERTICAL {
        (map_x as f64 - origin.x + (1.0 - stepX as f64) / 2.0) / direction.x
    } else {
        (map_y as f64 - origin.y + (1.0 - stepY as f64) / 2.0) / direction.y
    };

    Some(RaycastCollision{distance: distance, position: na::Vector2{x: 0.0, y: 0.0}, colour_index: 0, tangent: edirection })

}
