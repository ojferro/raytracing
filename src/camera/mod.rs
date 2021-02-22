pub use self::camera::Camera;

mod camera{
    use crate::vector::vec3;
    use crate::ray::Ray;
    use vec3 as point3;

    #[derive(Copy, Clone)]
    pub struct Camera {
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

    impl Camera {
        pub fn new(aspect_ratio: f64, viewport_height: f64,focal_length: f64, origin: vec3, samples_per_px: u32) -> Self {
            let viewport_width = aspect_ratio.clone() * viewport_height.clone();
            let horizontal = vec3::new(viewport_width.clone(), 0.0, 0.0);
            let vertical = vec3::new(0.0, viewport_height.clone(), 0.0);
            Self {
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
}