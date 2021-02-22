use vector::vec3;
mod vector;

use ray::Ray;
mod ray;

use camera::Camera;
mod camera;

use geometry::Sphere;
use geometry::Hittable;
use geometry::HittableList;
mod geometry;

use rand::Rng;

use vec3 as colour;
use vec3 as point3;

// const ANTIALISAING: bool = true;

////////////////////////// UTILITY FUNCTIONS /////////////////////////

/// COLOUR

fn rand_f()->f64{
    rand::thread_rng().gen()
}

fn clamp(x: f64, min: f64, max: f64) -> f64{
    if x<min {return min;}
    if x>max {return max;}
    return x;
}

fn write_colour(mut colour: colour, samples_per_px: u32){
    let scale = 1.0/samples_per_px as f64;
    colour = colour*scale;
    
    let ir = (256.0*clamp(colour.x, 0.0, 0.999)) as i32;
    let ig = (256.0*clamp(colour.y, 0.0, 0.999)) as i32;
    let ib = (256.0*clamp(colour.z, 0.0, 0.999)) as i32;
    print!("{} {} {}\n", ir, ig, ib);
}

/// RAY

fn ray_colour(&ray: &Ray, scene: &Hittable, ray_bounces: usize) -> colour{
    if ray_bounces <=0{ return colour::new(0.0, 0.0, 0.0);}

    let mut hr = geometry::HitRecord{p: point3::new(0.0,0.0,0.0), normal: vec3::new(0.0,0.0,0.0), t: 0.0, front_face:true};

    if scene.hit(&ray, 0.001, f64::INFINITY, &mut hr) { //hit anything in scene
        let target: point3 = hr.p + hr.normal + vec3::random_in_unit_sphere();
        return ray_colour(&Ray::new(hr.p, target-hr.p), scene, ray_bounces-1)*0.5;
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
    let samples_per_px = 100;
    let mut cam = Camera::new(aspect_ratio, viewport_height, focal_length, origin, samples_per_px);
    let max_ray_bounces = 50;

    // Scene
    let mut scene = HittableList::new();
    scene.add(Box::new(Sphere::new(point3::new(0.0,0.0,-1.0), 0.5)));
    scene.add(Box::new(Sphere::new(point3::new(0.0,-100.5,-1.0), 100.0)));
    // TODO: Writing to file makes runtime increase 60x. Write to mem instead, and offload writing to file.
    print!("P3\n{} {}\n255\n", image_width, image_height);
    for j in (0 .. image_height).rev(){
        // Debug msg
        eprint!("\rScanlines remaining: {}     ", j);
        for i in 0..image_width{
            let mut px_colour = colour::new(0.0, 0.0, 0.0);
            if samples_per_px > 1{
                // TODO: Improve aliasing. Make non-random.
                // TODO: Make anti-aliasing be a second stage process (i.e. have non-aliased preliminary result, then anti-alias).
                for s in 0..cam.samples_per_px {
                    let u = (i as f64 + rand_f()) / (image_width-1) as f64;
                    let v = (j as f64 + rand_f()) / (image_height-1) as f64;
                    let r = cam.get_ray(u, v);
                    px_colour += ray_colour(&r, &scene, max_ray_bounces);
                }
                write_colour(px_colour, cam.samples_per_px);
            }else{
                let u = i as f64 / (image_width-1) as f64;
                let v = j as f64 / (image_height-1) as f64;
                let r = Ray::new(origin, cam.lower_left_corner + cam.horizontal*u + cam.vertical*v - origin);
                let px_colour: colour = ray_colour(&r, &scene, max_ray_bounces);

                write_colour(px_colour, cam.samples_per_px);
            }
        }
    }
    eprintln!("\nDone!");
}