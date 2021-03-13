pub use self::camera::Camera;

mod camera{
    use crate::vector::vec3;
    use crate::ray::Ray;
    use vec3 as point3;

    #[derive(Copy, Clone)]
    pub struct Camera {
        pub v_fov: f64,
        pub aspect_ratio: f64,
        pub viewport_height: f64,
        pub viewport_width: f64,
        pub focal_length: f64,

        pub samples_per_px: u32,

        pub origin: point3,
        pub horizontal: vec3,
        pub vertical: vec3,
        pub lower_left_corner: vec3,
    }

    fn deg_to_rad(deg: f64) -> f64{
        deg*std::f64::consts::PI/180.0
    }

    impl Camera {
        pub fn new(v_fov: f64, aspect_ratio: f64, focal_length: f64, origin: vec3, samples_per_px: u32) -> Self {
            let theta = deg_to_rad(v_fov);
            let h = (theta/2.0).tan();

            let viewport_height = 2.0*h;
            let viewport_width = aspect_ratio.clone() * viewport_height.clone();

            let focal_length = focal_length;

            let horizontal = vec3::new(viewport_width.clone(), 0.0, 0.0);
            let vertical = vec3::new(0.0, viewport_height.clone(), 0.0);
            Self {
                v_fov: v_fov,
                aspect_ratio: aspect_ratio,
                viewport_height: viewport_height.clone(),
                viewport_width: viewport_width.clone(),
                focal_length: focal_length,
                origin: origin,
                //TODO: Let user specify initial rotation of camera, and convert that to horizontal/vertical
                horizontal: horizontal,
                vertical: vertical,
                lower_left_corner: origin - horizontal/2.0 - vertical/2.0 - vec3::new(0.0, 0.0, focal_length),

                samples_per_px: samples_per_px,
            }
        }

        pub fn get_ray(self, u: f64, v: f64) -> Ray {
            Ray::new(self.origin, self.lower_left_corner + self.horizontal*u + self.vertical*v - self.origin)
        }
    }

    impl Default for Camera{
        fn default() -> Self {

            let aspect_ratio = 16.0/9.0 as f64;
            let vp_h = 2.0*(deg_to_rad(90.0)/2.0).tan();
            let vp_w = aspect_ratio.clone()*vp_h.clone();

            let h = vec3::new(vp_w.clone(), 0.0, 0.0);
            let v = vec3::new(0.0, vp_h.clone(), 0.0);

            let focal_length = 1.0;

            Camera{
                v_fov: 90.0, // vFOV for a 50mm lens = 27.0 deg
                aspect_ratio: aspect_ratio.clone(),
                viewport_height: vp_h,
                viewport_width: vp_w,
                focal_length: focal_length.clone(),

                samples_per_px: 100,

                origin: vec3::new(0.0,0.0,0.0),
                horizontal: h.clone(),
                vertical: v.clone(),
                lower_left_corner: vec3::new(0.0,0.0,0.0) - h/2.0 - v/2.0 - vec3::new(0.0, 0.0, focal_length),
            }
        }
    }
}