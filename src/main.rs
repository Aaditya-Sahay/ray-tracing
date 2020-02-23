use vec3::Vec3;
pub mod vec3;
use ray::Ray;
pub mod ray;
use hittable::*;
pub mod hittable;
use sphere::Sphere;
pub mod sphere;

fn main(){
    generate_image();
}


pub fn generate_image() {
    let nx:u32 = 200;
    let ny:u32 = 100;
    println!("P3\n{} {}\n255", nx,ny);

    let lower_left_corner = Vec3::new(-2.0,-1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    let mut hittable_list = HittableList::new();
    hittable_list.add_sphere(Sphere::new(Vec3::new(0.0,0.0,-1.0), 0.5));
    hittable_list.add_sphere(Sphere::new(Vec3::new(0.0,-100.5,-1.0), 100.0));

    let mut j = (ny - 1) as i32;
    while j  >= 0  {
        for i in 0..nx {
            let u = i as f64 / nx as f64;
            let v = j as f64 / ny as f64;
            let ray:Ray = Ray::new(origin, lower_left_corner + u*horizontal + v*vertical);
            let p:Vec3 = ray.point_to_parameter(2.0);
            let col = color(ray, &hittable_list);
            let ir = (255.99 * col.r()) as u32;
            let ig = (255.99 * col.g()) as u32;
            let ib = (255.99 * col.b()) as u32;
            println!("{} {} {}",ir,ig,ib);
        }
        j-=1;
    }
}


// vec3 color(const ray& r, hittable *world) {
//     hit_record rec;
//     if (world->hit(r, 0.0, MAXFLOAT, rec)) {
//         return 0.5*vec3(rec.normal.x()+1, rec.normal.y()+1, rec.normal.z()+1);
//     }
//     else {
//         vec3 unit_direction = unit_vector(r.direction());
//         float t = 0.5*(unit_direction.y() + 1.0);
//         return (1.0-t)*vec3(1.0, 1.0, 1.0) + t*vec3(0.5, 0.7, 1.0);
//     }
// }
pub fn color(ray: Ray, world:&HittableList) -> Vec3 {
    let mut hit_record = HitRecord::new();
    if world.intersect(&ray, 0.0, std::f64::MAX, &mut hit_record){
        return 0.5*Vec3::new(hit_record.normal.x()+ 1.0, hit_record.normal.y()+1.0, hit_record.normal.z()+1.0);
    }else {
        let unit_direction:Vec3 = ray.direction().unit_vector();
        let t = 0.5* unit_direction.y() + 1.0;
        return (1.0-t) * Vec3::new(1.0, 1.0, 1.0) + t*Vec3::new(0.5, 0.7, 1.0);
    }
  
}

pub fn hit_sphere(center: Vec3, radius: f64, ray: Ray) -> f64 {
    let oc = ray.origin() - center;
    let a = ray.direction().dot(&ray.direction());
    let b = 2.0 * oc.dot(&ray.direction());
    let c = oc.dot(&oc) - radius * radius;
    let discriminant = b*b - 4.0*a*c;
    if discriminant < 0.0 {
        return -1.0;
    }
    else {
        return (-b - discriminant.sqrt() ) / (2.0*a);
    }
  
}