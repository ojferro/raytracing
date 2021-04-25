extern crate minifb;
use minifb::{Key, Window, WindowOptions};

use std::thread;
use std::sync::Arc;
use crossbeam::{bounded, TryRecvError};

use scene::Scene;
mod scene;

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
const NUM_FRAMES_TO_RENDER: u32 = 10;
/// COLOUR

fn clamp(x: f32, min: f32, max: f32) -> f32{
    if x<min {return min;}
    if x>max {return max;}
    return x;
}

fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

fn write_colour(mut colour: colour, samples_per_px: usize, buffer: &mut Vec<u32>, i: usize, row: usize, image_width: usize, image_height: usize){
    let scale = 1.0/samples_per_px as f32;
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
    let max_ray_len = f32::INFINITY;
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
    //     out.push(p as f32/255.0);
    // }
    // println!("{:?}", out);
    // return ();

    // use spherical_blue_noise::*;

    // let blue_noise_vec: Vec<(f32, f32, f32)> = BlueNoiseSphere::new(256, &mut rand::thread_rng()).into_iter().collect();
    // println!("{:?}", blue_noise_vec);
    // return ();


    // IMAGE
    let aspect_ratio = 16.0/9.0 as f32;
    let image_width: usize = 600;
    let image_height = (image_width as f32/aspect_ratio) as usize;

    /////////// SET UP DISPAY /////////////
    let mut img_buffer: Vec<u32> = vec![0; image_width * image_height];
    let mut window = Window::new("Test - ESC to exit", image_width as usize, image_height as usize, WindowOptions::default())
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });
    ///////////////////////////////////////

    let mut cam = Scene::get_camera();

    let max_ray_bounces = 10;
    let gamma_correction = true;

    // Scene
    let mut scene = Scene::get_scene();

    if !USE_BUFFER{ print!("P3\n{} {}\n255\n", image_width, image_height);}

    let num_threads = num_cpus::get();

    let (sender, receiver) = bounded(10000);
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
            samples_per_px: cam.samples_per_px as usize,
            max_ray_bounces: max_ray_bounces,
            gamma_correction: gamma_correction,
            blue_noise_disc: BlueNoise::get_disc().clone()
        };

        let h = thread::spawn(move || {
            for i in 0..NUM_FRAMES_TO_RENDER {
                calculate_some_pxls(context.thread_id, context.num_threads,  &(*context.scene), &context.cam, &context.sender, context.image_height,context.image_width,
                    context.samples_per_px, context.max_ray_bounces, context.gamma_correction, &context.blue_noise_disc);
            }
        });
        thread_handles.push(h);
    }

    let total_num_pxls = image_width*image_height;
    let mut ctr=0;
    let mut start_time = std::time::SystemTime::now();
    loop{
        match receiver.try_recv() {
            Ok(received) => {
                write_colour(received.c, received.num_samples, &mut img_buffer, received.col, received.row, image_width, image_height);
                ctr += 1;
            }
            Err(TryRecvError::Disconnected)  =>{ println!("\nINFO: Thread disconnected or finished."); }
            Err(TryRecvError::Empty)=> { }

        }
        

        if ctr%(image_width*image_height)==0{
            write_to_window(&mut window, &mut img_buffer, image_width, image_height);
            Scene::move_cam();
            if let Ok(elapsed) = start_time.elapsed(){
                println!("FPS: {}", 1000.0/elapsed.as_millis() as f32);
            }
            start_time = std::time::SystemTime::now();
        }
        if ctr == total_num_pxls*NUM_FRAMES_TO_RENDER as usize{
            break;
        }
    }

    write_to_window(&mut window, &mut img_buffer, image_width as usize, image_height as usize);

    for t in thread_handles{
        t.join().unwrap();
    }

    // while  window.is_open() && !window.is_key_down(Key::Escape) {}
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
    blue_noise_disc: Vec<(f32, f32)>
}

struct PxData{
    c: colour,
    row: usize,
    col: usize,
    num_samples: usize
}

fn calculate_some_pxls(thread_id: usize,
    num_threads: usize,
    _scene: &HittableList,
    _cam: &Camera,
    sender: &crossbeam::Sender<PxData>,
    image_height: usize,
    image_width: usize,
    samples_per_px: usize,
    max_ray_bounces: usize,
    gamma_correction: bool,
    blue_noise_disc: &Vec<(f32,f32)>){

    
    for j in (thread_id .. image_height).step_by(num_threads){
        let scene = Scene::get_scene();
        let cam = Scene::get_camera();
        for i in 0..image_width{
            let mut px_colour = colour::new(0.0, 0.0, 0.0);
            // TODO: Make anti-aliasing be a second stage process (i.e. have non-aliased preliminary result, then anti-alias).
            for s in 0..cam.samples_per_px {
                // let offset_x = blue_noise_disc[(s%blue_noise_disc.len() as u32) as usize].0;
                // let offset_y = blue_noise_disc[(s%blue_noise_disc.len() as u32) as usize].1;
                let (offset_x, offset_y) = BlueNoise::random_in_disc();
                let u = (i as f32 + offset_x) / (image_width-1) as f32;
                let v = (j as f32 + offset_y) / (image_height-1) as f32;

                let (a,b) = BlueNoise::random_in_disc();
                let for_depth_of_field = vec3::new(a, b, 0.0);

                let r = cam.get_ray(u, v, for_depth_of_field);
                px_colour += ray_colour(&r, &scene, max_ray_bounces, gamma_correction, (i,j,image_width));
            }
            let row;
            if USE_BUFFER{ row = image_height-1-j; }else{ row = j;}

            let px_data = PxData{c: px_colour, row: row, col: i, num_samples: samples_per_px};
            sender.send(px_data).unwrap();
        }
    }
}