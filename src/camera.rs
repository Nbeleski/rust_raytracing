use rand::Rng;
use crate::vec::{Vec3, Ray};

use std::f32::consts::PI;

#[derive(Clone, Copy, Debug)]
pub struct Camera {
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub origin: Vec3
}

impl Camera {
    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        return Ray::new(self.origin, self.lower_left_corner + u*self.horizontal + v*self.vertical - self.origin)
    }
}

impl Default for Camera {
    fn default() -> Camera {
        Camera{
            lower_left_corner: Vec3(-2.0, -1.0, -1.0),
            horizontal: Vec3(4.0, 0.0, 0.0),
            vertical: Vec3(0.0, 2.0, 0.0),
            origin: Vec3(0.0, 0.0, 0.0),
        }
    }
}