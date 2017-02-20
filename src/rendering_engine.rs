use scene::Scene;
use raycast::{raycast, RaycastCollision, Direction};
use na;
use util_math;
use util_math::clamp;
use texture::Texture;

pub fn clear(target_surface : &mut[u8], pitch : usize, height : usize) {
       let max = pitch * height;
       for i in 0..max {
           target_surface[i] = 0u8;
       }
}

pub fn draw(scene : &Scene, target_surface : &mut[u8], pitch : usize, width : usize, height : usize) {
    let (v1, v2) = get_raycast_interval_vectors(1.0, scene.camera().fov, scene.camera().orientation);

    for i in 0..(width as i32) {
        // println!("col {}", i);
        
        let ray_point = util_math::lerp(v1, v2, (i as f64) / (width as f64 - 1.0));
        draw_scanline(scene, target_surface, i, height as i32, ray_point, pitch);
    }
}

/** Returns vectors (v1, v2) that can be linearly interpolated to send rays through the scene for each scanline
    * @param d Distance to the viewing plane
    * @param f field of view angle in radians
    * @param theta Camera rotation angle in radians */
fn get_raycast_interval_vectors(d : f64, f : f64, theta : f64) -> (na::Vector2<f64>, na::Vector2<f64>) {
    let fo2 = f / 2.0;
    let theta1 = theta + fo2;
    let theta2 = theta - fo2;
    let magnitude = d / fo2.cos();
    (na::Vector2{x: theta1.cos(), y: theta1.sin()} * magnitude, na::Vector2{x: theta2.cos(), y: theta2.sin()} * magnitude)
}

fn draw_scanline(scene : &Scene, target_surface : &mut[u8], target_x : i32, target_height : i32, ray_point : na::Vector2<f64>, pitch : usize) {
    let collision = raycast(scene.camera().position, ray_point, &scene.worldmap);

    match collision {
        Some(RaycastCollision{distance, position, colour_index, tangent}) => {
            // Generate scanline based on collision information
            let wall_height = deduce_scanline_wall_height(distance, scene.camera().fov);
            let wall_height = clamp(wall_height, 0, target_height);
            
            // println!("wall_height {}", wall_height);
            
            let start_of_wall_y = (target_height - wall_height) / 2;
            let start_of_wall_y = clamp(start_of_wall_y, 0, target_height);

            for i in 0..wall_height {
                let y = start_of_wall_y + i;
                // let colour = extract_texture_colour(wall_texture, tangent, wall_height, i, position.x as i32, position.y as i32);
                let colour = 200u8;
                // println!("Setting ({}, {})", target_x, y);
                set_buffer_colour(target_surface, colour, target_x, y, pitch);
            }
        },
        None => {
            // Generate default scanline
            for y in 0..target_height {
                set_buffer_colour(target_surface, 0u8, target_x, y, pitch);
            }
        }
    }
}

fn set_buffer_colour(buffer : &mut[u8], colour : u8, x : i32, y : i32, pitch : usize) {
    let index = (x * 4 + y * (pitch as i32)) as usize;
    buffer[index] = colour;
    buffer[index+1] = colour;
    buffer[index+2] = colour;
    buffer[index+3] = colour;
}

// fn extract_texture_colour(texture : &Texture, tangent : Direction, wall_height : i32, y_from_wall_start : i32, world_x : i32, world_y : i32) {
// let x_index = match tangent {
//     HORIZONTAL => world_x, 
//     VERTICAL   => world_y,
// };
// let x_index =  x_index % texture.width;

// let y_index = (y_from_wall_start as f64) / wall_height * texture.height;
// let y_index = clamp(y_index as i32, 0, texture.height);

// texture[y_index][x_index]
// }

// fn fetch_texture_from_index(index : u8, textures : &Vec<Texture>) {
// textures[index]
// }

/** TODO use vertical fov */
fn deduce_scanline_wall_height(distance : f64, fov : f64) -> i32 {
    let wall_height_in_pixels = 5.0;
    let height_to_fill_screen = distance * (fov / 2.0).tan() * 2.0;
    let screen_height = 200.0;

    ((wall_height_in_pixels * screen_height) / height_to_fill_screen) as i32
}