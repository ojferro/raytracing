pub use self::camera::Camera;

mod camera{
    use crate::vector::vec3;
    use crate::ray::Ray;
    use vec3 as point3;

    #[derive(Copy, Clone, Debug)]
    pub struct Camera {
        pub v_fov: f64,
        pub aspect_ratio: f64,
        pub viewport_height: f64,
        pub viewport_width: f64,
        pub lens_radius: f64,
        pub focus_dist: f64,

        // u, v, w is the orthonormal vector defining camera orientation
        pub u: vec3,
        pub v: vec3,
        pub w: vec3,

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
        pub fn new(v_fov: f64, aspect_ratio: f64, aperture: f64, focus_dist: f64, origin: vec3,
            look_at: point3, v_up: point3, samples_per_px: u32) -> Self {
            
            let theta = deg_to_rad(v_fov);
            let h = (theta/2.0).tan();

            let viewport_height = 2.0*h;
            let viewport_width = aspect_ratio.clone() * viewport_height.clone();

            let v_up = v_up;
            let w = vec3::unit_vector(origin - look_at);
            let u = vec3::unit_vector(vec3::cross(&v_up, &w));
            let v = vec3::cross(&w,&u);

            let horizontal = u*viewport_width*focus_dist;
            let vertical = v*viewport_height*focus_dist;

            Self {
                v_fov: v_fov,
                aspect_ratio: aspect_ratio,
                viewport_height: viewport_height,
                viewport_width: viewport_width,
                lens_radius: aperture/2.0,
                focus_dist: focus_dist,

                w: w,
                u: u,
                v: v,

                origin: origin,
                
                //TODO: Let user specify initial rotation of camera, and convert that to horizontal/vertical
                horizontal: horizontal,
                vertical: vertical,
                lower_left_corner: origin - horizontal/2.0 - vertical/2.0 - w*focus_dist,

                samples_per_px: samples_per_px,
            }
        }

        pub fn get_ray(self, s: f64, t: f64) -> Ray {
            let rd = vec3::random_in_unit_disk()*self.lens_radius;
            let offset = self.u*rd.x + self.v*rd.y;
            Ray::new(self.origin + offset, self.lower_left_corner + self.horizontal*s + self.vertical*t - self.origin-offset)
        }

        pub fn position_camera(&mut self, look_from: point3, look_at: point3, v_up: vec3) {
            let w = vec3::unit_vector(look_from - look_at);
            let u = vec3::unit_vector(vec3::cross(&v_up, &w));
            let v = vec3::cross(&w,&u);

            self.origin = look_from;
            self.horizontal = u*self.viewport_width;
            self.vertical = v*self.viewport_height;
            self.lower_left_corner = self.origin - self.horizontal/2.0 - self.vertical/2.0 - w*self.focus_dist;
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

            let look_from = vec3::new(0.0,0.0,0.0);
            let look_at = vec3::new(0.0,0.0,1.0);
            let v_up = vec3::new(0.0,1.0,0.0);

            let w = vec3::unit_vector(look_from - look_at);
            let u = vec3::unit_vector(vec3::cross(&v_up, &w));
            let v = vec3::cross(&w,&u);

            Camera{
                v_fov: 27.0,//90.0, // vFOV for a 50mm lens = 27.0 deg
                aspect_ratio: aspect_ratio.clone(),
                viewport_height: vp_h,
                viewport_width: vp_w,
                lens_radius: 0.0, // Infinite depth of field
                focus_dist: 1.0, //Parameter is not used when lens_radius = 0

                w: w,
                u: u,
                v: v,

                samples_per_px: 100,

                origin: vec3::new(0.0,0.0,0.0),
                horizontal: h.clone()*u,
                vertical: v.clone()*v,
                lower_left_corner: vec3::new(0.0,0.0,0.0) - h/2.0 - v/2.0 - w,
            }
        }
    }
}