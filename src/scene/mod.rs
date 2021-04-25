pub use self::scene::Scene;


mod scene{
    use crate::vector::vec3;
    use crate::geometry::*;
    use crate::camera::Camera;

    use vec3 as colour;
    use vec3 as point3;


    pub struct Scene {}
    impl Scene{
        pub fn get_scene() -> HittableList{
            let mut scene = HittableList::new();


            // Yellow fuzzy metal sphere
            let m1 = Box::new(Metal{albedo: colour::new(0.8, 0.6, 0.2), fuzz: 0.25});
            let radius = 0.5;
            scene.add(Box::new(Sphere::new(point3::new(0.80, radius, -1.0), radius, m1)));

            // Red diffuse sphere
            let m2: Box<dyn Material> = Box::new( Lambertian{albedo: colour::new(0.7, 0.3, 0.3)});
            let radius = 0.25;
            scene.add(Box::new(Sphere::new(point3::new(-0.10, radius, -0.10), radius, m2)));

            // Shiny metal sphere
            let m3: Box<dyn Material> = Box::new( Metal{albedo: colour::new(0.8, 0.8, 0.8), fuzz: 0.0});
            let radius = 0.5;
            scene.add(Box::new(Sphere::new(point3::new(-0.80, radius, -1.0), radius, m3)));

            // Solid glass sphere
            let m4: Box<dyn Material> = Box::new( Dielectric{albedo: colour::new(1.0,1.0,1.0), index_of_refraction: 1.5});
            let radius = 0.1;
            scene.add(Box::new(Sphere::new(point3::new(0.25, 0.75, -0.5), radius, m4)));

            // Hollow glass sphere
            let m5: Box<dyn Material> = Box::new( Dielectric{albedo: colour::new(0.95,0.95,1.0), index_of_refraction: 1.5});
            scene.add(Box::new(Sphere::new(point3::new(-0.25, 0.75, -0.42), 0.14, m5)));
            let m5: Box<dyn Material> = Box::new( Dielectric{albedo: colour::new(0.95,0.95,1.0), index_of_refraction: 1.5});
            scene.add(Box::new(Sphere::new(point3::new(-0.25, 0.75, -0.42), -0.13, m5)));

            // Cube!
            let m6: Box<dyn Material> = Box::new( Lambertian{albedo: colour::new(0.7, 0.3, 0.7)});
            let w = 0.50; let h = 0.50; let d = 0.50;
            scene.add(Box::new(Cube::new(point3::new(0.0, 0.5, -1.0), w,h,d, m6)));

            // Plane
            let m6: Box<dyn Material> = Box::new( Lambertian{albedo: colour::new(0.3, 0.3, 0.3)});
            let single_sided = true;
            scene.add(Box::new(Plane::new(point3::new(0.0,1.0,0.0), point3::new(0.0,0.0,0.0), m6, single_sided)));

            
            scene
        }

        pub fn get_camera() -> Camera{

            let cam_origin = unsafe{
                point3::new(1.0,1.30,4.0) - vec3::new(0.0, 0.0, COUNTER as f32*0.1)
            };
            let look_at = vec3::new(0.25,0.60,-0.50);
            let samples_per_px = 1;

            Camera::new(
                27.0,
                16.0/9.0 as f32,
                0.0,//0.1,
                (cam_origin - look_at).length(),
                cam_origin,
                look_at,
                vec3::new(0.0,1.0,0.0),
                samples_per_px as u32)
        }

        pub fn move_cam() {
            unsafe{
                COUNTER += 1;
                COUNTER %= 100;
            }
        }

    }

    static mut COUNTER: u32 = 0;
}