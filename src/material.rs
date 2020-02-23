use crate::vec3::Vec3;
use crate::ray::Ray; 
use crate::hittable::*;
extern crate rand;
use rand::Rng;


#[derive(Clone, Copy, Debug)]
pub enum Material {
	Lambertian(Lambertian),
	Metal(Metal),
	Dielectric(Dielectric),
}

impl Scatterable for Material {
	fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
		match *self {
			Material::Lambertian(ref inner) => inner.scatter(ray_in, hit_record, attenuation, scattered),
			Material::Metal(ref inner) => inner.scatter(ray_in, hit_record, attenuation, scattered),
			Material::Dielectric(ref inner) => inner.scatter(ray_in, hit_record, attenuation, scattered),
		}
	}
}

pub trait Scatterable {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool;
}


#[derive(Clone, Copy, Debug)]
pub struct Metal {
    albedo: Vec3,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f64) -> Metal {
        let f: f64 = if fuzz < 1.0 {fuzz} else {1.0};
        Metal { albedo, fuzz: f }
    }
}

impl Scatterable for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let reflected: Vec3 = reflect(&ray_in.direction().unit_vector(), &hit_record.normal());
        *scattered = Ray::new(hit_record.p(), reflected + self.fuzz*point_in_unit_sphere());
        *attenuation = self.albedo;
        scattered.direction().dot(&hit_record.normal()) > 0.0
    }
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - 2.0 * v.dot(n) * *n
}



#[derive(Clone, Copy, Debug)]
pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Scatterable for Lambertian {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let target: Vec3 = hit_record.p() + hit_record.normal() + point_in_unit_sphere();
        *scattered = Ray::new(hit_record.p(), target - hit_record.p());
        *attenuation = self.albedo;
        true
    }
}

pub fn point_in_unit_sphere() -> Vec3 {
    // Initialising p with a value outside the unit sphere
    let mut p: Vec3 = Vec3::new(2.0, 2.0, 2.0);
    let mut rng = rand::thread_rng();
    while p.dot(&p) >= 1.0 {
        p = 2.0 * Vec3::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>()) - Vec3::new(1.0, 1.0, 1.0);
    }
    p
}




#[derive(Clone, Copy, Debug)]
pub struct Dielectric {
    ri: f64,
}

impl Dielectric {
    pub fn new(ri: f64) -> Dielectric {
        Dielectric { ri }
    }
}

fn refract(v: &Vec3, n: &Vec3, ni_over_nt: f64, refracted: &mut Vec3) -> bool {
    let uv: Vec3 = v.unit_vector();
    let dt: f64 = uv.dot(&n);

    let discriminant: f64 = 1.0 - ni_over_nt*ni_over_nt*(1.0 - dt*dt);
    if discriminant > 0.0 {
        *refracted = ni_over_nt*(uv - *n*dt) - *n*((discriminant).sqrt());
        return true;
    } else {
        return false;
    }
}

fn schlick(cosine: f64, ri: f64) -> f64 {
    let mut r0: f64 = (1.0 - ri)/(1.0 + ri);
    r0 = r0*r0;
    return r0 + (1.0-r0)*(1.0-cosine).powi(5);
}

impl Scatterable for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let outward_normal: Vec3;
        let reflected = reflect(&ray_in.direction().unit_vector(), &hit_record.normal());
        let ni_over_nt: f64;
        *attenuation = Vec3::new(1.0, 1.0, 1.0);
        let mut refracted: Vec3 = Vec3::new(0.0, 0.0, 0.0);
        let reflection_probability: f64;
        let cosine: f64;
        if ray_in.direction().dot(&hit_record.normal()) > 0.0 {
            outward_normal = -1.0*hit_record.normal();
            ni_over_nt = self.ri;
            cosine = self.ri*ray_in.direction().dot(&hit_record.normal())/ray_in.direction().length();
        } else {
            outward_normal = 1.0*hit_record.normal();
            ni_over_nt = 1.0/self.ri;
            cosine = -1.0*ray_in.direction().dot(&hit_record.normal())/ray_in.direction().length();
        }
        if refract(&ray_in.direction(), &outward_normal, ni_over_nt, &mut refracted) {
            reflection_probability = schlick(cosine, self.ri);
        } else {
            *scattered = Ray::new(hit_record.p(), reflected);
            reflection_probability = 1.0;
        }
        let mut rng = rand::thread_rng();
        if rng.gen::<f64>() < reflection_probability {
            *scattered = Ray::new(hit_record.p(), reflected);
        } else {
            *scattered = Ray::new(hit_record.p(), refracted);
        }
        return true;
    }
}