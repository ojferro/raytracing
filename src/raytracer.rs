extern crate minifb;
use minifb::{Key, Window, WindowOptions};

use rand::Rng;

use std::thread;
use std::sync::Arc;
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
const USE_BUFFER: bool = true;
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
    // IMAGE
    let aspect_ratio = 16.0/9.0 as f64;
    let image_width: usize = 500;
    let image_height = (image_width as f64/aspect_ratio) as usize;

    /////////// SET UP DISPAY /////////////
    let mut img_buffer: Vec<u32> = vec![0; image_width * image_height];
    let mut window = Window::new("Test - ESC to exit", image_width as usize, image_height as usize, WindowOptions::default())
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });
    ///////////////////////////////////////
    
    eprintln!("W: {}, H: {}", image_width, image_height);

    // Camera
    let samples_per_px: usize = 100;

    let cam_origin = point3::new(1.0,1.30,3.0);
    let look_at = vec3::new(0.25,0.60,-0.50);
    let mut cam = Camera::new(
        27.0,
        16.0/9.0 as f64,
        0.12,
        (cam_origin - look_at).length(),
        cam_origin,
        look_at,
        vec3::new(0.0,1.0,0.0),
        samples_per_px as u32);

    let max_ray_bounces = 10;
    let gamma_correction = true;

    // Scene
    let mut scene = Scene::get_scene();

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
            gamma_correction: gamma_correction
        };

        let h = thread::spawn(move || {
            
            let start_time = std::time::SystemTime::now();
            calculate_some_pxls(context.thread_id, context.num_threads,  &(*context.scene), &context.cam, &context.sender, context.image_height,context.image_width,
                context.samples_per_px, context.max_ray_bounces, context.gamma_correction);
            if let Ok(elapsed) = start_time.elapsed(){
                println!("\nThread: {}, signing off. t: {}", context.thread_id, elapsed.as_millis());
            }

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
        

        if ctr%(image_width*2)==0{
            write_to_window(&mut window, &mut img_buffer, image_width, image_height);
        }
        if ctr == total_num_pxls{
            break;
        }
    }

    write_to_window(&mut window, &mut img_buffer, image_width as usize, image_height as usize);

    for t in thread_handles{
        t.join().unwrap();
    }

    eprintln!("\nDone!");
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
    
    for j in (thread_id .. image_height).step_by(num_threads){
        let scene = Scene::get_scene();
        for i in 0..image_width{
            let mut px_colour = colour::new(0.0, 0.0, 0.0);
            if samples_per_px > 1{
                // TODO: Improve aliasing. Make non-random.
                // TODO: Make anti-aliasing be a second stage process (i.e. have non-aliased preliminary result, then anti-alias).
                for _s in 0..cam.samples_per_px {
                    let u = (i as f64 + rand_f()) / (image_width-1) as f64;
                    let v = (j as f64 + rand_f()) / (image_height-1) as f64;
                    let r = cam.get_ray(u, v);
                    px_colour += ray_colour(&r, &scene, max_ray_bounces, gamma_correction);
                }
                let row;
                if USE_BUFFER{ row = image_height-1-j; }else{ row = j;}

                let px_data = PxData{c: px_colour, row: row, col: i, num_samples: samples_per_px};
                sender.send(px_data).unwrap();
            }
        }
    }
}