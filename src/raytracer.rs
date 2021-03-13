use rand::Rng;

use vector::vec3;
mod vector;

use ray::Ray;
mod ray;

use camera::Camera;
mod camera;

use geometry::Sphere;
use geometry::Hittable;
use geometry::HittableList;
use geometry::Material;
use geometry::*;
mod geometry;

use image::PPM;
use image::RGB;
mod image;


use vec3 as colour;
use vec3 as point3;

// const ANTIALISAING: bool = true;
const USE_BUFFER: bool = true; // Use buffer instead of outputting directly to file

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

fn write_colour(mut colour: colour, samples_per_px: u32, buffer: &mut PPM, i: u32, j: u32){
    let scale = 1.0/samples_per_px as f64;
    colour = colour*scale;
    
    let ir = (256.0*clamp(colour.x, 0.0, 0.999)) as u8;
    let ig = (256.0*clamp(colour.y, 0.0, 0.999)) as u8;
    let ib = (256.0*clamp(colour.z, 0.0, 0.999)) as u8;

    if !USE_BUFFER{
        // TODO: only about 4% faster to use buffer. Consider removing for simplicity.
        print!("{} {} {}\n", ir, ig, ib);
    }else{
        buffer.set_pixel(i, j, RGB{r: ir, g: ig, b: ib});
    }
}

/// RAY

fn ray_colour(&ray: &Ray, scene: &dyn Hittable, ray_bounces: usize, gamma_correction: bool) -> colour{
    if ray_bounces <=0{ return colour::new(0.0, 0.0, 0.0);}

    let mut hr = geometry::HitRecord::default();

    let mut attenuation = colour::new(0.0,0.0,0.0);
    if let Some(r) = scene.hit(&ray, &mut attenuation, 0.001, f64::INFINITY, &mut hr) { //hit anything in scene
        // Compute Lambertian reflection
        // TODO: Use r instead of recalculating it
        // let target: point3 = hr.p + hr.normal + vec3::random_unit_vector();
        return attenuation*ray_colour(&r, scene, ray_bounces-1, gamma_correction);
    }
    let unit_dir: vec3 = vec3::unit_vector(ray.dir);
    let t = 0.5*unit_dir.y+1.0;

    let mut colour = colour::new(1.0, 1.0, 1.0)*(1.0-t) + colour::new(0.5, 0.7, 1.0)*t;

    if gamma_correction{
        colour.x = colour.x.sqrt();
        colour.y = colour.y.sqrt();
        colour.z = colour.z.sqrt();
    }
    colour
}

//////////////////////////////////////////////////////////////////////////////

fn main(){
    // IMAGE
    let aspect_ratio = 16.0/9.0 as f64;
    let image_width: u32 = 200;
    let image_height = (image_width as f64/aspect_ratio) as u32;

    let mut img_buffer = PPM::new(image_height.clone(), image_width.clone());
    
    eprintln!("W: {}, H: {}", image_width, image_height);

    // Camera
    let origin = point3::new(0.0, 0.0, 0.0);
    let samples_per_px = 100;
    let mut cam = Camera{..Default::default()};
    cam.position_camera(point3::new(-1.0,1.0,1.0), point3::new(0.0, 0.0, -1.0), vec3::new(0.0,1.0,0.0));
    print!("{:?}\n", cam);
    cam.samples_per_px = samples_per_px;

    let max_ray_bounces = 50;
    let gamma_correction = true;

    // Scene
    let mut scene = HittableList::new();

    let m1: Box<dyn Material> = Box::new(geometry::Metal{albedo: colour::new(0.8, 0.6, 0.2), fuzz: 0.25});
    scene.add(Box::new(Sphere::new(point3::new(1.0, 0.0, -1.0), 0.5, m1)));

    let m2: Box<dyn Material> = Box::new(geometry::Lambertian{albedo: colour::new(0.7, 0.3, 0.3)});
    scene.add(Box::new(Sphere::new(point3::new(0.0, 0.0, -1.0), 0.5, m2)));

    let m3: Box<dyn Material> = Box::new(geometry::Metal{albedo: colour::new(0.8, 0.8, 0.8), fuzz: 0.0});
    scene.add(Box::new(Sphere::new(point3::new(-1.0, 0.0, -1.0), 0.5, m3)));

    // 3 glass spheres combined (looks like cracked glass)
    let m4: Box<dyn Material> = Box::new(geometry::Dielectric{albedo: colour::new(1.0,1.0,1.0), index_of_refraction: 1.5});
    scene.add(Box::new(Sphere::new(point3::new(0.25, 0.15, -0.5), 0.1, m4)));
    let m4: Box<dyn Material> = Box::new(geometry::Dielectric{albedo: colour::new(1.0,1.0,1.0), index_of_refraction: 1.5});
    scene.add(Box::new(Sphere::new(point3::new(0.30, 0.15, -0.5), 0.1, m4)));
    let m4: Box<dyn Material> = Box::new(geometry::Dielectric{albedo: colour::new(1.0,1.0,1.0), index_of_refraction: 1.5});
    scene.add(Box::new(Sphere::new(point3::new(0.275, 0.125, -0.5), 0.1, m4)));

    // Hollow glass sphere
    let m5: Box<dyn Material> = Box::new(geometry::Dielectric{albedo: colour::new(0.95,0.95,1.0), index_of_refraction: 1.5});
    scene.add(Box::new(Sphere::new(point3::new(-0.25, 0.15, -0.42), 0.14, m5)));
    let m5: Box<dyn Material> = Box::new(geometry::Dielectric{albedo: colour::new(0.95,0.95,1.0), index_of_refraction: 1.5});
    scene.add(Box::new(Sphere::new(point3::new(-0.25, 0.15, -0.42), -0.13, m5)));

    let m_ground: Box<dyn Material> = Box::new(geometry::Lambertian{albedo: colour::new(0.8, 0.8, 0.0)});
    scene.add(Box::new(Sphere::new(point3::new(0.0,-100.5,-1.0), 100.0, m_ground)));
    // TODO: Writing to file makes runtime increase 60x. Write to mem instead, and offload writing to file.
    if !USE_BUFFER{ print!("P3\n{} {}\n255\n", image_width, image_height);}
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
                    px_colour += ray_colour(&r, &scene, max_ray_bounces, gamma_correction);
                }
                let row;
                if USE_BUFFER{ row = image_height-j; }else{ row = j;}
                write_colour(px_colour, cam.samples_per_px, &mut img_buffer, i, row);
            }else{
                let u = i as f64 / (image_width-1) as f64;
                let v = j as f64 / (image_height-1) as f64;
                let r = Ray::new(origin, cam.lower_left_corner + cam.horizontal*u + cam.vertical*v - origin);
                let px_colour: colour = ray_colour(&r, &scene, max_ray_bounces, gamma_correction);

                let row;
                if USE_BUFFER{ row = image_height-j; }else{ row = j;}
                write_colour(px_colour, cam.samples_per_px, &mut img_buffer, i, row);
            }
        }
    }
    eprintln!("\nDone!");
    img_buffer.write_file("image.ppm").expect("Error writing to file.");
}