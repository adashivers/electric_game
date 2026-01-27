use core::f32;

use bevy::{math::FloatPow, prelude::*};

pub(super) fn get_parabola(t: f32, start_pos: Vec3, end_pos: Vec3, hang: f32) -> Option<Vec3> {

    // hang is negative
    if hang < 0.0 { return None }

    let dxz = end_pos.xz() - start_pos.xz();
    let dy = end_pos.y - start_pos.y;
    let dx = dxz.length();
    let k = (dy+hang).max(hang);

    if k == 0.0 {
        return Some(start_pos.move_towards(end_pos, t))
    }

    // compute 2d parabola  
    match dx == 0.0 {
        // if vertical, take a parabola close by and use only y component
        true => {
            let local_y = parabola_2d_helper(k, 1.0, dy, t);
            match local_y.is_nan() {
                true => return None,
                _ => {}
            }
            let out = local_y * Vec3::Y;
            Some(start_pos + out)
        },
        // if not, apply basis
        false => {
            let local_y = parabola_2d_helper(k, dx, dy, t);
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

fn parabola_2d_helper(k: f32, dx: f32, dy: f32, t: f32) -> f32 {
    let a = ((k.sqrt() + (k-dy).sqrt()) / dx).squared();
    let h = ((k - dy) / a).sqrt();

    let raw_parabola = |x: f32| a * (x - h).squared() + (dy - k);
    raw_parabola(t * dx)
}

#[cfg(test)]
mod tests {
    use bevy::prelude::*;
    use super::*;

    // -- basic --
    // endpoints are at the right places
    #[test]
    fn test_endpoints_correct() {
        let (exp_start, exp_end) = (Vec3::new(-2.0, 4.0, 5.2), Vec3::new(0.0, -4.7, -1.23));

        let start = get_parabola(0.0, exp_start, exp_end, 3.0).unwrap();
        let end = get_parabola(1.0, exp_start, exp_end, 3.0).unwrap();
        assert!((start - exp_start).length() < 0.001);
        assert!((end-exp_end).length() < 0.001);
    }

    // -- edge cases --
    // giving equal start and end positions
    #[test]
    fn test_equal() {
        let (exp_start, exp_end) = (Vec3::new(5.0, 4.2, 2.04), Vec3::new(5.0, 4.2, 2.04));
        let start = get_parabola(0.0, exp_start, exp_end, 3.0).unwrap();
        let end = get_parabola(1.0, exp_start, exp_end, 3.0).unwrap();
        assert!((start - exp_start).length() < 0.001);
        assert!((end - exp_end).length() < 0.001);

        let mid = get_parabola(0.5, exp_start, exp_end, 3.0);
        assert!(mid.is_some());

    }

    // giving start and end positions with the same x and z components
    #[test]
    fn test_vertical() {
        let (exp_start, exp_end) = (Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
        let start = get_parabola(0.0, exp_start, exp_end, 3.0).unwrap();
        let end = get_parabola(1.0, exp_start, exp_end, 3.0).unwrap();
        assert!(get_parabola(0.5, exp_start, exp_end, 3.0).is_some());
        assert!((start - exp_start).length() < 0.001);
        assert!((end - exp_end).length() < 0.001);

        let mid = get_parabola(0.5, exp_start, exp_end, 3.0);
        assert!(mid.is_some());
    }
}