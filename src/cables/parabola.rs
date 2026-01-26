use core::f32;

use bevy::{math::FloatPow, prelude::*};

pub(super) fn get_parabola(t: f32, start_pos: Vec3, end_pos: Vec3) -> Option<Vec3> {
    let dxz = end_pos.xz() - start_pos.xz();
    
    let dy = end_pos.y - start_pos.y;
    let dx = dxz.length();
    let k = dy.max(0.0);

    match dy == 0.0 {
        true => {
            Some(start_pos.move_towards(end_pos, t))
        }
        false => {
            // compute 2d parabola
            let a = ((k.sqrt() + (k-dy).sqrt()) / dx).squared();
            
            let h = ((k - dy) / a).sqrt();
            let raw_parabola = |x: f32| a * (x - h).squared() + (dy - k);
            let local_y = raw_parabola(t * dx);
            match local_y.is_nan() {
                true => return None,
                _ => {}
            }
            // map back to 3d
            let out = t * dx * Vec3::new(dxz.x, 0.0, dxz.y).normalize() + local_y * Vec3::Y;
            Some(start_pos + out)
        }
    }
    
    
}