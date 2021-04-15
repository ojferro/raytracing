extern crate minifb;
use minifb::{Key, Window, WindowOptions};

use rand::Rng;

use std::thread;
use std::sync::Arc;
use crossbeam::{unbounded, TryRecvError};
// use crossbeam::crossbeam_utils::thread;

use denoising::BlueNoise;
mod denoising;

use vector::vec3;
mod vector;

use ray::Ray;
mod ray;

use camera::Camera;
mod camera;

use geometry::*;
mod geometry;

use vec3 as colour;
use vec3 as point3;


////////////////////////// UTILITY FUNCTIONS /////////////////////////
const USE_BUFFER: bool = true;
/// COLOUR

fn clamp(x: f64, min: f64, max: f64) -> f64{
    if x<min {return min;}
    if x>max {return max;}
    return x;
}

fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

fn write_colour(mut colour: colour, samples_per_px: usize, buffer: &mut Vec<u32>, i: usize, row: usize, image_width: usize, image_height: usize){
    let scale = 1.0/samples_per_px as f64;
    colour = colour*scale;
    
    let ir = (256.0*clamp(colour.x, 0.0, 0.999)) as u8;
    let ig = (256.0*clamp(colour.y, 0.0, 0.999)) as u8;
    let ib = (256.0*clamp(colour.z, 0.0, 0.999)) as u8;

    buffer[row*image_width + i] = from_u8_rgb(ir, ig, ib);
}

fn write_to_window(window: &mut minifb::Window, buffer: &mut Vec<u32>, width: usize, height: usize){
    if window.is_open() {

        window
            .update_with_buffer(&buffer, width, height)
            .unwrap();
    }
}

/// RAY

