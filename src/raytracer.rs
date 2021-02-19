use vector::vec3;
mod vector;

use ray::Ray;
mod ray;

use geometry::Sphere;
use geometry::Hittable;
mod geometry;

use vec3 as colour;
use vec3 as point3;

////////////////////////// UTILITY FUNCTIONS /////////////////////////

/// COLOUR
fn write_colour(colour: vec3){
    let ir = (255.999*colour.x) as i32;
    let ig = (255.999*colour.y) as i32;
    let ib = (255.999*colour.z) as i32;
    print!("{} {} {}\n", ir, ig, ib);
}

/// RAY

fn ray_colour(&ray: &Ray) -> colour{
    let s: Sphere = Sphere::new(point3::new(0.0,0.0,-1.0), 0.5);
    let mut hr = geometry::HitRecord{p: point3::new(0.0,0.0,0.0), normal: vec3::new(0.0,0.0,0.0), t: 0.0, front_face:true};

    let did_hit = s.hit(&ray, -10.0, 10.0, &mut hr); //TODO
    if did_hit { //hit sphere
        let N: vec3 = vec3::unit_vector(ray.at(hr.t)-vec3::new(0.0,0.0,-1.0));
        return colour::new(N.x+1.0, N.y+1.0, N.z+1.0)*0.5;
    }
    let unit_dir: vec3 = vec3::unit_vector(ray.dir);
    let t = 0.5*unit_dir.y+1.0;

    colour::new(1.0, 1.0, 1.0)*(1.0-t) + colour::new(0.5, 0.7, 1.0)*t
}

//////////////////////////////////////////////////////////////////////////////

fn main(){
    // IMAGE
    let aspect_ratio = 16.0/9.0 as f64;
    let image_width = 400;
    let image_height = (image_width as f64/aspect_ratio) as u32;
    eprintln!("W: {}, H: {}", image_width, image_height);

    // Camera

    let viewport_height = 2.0f64;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = point3::new(0.0, 0.0, 0.0);
    let horizontal = vec3::new(viewport_width, 0.0, 0.0);
    let vertical = vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - vec3::new(0.0, 0.0, focal_length);

    print!("P3\n{} {}\n255\n", image_width, image_height);
    for j in (0 .. image_height).rev(){
        // Debug msg
        eprint!("\rScanlines remaining: {}     ", j);
        for i in 0..image_width{
            let u = i as f64 / (image_width-1) as f64;
            let v = j as f64 / (image_height-1) as f64;
            let r = Ray::new(origin, lower_left_corner + horizontal*u + vertical*v - origin);
            let px_colour: colour = ray_colour(&r);
            write_colour(px_colour);
        }
    }
    eprintln!("\nDone!");
}