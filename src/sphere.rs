use crate::vec3::Vec3;
use crate::ray::Ray; 
use crate::hittable::*;

pub struct Sphere {
    center: Vec3,
    radius: f64
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Self{
        Sphere {
            center,
            radius
        }
    } 
    pub fn center(&self) -> Vec3 {
        self.center
    }
    pub fn radius(&self) -> f64{
        self.radius
    }
}

impl Hittable for Sphere {
    fn intersect(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc: Vec3 = ray.origin() - self.center();
        let a: f64 = ray.direction().dot(&ray.direction());
        let b: f64 = 2.0 * oc.dot(&ray.direction());
        let c: f64 = oc.dot(&oc) - self.radius * self.radius;
        let discriminant: f64 = b * b - 4.0 * a * c;
        if discriminant > 0.0 {
            let t: f64 = (0.0 - b - discriminant.sqrt()) / (2.0 * a);
            if t < t_max && t > t_min {
                rec.t = t;
                rec.p = ray.point_to_parameter(t);
                // The length p - c would be the radius
                rec.normal = (rec.p - self.center())/self.radius();
      
                return true;
            }
            let t: f64 = (0.0 - b + discriminant.sqrt()) / (2.0 * a);
            if t < t_max && t > t_min {
                rec.t = t;
                rec.p = ray.point_to_parameter(t);
                // The length p - c would be the radius
                rec.normal = (rec.p - self.center())/self.radius();
                return true;
            }
        }
        false
    }
}