fn ray_colour(&ray: &Ray, scene: &HittableList, ray_bounces: usize, gamma_correction: bool, pixel_data: (usize,usize,usize)) -> colour{
    if ray_bounces <=0{ return colour::new(0.0, 0.0, 0.0);}

    let mut hr = geometry::HitRecord::default();

    let mut attenuation = colour::new(0.0,0.0,0.0);
    let max_ray_len = f64::INFINITY;
    if let Some(r) = scene.hit(&ray, &mut attenuation, 0.001, max_ray_len, &mut hr, pixel_data) { //hit anything in scene
        // Compute Lambertian reflection
        // TODO: Use r instead of recalculating it
        // let target: point3 = hr.p + hr.normal + vec3::random_unit_vector();
        return attenuation*ray_colour(&r, scene, ray_bounces-1, gamma_correction, pixel_data);
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

    // use image::*;
    // let im = image::open("data/128_128_tile.png").unwrap();
    // let pxls = im.raw_pixels().to_vec();
    // let mut out = vec![];
    // // print!("{}", pxls.len());
    // for p in pxls{
    //     // println!("{:?}", p);
    //     out.push(p as f64/255.0);
    // }
    // println!("{:?}", out);
    // return ();

    // use spherical_blue_noise::*;

    // let blue_noise_vec: Vec<(f32, f32, f32)> = BlueNoiseSphere::new(256, &mut rand::thread_rng()).into_iter().collect();
    // println!("{:?}", blue_noise_vec);
    // return ();


    // IMAGE
    let aspect_ratio = 16.0/9.0 as f64;
    let image_width: usize = 1080;
    let image_height = (image_width as f64/aspect_ratio) as usize;

    /////////// SET UP DISPAY /////////////
    let mut img_buffer: Vec<u32> = vec![0; image_width * image_height];
    let mut window = Window::new("Test - ESC to exit", image_width as usize, image_height as usize, WindowOptions::default())
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });
    ///////////////////////////////////////

    // eprintln!("W: {}, H: {}", image_width, image_height);

    // Camera
    let samples_per_px: usize = 24;

    let cam_origin = point3::new(1.0,1.30,3.0);
    let look_at = vec3::new(0.25,0.60,-0.50);
    let mut cam = Camera::new(
        27.0,
        16.0/9.0 as f64,
        0.1,
        (cam_origin - look_at).length(),
        cam_origin,
        look_at,
        vec3::new(0.0,1.0,0.0),
        samples_per_px as u32);

    let max_ray_bounces = 10;
    let gamma_correction = true;

    // Scene
    let mut scene = HittableList::new();

    // Yellow fuzzy metal sphere
    let m1 = Box::new(geometry::Metal{albedo: colour::new(0.8, 0.6, 0.2), fuzz: 0.25});
    let radius = 0.5;
    scene.add(Box::new(Sphere::new(point3::new(0.80, radius, -1.0), radius, m1)));

    // Red diffuse sphere
    let m2: Box<dyn Material> = Box::new(geometry::Lambertian{albedo: colour::new(0.7, 0.3, 0.3)});
    let radius = 0.25;
    scene.add(Box::new(Sphere::new(point3::new(-0.10, radius, -0.10), radius, m2)));

    // Shiny metal sphere
    let m3: Box<dyn Material> = Box::new(geometry::Metal{albedo: colour::new(0.8, 0.8, 0.8), fuzz: 0.0});
    let radius = 0.5;
    scene.add(Box::new(Sphere::new(point3::new(-0.80, radius, -1.0), radius, m3)));

    // Solid glass sphere
    let m4: Box<dyn Material> = Box::new(geometry::Dielectric{albedo: colour::new(1.0,1.0,1.0), index_of_refraction: 1.5});
    let radius = 0.1;
    scene.add(Box::new(Sphere::new(point3::new(0.25, 0.75, -0.5), radius, m4)));

    // Hollow glass sphere
    let m5: Box<dyn Material> = Box::new(geometry::Dielectric{albedo: colour::new(0.95,0.95,1.0), index_of_refraction: 1.5});
    scene.add(Box::new(Sphere::new(point3::new(-0.25, 0.75, -0.42), 0.14, m5)));
    let m5: Box<dyn Material> = Box::new(geometry::Dielectric{albedo: colour::new(0.95,0.95,1.0), index_of_refraction: 1.5});
    scene.add(Box::new(Sphere::new(point3::new(-0.25, 0.75, -0.42), -0.13, m5)));

    // Cube!
    let m6: Box<dyn Material> = Box::new(geometry::Lambertian{albedo: colour::new(0.7, 0.3, 0.7)});
    let w = 0.50; let h = 0.50; let d = 0.50;
    scene.add(Box::new(Cube::new(point3::new(0.0, 0.5, -1.0), w,h,d, m6)));

    // Plane
    let m6: Box<dyn Material> = Box::new(geometry::Lambertian{albedo: colour::new(0.3, 0.3, 0.3)});
    let single_sided = true;
    scene.add(Box::new(Plane::new(point3::new(0.0,1.0,0.0), point3::new(0.0,0.0,0.0), m6, single_sided)));

    if !USE_BUFFER{ print!("P3\n{} {}\n255\n", image_width, image_height);}

    let num_threads = num_cpus::get()*10;

    let (sender, receiver) = unbounded();
    let scene_arc = Arc::new(scene);
    let mut thread_handles = Vec::with_capacity(num_threads);

    for i in 0..num_threads{
        let sender_clone = sender.clone();
        let scene_clone = scene_arc.clone();

        let context = ThreadContext{
            thread_id: i,
            num_threads: num_threads,
            sender: sender_clone,
            scene: scene_clone,
            cam: cam.clone(),
            image_height: image_height,
            image_width: image_width,
            samples_per_px: samples_per_px,
            max_ray_bounces: max_ray_bounces,
            gamma_correction: gamma_correction,
            blue_noise_disc: BlueNoise::get_disc().clone()
        };

        let h = thread::spawn(move || {
            
            // let start_time = std::time::SystemTime::now();
            calculate_some_pxls(context.thread_id, context.num_threads,  &(*context.scene), &context.cam, &context.sender, context.image_height,context.image_width,
                context.samples_per_px, context.max_ray_bounces, context.gamma_correction, context.blue_noise_disc);
            // if let Ok(elapsed) = start_time.elapsed(){
            //     // println!("\nThread: {}, signing off. t: {}", context.thread_id, elapsed.as_millis());
            // }

        });
        thread_handles.push(h);
    }

    let total_num_pxls = image_width*image_height;
    let mut ctr=0;
    loop{
        match receiver.try_recv() {
            Ok(received) => {
                write_colour(received.c, received.num_samples, &mut img_buffer, received.col, received.row, image_width, image_height);
                ctr += 1;
            }
            Err(TryRecvError::Disconnected)  =>{ println!("\nINFO: Thread disconnected or finished."); }
            Err(TryRecvError::Empty)=> { }

        }
        

        // if ctr%(image_width*2)==0{
        //     write_to_window(&mut window, &mut img_buffer, image_width, image_height);
        // }
        if ctr == total_num_pxls{
            break;
        }
    }

    write_to_window(&mut window, &mut img_buffer, image_width as usize, image_height as usize);

    for t in thread_handles{
        t.join().unwrap();
    }

    // eprintln!("\nDone!");
    while  window.is_open() && !window.is_key_down(Key::Escape) {}
}

