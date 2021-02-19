// use std::ops; // To allow for operator overloading
// use std::fmt; // For printing out structs
// use std::marker::Copy;


// use vector::vec3;

// use vec3 as colour;
// use vec3 as point3;

// pub use self::ray::Ray;


pub use self::ray::Ray;

mod ray{
    use crate::vector::vec3;

    use vec3 as colour;
    use vec3 as point3;

    #[derive(Copy, Clone)]
    pub struct Ray {
        pub origin: point3,
        pub dir: vec3,
    }

    impl Ray {
        pub fn new(origin: point3, dir: vec3) -> Self {
            Self {origin: origin, dir: dir}
        }

        pub fn at(self, t: f64) -> point3{
            self.origin + self.dir*t
        }
    }
}