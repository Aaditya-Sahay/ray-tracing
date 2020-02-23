extern crate image;

pub fn generate_image() {
    let nx:u32 = 200;
    let ny:u32 = 100;
    println!("P3\n{} {}\n255", nx,ny);
    let mut j = (ny - 1) as i32;

    while j  >= 0  {
        for i in 0..nx {
            let r = i as f32 / nx as f32;
            let g = j as f32 / ny as f32;
            let b:f32 = 0.2;

            let ir = (255.99 * r) as u32;
            let ig = (255.99 * g) as u32;
            let ib = (255.99 * b) as u32;
            println!("{} {} {}",ir,ig,ib);
        }
        j-=1;
    }
}


// relevant C++ code  from the book 

/*
int main() {
    int nx = 200;
    int ny = 100;
    std::cout << "P3\n" << nx << " " << ny << "\n255\n";
    for (int j = ny-1; j >= 0; j--) {
        for (int i = 0; i < nx; i++) {
            float r = float(i) / float(nx);
            float g = float(j) / float(ny);
            float b = 0.2;
            int ir = int(255.99*r);
            int ig = int(255.99*g);
            int ib = int(255.99*b);
            std::cout << ir << " " << ig << " " << ib << "\n";
        }
    }
}
*/