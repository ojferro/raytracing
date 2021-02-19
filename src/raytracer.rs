use vector::vec3;
mod vector;

use ray::Ray;
mod ray;

use vec3 as colour;
use vec3 as point3;

////////////////////////// UTILITY FUNCTIONS /////////////////////////
/// 
/// VEC

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

/// COLOUR
fn write_colour(colour: vec3){
    let ir = (255.999*colour.x) as i32;
    let ig = (255.999*colour.y) as i32;
    let ib = (255.999*colour.z) as i32;
    print!("{} {} {}\n", ir, ig, ib);
}

/// RAY

fn ray_colour(&ray: &Ray) -> colour{
    let unit_dir: vec3 = unit_vector(ray.dir);
    let t = 0.5*unit_dir.y+1.0;

    colour::new(1.0, 1.0, 1.0)*(1.0-t) + colour::new(0.5, 0.7, 1.0)*t
}

//////////////////////////////////////////////////////////////////////////////

fn main(){
    // IMAGE
    let aspect_ratio = 16.0/9.0;
    let image_width = 400;
    let image_height = image_width/aspect_ratio as u32;

    // Camera

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = point3::new(0.0, 0.0, 0.0);
    let horizontal = vec3::new(viewport_width, 0.0, 0.0);
    let vertical = vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - vec3::new(0.0, 0.0, focal_length);

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

            let u = i as f64 / (image_width-1) as f64;
            let v = j as f64 / (image_height-1) as f64;
            let r = Ray::new(origin, lower_left_corner + horizontal*u + vertical*v - origin);
            let pixel_color: colour = ray_colour(&r);
            write_colour(c);
        }
    }
    eprintln!("\nDone!");
}