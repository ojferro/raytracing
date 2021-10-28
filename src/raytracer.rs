extern crate minifb;
use minifb::{Key, Window, WindowOptions};

use rand::Rng;

use clap::{Arg, App};

use std::thread;
use std::sync::Arc;
use std::io::Write;
use crossbeam::{unbounded, TryRecvError};

use scene::Scene;
mod scene;

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
const USE_BUFFER: bool = false;
/// COLOUR

fn rand_f()->f64{
    rand::thread_rng().gen()
}

fn clamp(x: f64, min: f64, max: f64) -> f64{
    if x<min {return min;}
    if x>max {return max;}
    return x;
}

fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

fn write_colour(mut colour: colour, buffer: &mut Vec<u32>, i: usize, row: usize, percent_done: f64, total_spp: usize, image_width: usize, image_height: usize){
    colour = colour/(total_spp as f64*percent_done);
    
    let ir = (256.0*clamp(colour.x, 0.0, 0.999)) as u8;
    let ig = (256.0*clamp(colour.y, 0.0, 0.999)) as u8;
    let ib = (256.0*clamp(colour.z, 0.0, 0.999)) as u8;

    buffer[row*image_width + i] = from_u8_rgb(ir, ig, ib);
}

fn write_to_vec(mut colour: colour, total_spp: f64, img_vec: &mut Vec<Vec<colour>>, col: usize, row: usize){
    img_vec[row][col] += colour;
}

fn write_to_window(window: &mut minifb::Window, buffer: &mut Vec<u32>, img_vec: &Vec<Vec<colour>>, percent_done: f64, total_spp: usize, width: usize, height: usize){
    for row in 0..height{
        for col in 0..width{
            write_colour(img_vec[row][col], buffer, col, row, percent_done, total_spp, width, height);
        }
    }

    if window.is_open() {
        window
            .update_with_buffer(&buffer, width, height)
            .unwrap();
    }
}

fn write_to_file(img_vec: &Vec<Vec<colour>>, total_spp: usize, width: usize, height: usize){
    print!("P3\n{} {}\n255\n", width, height);
    for row in 0..height{
        for col in 0..width{
            let colour = img_vec[row][col]/(total_spp as f64);
    
            let ir = (256.0*clamp(colour.x, 0.0, 0.999)) as u8;
            let ig = (256.0*clamp(colour.y, 0.0, 0.999)) as u8;
            let ib = (256.0*clamp(colour.z, 0.0, 0.999)) as u8;

            print!("{} {} {}\n", ir, ig, ib);
        }
    }
}

/// RAY

