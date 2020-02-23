
mod vec3;

fn main(){
    generate_image();
}


pub fn generate_image() {
    let nx:u32 = 200;
    let ny:u32 = 100;
    println!("P3\n{} {}\n255", nx,ny);
    let mut j = (ny - 1) as i32;

    while j  >= 0  {
        for i in 0..nx {
            let col = vec3::Vec3::new(i as f64 / nx as f64, j as f64 / ny as f64, 0.2);
            let ir = (255.99 * col.r()) as u32;
            let ig = (255.99 * col.g()) as u32;
            let ib = (255.99 * col.b()) as u32;
            println!("{} {} {}",ir,ig,ib);
        }
        j-=1;
    }
}