struct ThreadContext{
    thread_id: usize,
    num_threads: usize,
    sender: crossbeam::Sender<PxData>,
    
    scene: Arc<HittableList>,
    cam: Camera,
    image_height: usize,
    image_width: usize,
    samples_per_px: usize,
    max_ray_bounces: usize,
    gamma_correction: bool,
    blue_noise_disc: Vec<(f64, f64)>
}

struct PxData{
    c: colour,
    row: usize,
    col: usize,
    num_samples: usize
}

// Use this function if you have a single thread. It calculates all pxls in the img.
// fn calculate_all_pxls(scene: &HittableList, cam: &Camera, sender: &crossbeam::Sender<PxData>,
//     image_height: usize, image_width: usize, samples_per_px: usize, max_ray_bounces: usize, gamma_correction: bool){
    
//     for j in (0 .. image_height).rev(){
//         // Debug msg
//         // eprint!("\rScanlines remaining: {}     ", j);
//         for i in 0..image_width{
//             let mut px_colour = colour::new(0.0, 0.0, 0.0);
//             // TODO: Improve aliasing. Make non-random.
//             // TODO: Make anti-aliasing be a second stage process (i.e. have non-aliased preliminary result, then anti-alias).
//             for s in 0..cam.samples_per_px {
//                 let u = (i as f64 + rand_f()) / (image_width-1) as f64;
//                 let v = (j as f64 + rand_f()) / (image_height-1) as f64;
//                 let r = cam.get_ray(u, v);
//                 px_colour += ray_colour(&r, scene, max_ray_bounces, gamma_correction);
//             }
//             let row;
//             if USE_BUFFER{ row = image_height-1-j; }else{ row = j;}

//             let px_data = PxData{c: px_colour, row: row, col: i, num_samples: samples_per_px};
//             sender.send(px_data).unwrap();
            
//         }
//     }
// }

fn calculate_some_pxls(thread_id: usize,
    num_threads: usize,
    scene: &HittableList,
    cam: &Camera,
    sender: &crossbeam::Sender<PxData>,
    image_height: usize,
    image_width: usize,
    samples_per_px: usize,
    max_ray_bounces: usize,
    gamma_correction: bool,
    blue_noise_disc: Vec<(f64,f64)>){
    
    for j in (thread_id .. image_height).step_by(num_threads){
        // Debug msg
        // eprint!("\rScanlines remaining: {}     ", j);
        for i in 0..image_width{
            let mut px_colour = colour::new(0.0, 0.0, 0.0);
            // TODO: Improve aliasing. Make non-random.
            // TODO: Make anti-aliasing be a second stage process (i.e. have non-aliased preliminary result, then anti-alias).
            for s in 0..cam.samples_per_px {
                // let offset_x = blue_noise_disc[(s%blue_noise_disc.len() as u32) as usize].0;
                // let offset_y = blue_noise_disc[(s%blue_noise_disc.len() as u32) as usize].1;
                let (offset_x, offset_y) = BlueNoise::random_in_disc();
                let u = (i as f64 + offset_x) / (image_width-1) as f64;
                let v = (j as f64 + offset_y) / (image_height-1) as f64;

                let (a,b) = BlueNoise::random_in_disc();
                let for_depth_of_field = vec3::new(a, b, 0.0);
                // let sp_bn = BlueNoise::get_screenspace(i,j,image_width);
                // let mut cleanup_for_dof = BlueNoise::blue_noise_cleanup((i,j,image_width));
                // cleanup_for_dof.z = 0.0;
                let r = cam.get_ray(u, v, for_depth_of_field);
                // let a = BlueNoise::get_screenspace((i, j, image_width));
                px_colour += ray_colour(&r, scene, max_ray_bounces, gamma_correction, (i,j,image_width));
            }
            let row;
            if USE_BUFFER{ row = image_height-1-j; }else{ row = j;}

            // write_colour(px_colour, cam.samples_per_px, img_buffer, i, row, image_width, image_height);
            let px_data = PxData{c: px_colour, row: row, col: i, num_samples: samples_per_px};
            sender.send(px_data).unwrap();
        }
    }
}