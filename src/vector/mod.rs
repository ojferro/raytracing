pub use self::vector::vec3;


mod vector{
    use std::ops; // To allow for operator overloading
    use std::fmt; // For printing out structs
    use std::marker::Copy;
    use rand::Rng;
    static spherical_blue_noise_64: &'static [(f64, f64, f64)] = &[(0.06719841, -0.95855117, -0.27688232), (0.9578699, -0.0032531766, -0.2871842), (-0.8169513, 0.25040236, -0.5195082), (0.4437619, 0.6144255, 0.6523469), (-0.20522565, 0.9204008, -0.33278316), (-0.34534046, 0.54488707, 0.7640928), (0.12369734, -0.56490684, 0.81583023), (-0.25005588, -0.76166075, 0.5977833), (0.80913216, 0.41768932, 0.41332895), (-0.4232499, 0.7900235, 0.44353345), (0.7863261, -0.31219956, -0.53312534), (0.4667257, -0.2580736, 0.84591085), (0.06979886, 0.8567019, 0.5110681), (-0.58510876, 0.8097824, -0.043592248), (-0.778557, 0.5645306, 0.2741425), (0.9481215, 0.31560096, 0.038226053), (0.51279813, 0.015121469, -0.8583759), (-0.878782, -0.44322756, -0.17689466), (0.74198866, 0.23762214, -0.6268878), (-0.11276091, -0.47617903, -0.87208855), (0.4190057, -0.719198, 0.5542457), (0.13493076, -0.74644786, -0.6516202), (0.058858782, 0.5146765, 0.8553616), (-0.8842624, -0.36535546, 0.29085237), (0.6012128, 0.78297126, -0.15968505), (-0.61283344, -0.7594801, -0.21823186), (0.0847191, -0.07356083, -0.9936859), (-0.6315406, 0.64514667, -0.43004888), (0.27817443, 0.8790498, -0.38715684), (0.9603933, -0.0602955, 0.2720467), (-0.2927667, -0.344799, 0.89185274), (0.4228658, 0.51119643, -0.74823976), (0.027571838, -0.9268116, 0.37451282), (0.5846646, 0.76913804, 0.2580584), (-0.6920372, -0.10532027, 0.71413696), (-0.6880917, 0.32648668, 0.64802474), (-0.2763285, -0.96059257, 0.030070698), (0.5335626, -0.6788355, -0.50447303), (0.71934, 0.11597821, 0.6849076), (0.61804205, -0.7744751, -0.13495344), (0.23013581, 0.9704936, 0.071968265), (-0.61610675, -0.75332814, 0.23001897), (0.8029452, 0.49623176, -0.33020124), (-0.93354183, 0.1029665, 0.34336263), (0.06210029, -0.09567437, 0.99347365), (0.30318362, -0.9497823, 0.07741359), (0.7226901, -0.6446429, 0.24930875), (0.042306624, 0.7202276, -0.692447), (-0.31802693, 0.13362445, 0.93861777), (0.9220656, -0.3814953, -0.065236494), (-0.59034693, -0.4842521, -0.64574784), (-0.39631927, -0.15794349, -0.904425), (0.39526618, -0.39908296, -0.82734346), (-0.19609348, 0.97428733, 0.110957816), (-0.30288544, -0.79785925, -0.5212301), (-0.4875078, 0.22043341, -0.8448343), (-0.34082714, 0.5958962, -0.7271478), (0.76323843, -0.34081197, 0.5489207), (-0.9173129, 0.38252744, -0.110497974), (-0.5981418, -0.5180011, 0.6114746), (0.016898068, 0.31932566, -0.9474942), (-0.8171272, -0.14419654, -0.5581311), (-0.9961651, -0.054192837, -0.06869483), (0.34877515, 0.22429883, 0.9099705)];
    static spherical_blue_noise_16: &'static [(f64, f64, f64)] = &[(0.2642377, 0.8333146, -0.4855569), (0.95293754, 0.2802258, -0.11568863), (0.41527894, 0.121686935, -0.9015185), (0.3184647, 0.8710724, 0.373916), (-0.75308466, -0.22111271, 0.6196553), (-0.026836863, -0.7566735, -0.6532417), (-0.00034593372, -0.21745783, 0.9760697), (-0.9792196, 0.1657653, -0.11683571), (0.64774984, -0.65245926, 0.3933401), (0.6681226, 0.1880825, 0.7198869), (-0.52438706, 0.81266195, -0.25416327), (-0.7322591, -0.63822496, -0.23762493), (-0.45351753, 0.07909719, -0.8877305), (-0.14146732, -0.93057716, 0.33765846), (-0.4267974, 0.60968375, 0.66792965), (0.73487806, -0.53804797, -0.41286623)];

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
        // pub fn rotate()
        pub fn random_in_unit_sphere(random_seed: usize) -> vec3{
            // let rand_rotation = rand::thread_rng().gen_range(0.0 .. 2.0*3.141592);

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
            vec3::random_in_unit_sphere(rand::thread_rng().gen_range(0..64))*0.2
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