use vec3::Vec3;
pub mod vec3;
use ray::Ray;
pub mod ray;
use hittable::*;
pub mod hittable;
use sphere::Sphere;
pub mod sphere;
use camera::Camera;
use material::*;
pub mod camera;
pub mod material;

extern crate rand;
use rand::Rng;

fn main() {
    generate_image();
}

pub fn generate_image() {
    let mut rng = rand::thread_rng();
    let nx: u32 = 120;
    let ny: u32 = 60;
    let ns: u32 = 100;

    println!("P3\n{} {}\n255", nx, ny);

    let hittable_list = random_spheres();

    //setting up camera
    let lookfrom = Vec3::new(5.0,4.0,3.0);
    let lookat = Vec3::new(0.0, 0.0, 1.0);
    let dist_to_focus = (lookfrom - lookat).length();
    let aperture = 1.0;

    let camera = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0.0, 1.0, 0.0),
        40.0,
        (nx as f64) / ny as f64,
        aperture,
        dist_to_focus,
    );

    let mut j = (ny - 1) as i32;
    while j >= 0 {
        for i in 0..nx {
            let mut col = Vec3::new(0.0, 0.0, 0.0);
            for _s in 0..ns {
                let u = (i as f64 + rng.gen::<f64>()) / nx as f64;
                let v = (j as f64 + rng.gen::<f64>()) / ny as f64;
                let ray: Ray = camera.get_ray(u, v);
                col = col + color(ray, &hittable_list, 0);
            }
            col = col / ns as f64;
            col = Vec3::new(col.r().sqrt(), col.g().sqrt(), col.b().sqrt());
            let ir = (255.99 * col.r()) as u32;
            let ig = (255.99 * col.g()) as u32;
            let ib = (255.99 * col.b()) as u32;
            println!("{} {} {}", ir, ig, ib);
        }
        j -= 1;
    }
}

pub fn color(ray: Ray, world: &HittableList, depth: u32) -> Vec3 {
    // if (world->hit(r, 0.001, MAXFLOAT, rec)) {
    //     ray scattered;
    //     vec3 attenuation;
    //     if (depth < 50 && rec.mat_ptr->scatter(r, rec, attenuation, scattered)) {
    //         return attenuation*color(scattered, world, depth+1);
    //     }
    //     else {
    //         return vec3(0,0,0);
    //     } from peter sherley ray tracing in one weekend

    let mut hit_record = HitRecord::new();
    if world.intersect(&ray, 0.001, std::f64::MAX, &mut hit_record) {
        let mut scattered: Ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0));
        let mut attenuation: Vec3 = Vec3::new(0.0, 0.0, 0.0);
        if depth < 50
            && hit_record
                .material
                .scatter(&ray, &hit_record, &mut attenuation, &mut scattered)
        {
            return attenuation * color(scattered, world, depth + 1);
        } else {
            return Vec3::new(0.0, 0.0, 0.0);
        }
    } else {
        let unit_direction: Vec3 = ray.direction().unit_vector();
        let t = 0.5 * unit_direction.y() + 1.0;
        return (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0);
    }
}

pub fn hit_sphere(center: Vec3, radius: f64, ray: Ray) -> f64 {
    let oc = ray.origin() - center;
    let a = ray.direction().dot(&ray.direction());
    let b = 2.0 * oc.dot(&ray.direction());
    let c = oc.dot(&oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-b - discriminant.sqrt()) / (2.0 * a);
    }
}

//int n = 500;
// hittable **list = new hittable*[n+1];
// list[0] =  new sphere(vec3(0,-1000,0), 1000, new lambertian(vec3(0.5, 0.5, 0.5)));
// int i = 1;
// for (int a = -11; a < 11; a++) {
//     for (int b = -11; b < 11; b++) {
//         float random = random_double();
//         vec3 center(a+0.9*random_double(),0.2,b+0.9*random_double());
//         if ((center-vec3(4,0.2,0)).length() > 0.9) {
//             if (choose_mat < 0.8) {  // diffuse
//                 list[i++] = new sphere(center, 0.2,
//                     new lambertian(vec3(random_double()*random_double(),
//                                         random_double()*random_double(),
//                                         random_double()*random_double())
//                     )
//                 );
//             }
//             else if (choose_mat < 0.95) { // metal
//                 list[i++] = new sphere(center, 0.2,
//                         new metal(vec3(0.5*(1 + random_double()),
//                                        0.5*(1 + random_double()),
//                                        0.5*(1 + random_double())),
//                                   0.5*random_double()));
//             }
//             else {  // glass
//                 list[i++] = new sphere(center, 0.2, new dielectric(1.5));
//             }
//         }
//     }
// }

pub fn random_spheres() -> HittableList {
    let mut rng = rand::thread_rng();
    let mut hittable_list = HittableList::new();
    hittable_list.add_sphere(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0, 
        Material::Lambertian(Lambertian::new(Vec3::new(0.5, 0.5, 0.5))),
    ));
    for a in -11..11 {
        for b in -11..11 {
            let random:f64 = rng.gen();
            let center = Vec3::new(a as f64 + 0.9 * rng.gen::<f64>(), 0.2, b as f64 + 0.9 * rng.gen::<f64>());
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if random < 0.8 {
                    // diffuse
                    hittable_list.add_sphere(Sphere::new(
                        center,
                        0.2,
                        Material::Lambertian(Lambertian::new(Vec3::new(
                            rng.gen::<f64>() * rng.gen::<f64>(),
                            rng.gen::<f64>() * rng.gen::<f64>(),
                            rng.gen::<f64>() * rng.gen::<f64>(),
                        ))),
                    ));
                } else if random < 0.95 {
                    // metal
                    hittable_list.add_sphere(Sphere::new(
                        center,
                        0.2,
                        Material::Metal(Metal::new(
                            Vec3::new(
                                rng.gen::<f64>() * rng.gen::<f64>(),
                                rng.gen::<f64>() * rng.gen::<f64>(),
                                rng.gen::<f64>() * rng.gen::<f64>(),
                            ),
                            0.5 * rng.gen::<f64>(),
                        )),
                    ));
                } else {
                    // glass
                    hittable_list.add_sphere(Sphere::new(
                        center,
                        0.2,
                        Material::Dielectric(Dielectric::new(1.5)),
                    ));
                }
            }
        }
    }

    hittable_list.add_sphere(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Material::Dielectric(Dielectric::new(1.5)),
    ));
    hittable_list.add_sphere(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Material::Lambertian(Lambertian::new(Vec3::new(
            rng.gen::<f64>() * rng.gen::<f64>(),
            rng.gen::<f64>() * rng.gen::<f64>(),
            rng.gen::<f64>() * rng.gen::<f64>(),
        ))),
    ));
    hittable_list.add_sphere(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Material::Metal(Metal::new(
            Vec3::new(
                rng.gen::<f64>() * rng.gen::<f64>(),
                rng.gen::<f64>() * rng.gen::<f64>(),
                rng.gen::<f64>() * rng.gen::<f64>(),
            ),
            0.5 * rng.gen::<f64>(),
        )),
    ));
    hittable_list
}
