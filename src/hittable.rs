use crate::vec3::Vec3;
use crate::ray::Ray; 
use crate::sphere::Sphere; 



pub trait Hittable {
    fn intersect(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

pub struct HitRecord {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord{
            t: 0.0, 
            p: Vec3::new(0.0, 0.0, 0.0), 
            normal: Vec3::new(0.0, 0.0, 0.0)
        }
    }
    pub fn normal(&self) -> Vec3 {
        self.normal
    }

    pub fn p(&self) -> Vec3 {
        self.p
    }

}

pub struct HittableList {
    spheres: Vec<Sphere>
}

impl HittableList {
    pub fn new() -> Self {
        let spheres_list: Vec<Sphere> = Vec::new();
        HittableList {
            spheres: spheres_list
        }
    }
    pub fn add_sphere(&mut self, sphere: Sphere) {
        self.spheres.push(sphere);
    }

    pub fn size(&self) -> usize {
        self.spheres.len()
    }
}

impl Hittable for HittableList {
    fn intersect(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec: HitRecord = HitRecord::new();
        let mut hit_anything: bool = false;
        let mut closest_so_far: f64 = t_max;
        for i in 0..self.size() {
            if self.spheres[i].intersect(ray, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                rec.p = temp_rec.p;
                rec.t = temp_rec.t;
                rec.normal = temp_rec.normal;
            }
        }
        hit_anything
    }
}