fn ray_colour(&ray: &Ray, scene: &HittableList, ray_bounces: usize, gamma_correction: bool) -> colour{
    if ray_bounces <=0{ return colour::new(0.0, 0.0, 0.0);}

    let mut hr = geometry::HitRecord::default();

    let mut attenuation = colour::new(0.0,0.0,0.0);
    let max_ray_len = f64::INFINITY;
    if let Some(r) = scene.hit(&ray, &mut attenuation, 0.001, max_ray_len, &mut hr) { //hit anything in scene
        // Compute Lambertian reflection
        // TODO: Use r instead of recalculating it
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
    // Args
    let args = App::new("Raytracer")
        .arg(Arg::with_name("samples_per_px")
                 .short("s")
                 .long("spp")
                 .takes_value(true)
                 .help("Number of samples per pixel"))
        .arg(Arg::with_name("image_width")
                 .short("w")
                 .long("width")
                 .takes_value(true)
                 .help("Width of the image (16x9 aspect ratio)"))
        .get_matches();

    let spp = args.value_of("samples_per_px").unwrap_or("10");
    let samples_per_px = spp.parse::<usize>().expect("Number of samples per px must be a number!");

    let img_width = args.value_of("image_width").unwrap_or("640");
    let image_width = img_width.parse::<usize>().expect("Image width must be a number!");
    
    // IMAGE
    let aspect_ratio = 16.0/9.0 as f64;
    let image_height = (image_width as f64/aspect_ratio) as usize;

    /////////// SET UP DISPAY /////////////
    let mut img_buffer: Vec<u32> = vec![0; image_width * image_height];
    let mut img_vec: Vec<Vec<colour>> = vec![vec![vec3::new(0.0,0.0,0.0); image_width]; image_height];
    let mut window = Window::new("Test - ESC to exit", image_width as usize, image_height as usize, WindowOptions::default())
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });
    ///////////////////////////////////////
    
    // Camera

    let cam_origin = point3::new(1.0,1.30,3.0);
    let look_at = vec3::new(0.25,0.60,-0.50);
    let mut cam = Camera::new(
        27.0,
        16.0/9.0 as f64,
        0.05,//0.12,
        (cam_origin - look_at).length(),
        cam_origin,
        look_at,
        vec3::new(0.0,1.0,0.0),
        samples_per_px as u32);

    let max_ray_bounces = 10;
    let gamma_correction = true;

    // Scene
    let mut scene = Scene::get_scene();

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
            gamma_correction: gamma_correction
        };

        let h = thread::spawn(move || {
            
            let start_time = std::time::SystemTime::now();
            calculate_some_pxls(context.thread_id, context.num_threads,  &(*context.scene), &context.cam, &context.sender, context.image_height,context.image_width,
                context.samples_per_px, context.max_ray_bounces, context.gamma_correction);
            if let Ok(elapsed) = start_time.elapsed(){
                eprintln!("\nThread: {}, signing off. t: {}", context.thread_id, elapsed.as_millis());
            }

        });
        thread_handles.push(h);
    }

    let total_num_pxls = image_width*image_height*samples_per_px;
    let mut ctr=0;
    loop{
        match receiver.try_recv() {
            Ok(received) => {
                write_to_vec(received.c, samples_per_px as f64, &mut img_vec, received.col, received.row);
                ctr += 1;
            }
            Err(TryRecvError::Disconnected)  =>{ println!("\nINFO: Thread disconnected or finished."); }
            Err(TryRecvError::Empty)=> { }

        }
        

        if ctr%(image_width*num_threads)==0{
            let percent_done = ctr as f64/total_num_pxls as f64;
            write_to_window(&mut window, &mut img_buffer, &img_vec, percent_done, samples_per_px, image_width, image_height);
            eprint!("\r{:.2}% done.      ", percent_done*100.0);
            std::io::stdout().flush().unwrap();
        }
        if ctr == total_num_pxls{
            break;
        }
    }

    write_to_window(&mut window, &mut img_buffer, &img_vec, 1.0, samples_per_px, image_width as usize, image_height as usize);

    for t in thread_handles{
        t.join().unwrap();
    }

    eprintln!("\nDone!");
    write_to_file(&img_vec, samples_per_px, image_width as usize, image_height as usize);
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
    gamma_correction: bool
}

struct PxData{
    c: colour,
    row: usize,
    col: usize,
    num_samples: usize
}

fn calculate_some_pxls(thread_id: usize, num_threads: usize, _scene: &HittableList, cam: &Camera, sender: &crossbeam::Sender<PxData>,
    image_height: usize, image_width: usize, samples_per_px: usize, max_ray_bounces: usize, gamma_correction: bool){
    
    for current_spp in 0..cam.samples_per_px {
        for j in (thread_id .. image_height).step_by(num_threads){
            let scene = Scene::get_scene();
            for i in 0..image_width{
                let mut px_colour = colour::new(0.0, 0.0, 0.0);
                // TODO: Improve aliasing. Make non-random.
                // TODO: Make anti-aliasing be a second stage process (i.e. have non-aliased preliminary result, then anti-alias).
            
                let u = (i as f64 + rand_f()) / (image_width-1) as f64;
                let v = (j as f64 + rand_f()) / (image_height-1) as f64;
                let r = cam.get_ray(u, v);
                px_colour = ray_colour(&r, &scene, max_ray_bounces, gamma_correction);

                let row = image_height-1-j; //Flip the image vertically

                let px_data = PxData{c: px_colour, row: row, col: i, num_samples: current_spp as usize};
                sender.send(px_data).unwrap();
            }
        }
    }
}