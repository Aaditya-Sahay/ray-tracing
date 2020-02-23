use crate::vec3::Vec3;
use crate::ray::Ray; 

extern crate rand;
use rand::Rng;

pub struct Camera {
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            lower_left_corner: Vec3::new(-2.0, -1.0, -1.0),
            horizontal: Vec3::new(4.0, 0.0, 0.0),
            vertical: Vec3::new(0.0, 2.0, 0.0),
            origin: Vec3::new(0.0, 0.0, 0.0),
        }
    
    }
    pub fn get_ray(&self, u:f64, v:f64) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + u* self.horizontal + v* self.vertical)
    }
}

fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    let mut p: Vec3;
    while {
        p = 2.0*Vec3::new(rng.gen::<f64>(), rng.gen::<f64>(), 0.0) - Vec3::new(1.0, 1.0, 0.0);
        p.dot(&p) >= 1.0
    } {}
    p
}