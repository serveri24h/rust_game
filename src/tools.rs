use bevy::prelude::*;

use crate::HEX_RADIUS;

pub fn compute_distance(    
    pos_a: Vec3,
    pos_b: Vec3,
) -> f32 {
        let dif = pos_a - pos_b;
        (dif[0]*dif[0]+dif[1]*dif[1]).sqrt()
}

pub trait Collider {

    fn get_translation(&self) -> Vec3;

    fn get_radius(&self) -> f32 {
        HEX_RADIUS
    } 
    

    fn collision_check(
        &self,
        pos: Vec3,
    ) -> bool {
        if compute_distance(self.get_translation(), pos) < self.get_radius() {
            return true;
        }; 
        return  false;
    }
}
