// use raytracer::*;
use std::ops; // To allow for operator overloading
use std::fmt; // For printing out structs
use std::marker::Copy;


#[derive(Copy)]
struct vec3 {
    pub v: Vec<f64>,
}

trait vec3_traits{
    fn new(x: f64, y: f64, z: f64) -> Self;

    fn length_squared(self) -> f64;
    fn length(self) -> f64;

    fn x(&self) -> f64;
    fn y(&self) -> f64;
    fn z(&self) -> f64;

}

impl vec3_traits for vec3 {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Self { v: vec![x, y, z] }
    }

    fn length_squared(self) -> f64{
        self.v[0]*self.v[0] + self.v[1]*self.v[1] + self.v[2]*self.v[2]
    }
    fn length(self) -> f64{
        self.length_squared().sqrt()
    }

    fn x(&self) -> f64{ self.v[0] }
    fn y(&self) -> f64{ self.v[1] }
    fn z(&self) -> f64{ self.v[2] }

}

// impl  for vec3 { }

impl fmt::Debug for vec3 {
    // To debug print the vec3 struct
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("vec3")
         .field("x", &self.v[0])
         .field("y", &self.v[1])
         .field("z", &self.v[2])
         .finish()
    }
}

impl ops::Neg for vec3 {
    type Output = Self;
    fn neg(mut self) -> Self {
        self.v[0] = -self.x();
        self.v[1] = -self.y();
        self.v[2] = -self.z();
        self
    }
}

impl ops::AddAssign for vec3 {
    fn add_assign(&mut self, v2: Self) {
        self.v[0] = self.x() + v2.x();
        self.v[1] = self.y() + v2.y();
        self.v[2] = self.z() + v2.z();
    }
}

impl ops::MulAssign<f64> for vec3 {
    fn mul_assign(&mut self, t: f64) {
        self.v[0] = self.x()*t;
        self.v[1] = self.y()*t;
        self.v[2] = self.z()*t;
    }
}

impl ops::DivAssign<f64> for vec3 {
    fn div_assign(&mut self, t: f64) {
        self.v[0] = self.x()/t;
        self.v[1] = self.y()/t;
        self.v[2] = self.z()/t;
    }
}

impl ops::Add for vec3 {
    type Output = Self;

    fn add(self, v2: Self) -> Self {
        Self {
            v: vec![self.x()+v2.x(), self.y()+v2.y(), self.z()+v2.z()]
        }
    }
}

impl ops::Sub for vec3 {
    type Output = Self;

    fn sub(self, v2: Self) -> Self {
        Self {
            v: vec![self.x()-v2.x(), self.y()-v2.y(), self.z()-v2.z()]
        }
    }
}


impl ops::Mul<f64> for vec3 {
    type Output = Self;

    fn mul(self, t: f64) -> Self {
        Self {
            v: vec![self.x()*t, self.y()*t, self.z()*t]
        }
    }
}
impl ops::Mul<Self> for vec3 {
    type Output = Self;

    fn mul(self, v2: Self) -> Self {
        Self {
            v: vec![self.x()*v2.x(), self.y()*v2.y(), self.z()*v2.z()]
        }
    }
}

impl ops::Div<f64> for vec3 {
    type Output = Self;

    fn div(self, t: f64) -> Self {
        Self {
            v: vec![self.x()/t, self.y()/t, self.z()/t]
        }
    }
}

fn dot(v1: vec3, v2: vec3) -> f64{
    v1.x()*v2.x()+v1.y()*v2.y()+v1.z()+v2.y()
}

fn cross(v1: vec3, v2: vec3) -> vec3{
    vec3::new(v1.v[1] * v2.v[2] - v1.v[2] * v2.v[1],
        v1.v[2] * v2.v[0] - v1.v[0] * v2.v[2],
        v1.v[0] * v2.v[1] - v1.v[1] * v2.v[0])
}

fn unit_vector(v1: vec3) -> vec3{
    let norm = v1.length();
    v1/norm
}

////////////////////////// COLOUR UTILITY FUNCTIONS /////////////////////////

fn write_colour(colour: vec3){
    let ir = (255.999*colour.x()) as i32;
    let ig = (255.999*colour.y()) as i32;
    let ib = (255.999*colour.z()) as i32;
    print!("{} {} {}\n", ir, ig, ib);
}


use vec3 as colour;
// use vec3 as point3;

fn main(){
    let image_width = 256;
    let image_height = 256;

    let mut v1 = vec3::new(1.0, 2.0, 3.0);
    eprintln!("The vector is {:?}.", v1);
    eprintln!("X is: {}", v1.x());

    print!("P3\n{} {}\n255\n", image_width, image_height);
    for j in (0 .. image_height).rev(){
        // Debug msg
        eprint!("\rScanlines remaining: {}     ", j);
        for i in 0..image_width{
            let c: colour = colour::new(
                (i as f64)/(image_width-1) as f64,
                (j as f64)/(image_height-1) as f64,
                0.25);
            write_colour(c);
        }
    }
    eprintln!("\nDone!");
}