// use raytracer::*;
mod vector;
use vector::vec3;

fn dot(v1: vec3, v2: vec3) -> f64{
    v1.x*v2.x+v1.y*v2.y+v1.z+v2.y
}

fn cross(v1: vec3, v2: vec3) -> vec3{
    vec3::new(v1.y * v2.z - v1.z * v2.y,
        v1.z * v2.x - v1.x * v2.z,
        v1.x * v2.y - v1.y * v2.x)
}

fn unit_vector(v1: vec3) -> vec3{
    v1/v1.length()
}

////////////////////////// COLOUR UTILITY FUNCTIONS /////////////////////////

fn write_colour(colour: vec3){
    let ir = (255.999*colour.x) as i32;
    let ig = (255.999*colour.y) as i32;
    let ib = (255.999*colour.z) as i32;
    print!("{} {} {}\n", ir, ig, ib);
}


use vec3 as colour;
// use vec3 as point3;

fn main(){
    let image_width = 256;
    let image_height = 256;

    let mut v1 = vec3::new(1.0, 2.0, 3.0);
    eprintln!("The vector is {:?}.", v1);
    eprintln!("X is: {}", v1.x);

    print!("P3\n{} {}\n255\n", image_width, image_height);
    for j in (0 .. image_height).rev(){
        // Debug msg
        eprint!("\rScanlines remaining: {}     ", j);
        for i in 0..image_width{
            // colour is an alias for vec3
            let c: colour = colour::new(
                (i as f64)/(image_width-1) as f64,
                (j as f64)/(image_height-1) as f64,
                0.25);
            write_colour(c);
        }
    }
    eprintln!("\nDone!");
}