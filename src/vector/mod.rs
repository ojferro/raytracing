pub use self::vector::vec3;

mod vector{
    use std::ops; // To allow for operator overloading
    use std::fmt; // For printing out structs
    use std::marker::Copy;

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