pub use self::vector::vec3;


mod vector{
    use std::ops; // To allow for operator overloading
    use std::fmt; // For printing out structs
    use std::marker::Copy;
    use rand::Rng;
    use crate::denoising::BlueNoise;

    #[derive(Copy, Clone)]
    pub struct vec3 {
        pub x: f64,
        pub y: f64,
        pub z: f64,
    }

    impl vec3 {
        pub fn new(x: f64, y: f64, z: f64) -> Self {
            Self { x: x, y: y, z: z }
        }

        pub fn length_squared(self) -> f64{
            self.x*self.x + self.y*self.y + self.z*self.z
        }
        pub fn length(self) -> f64{
            self.length_squared().sqrt()
        }
        pub fn is_near_zero(&self) -> bool{
            let eps = 1e-8;
            self.x < 0.0 && self.y < 0.0 && self.z < 0.0
        }

        pub fn dot(v1: &vec3, v2: &vec3) -> f64{
            v1.x*v2.x+v1.y*v2.y+v1.z*v2.z
        }
        
        pub fn cross(v1: &vec3, v2: &vec3) -> vec3{
            vec3::new(v1.y * v2.z - v1.z * v2.y,
                v1.z * v2.x - v1.x * v2.z,
                v1.x * v2.y - v1.y * v2.x)
        }
        
        pub fn unit_vector(v1: vec3) -> vec3{
            v1/v1.length()
        }
        pub fn random() -> vec3{
            return vec3::new(rand::thread_rng().gen(), rand::thread_rng().gen(), rand::thread_rng().gen())
        }
        pub fn random_in_range(min: f64, max: f64) -> vec3{
            return vec3::new(
                    rand::thread_rng().gen_range(min..max),
                    rand::thread_rng().gen_range(min..max),
                    rand::thread_rng().gen_range(min..max)
                )
        }
        pub fn random_in_unit_sphere(random_seed: usize) -> vec3{
            // let rand_rotation = rand::thread_rng().gen_range(0.0 .. 2.0*3.141592);
            let spherical_blue_noise_64 = BlueNoise::get_spherical_64();
            vec3::new(
                spherical_blue_noise_64[random_seed%spherical_blue_noise_64.len()].0,
                spherical_blue_noise_64[random_seed%spherical_blue_noise_64.len()].1,
                spherical_blue_noise_64[random_seed%spherical_blue_noise_64.len()].2)
            // TODO: Very inefficient right now. Fix.
            // loop{
            //     let p = vec3::random_in_range(-1.0, 1.0);
            //     if p.length_squared() < 1.0{
            //         return p;
            //     }
            // }
        }
        pub fn blue_noise_cleanup() -> vec3{
            //TODO: Change rand for a value depending on the screen-space blue noise
            vec3::random_in_unit_sphere(rand::thread_rng().gen_range(0..32))*0.2
        }
        pub fn random_in_unit_disk() -> vec3{
            // TODO: Very inefficient right now. Fix.
            loop{
                let p = vec3::new(
                    rand::thread_rng().gen_range(-1.0..1.0),
                    rand::thread_rng().gen_range(-1.0..1.0),
                    0.0
                );
                if p.length_squared() >= 1.0 {continue;}
                
                return p;
            }
        }
        pub fn random_unit_vector(random_seed: usize)-> vec3{
            vec3::unit_vector(vec3::random_in_unit_sphere(random_seed))
        }

        pub fn reflect(v: vec3, n: vec3) -> vec3{
            v-n*2.0*vec3::dot(&v, &n)
        }
    }

    impl fmt::Debug for vec3 {
        // To debug print the vec3 struct
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("vec3")
            .field("x", &self.x)
            .field("y", &self.y)
            .field("z", &self.z)
            .finish()
        }
    }

    impl ops::Neg for vec3 {
        type Output = Self;
        fn neg(mut self) -> Self {
            self.x = -self.x;
            self.y = -self.y;
            self.z = -self.z;
            self
        }
    }

    impl ops::AddAssign for vec3 {
        fn add_assign(&mut self, v2: Self) {
            self.x = self.x + v2.x;
            self.y = self.y + v2.y;
            self.z = self.z + v2.z;
        }
    }

    impl ops::MulAssign<f64> for vec3 {
        fn mul_assign(&mut self, t: f64) {
            self.x = self.x*t;
            self.y = self.y*t;
            self.z = self.z*t;
        }
    }

    impl ops::DivAssign<f64> for vec3 {
        fn div_assign(&mut self, t: f64) {
            self.x = self.x/t;
            self.y = self.y/t;
            self.z = self.z/t;
        }
    }

    impl ops::Add for vec3 {
        type Output = Self;

        fn add(self, v2: Self) -> Self {
            Self {
                x: self.x+v2.x, y: self.y+v2.y, z: self.z+v2.z
            }
        }
    }

    impl ops::Sub for vec3 {
        type Output = Self;

        fn sub(self, v2: Self) -> Self {
            Self {
                x: self.x-v2.x, y: self.y-v2.y, z: self.z-v2.z,
            }
        }
    }


    impl ops::Mul<f64> for vec3 {
        type Output = Self;

        fn mul(self, t: f64) -> Self {
            Self {
                x: self.x*t, y: self.y*t, z: self.z*t
            }
        }
    }
    impl ops::Mul<Self> for vec3 {
        type Output = Self;

        fn mul(self, v2: Self) -> Self {
            Self {
                x: self.x*v2.x, y: self.y*v2.y, z: self.z*v2.z
            }
        }
    }

    impl ops::Div<f64> for vec3 {
        type Output = Self;

        fn div(self, t: f64) -> Self {
            Self {
                x: self.x/t, y: self.y/t, z: self.z/t
            }
        }
    }

}