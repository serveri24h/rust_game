use bevy::prelude::*;

pub fn compute_distance(    
    pos_a: Vec3,
    pos_b: Vec3,
) -> f32 {
        let x_dif = pos_a[0]-pos_b[0];
        let y_dif = pos_a[1]-pos_b[1];
        (x_dif*x_dif+y_dif*y_dif).sqrt()
}