pub use self::geometry::HitRecord;
pub use self::geometry::Hittable;
pub use self::geometry::Sphere;
pub use self::geometry::Cube;
pub use self::geometry::Plane;
pub use self::geometry::HittableList;

pub use self::geometry::Material;
pub use self::geometry::Metal;
pub use self::geometry::Lambertian;
pub use self::geometry::Dielectric;

mod geometry{
    use rand::Rng;
    use crate::vector::vec3;
    use crate::ray::Ray;
    use crate::denoising::BlueNoise;
    use vec3 as point3;
    use vec3 as colour;

    ///////////////////////// Store information about ray hits /////////////////////////
    pub struct HitRecord {
        pub p: point3,
        pub normal: vec3,
        pub t: f32,
        pub front_face: bool,
    }

    impl HitRecord{
        pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &vec3){
            self.front_face = vec3::dot(&ray.dir, &outward_normal) < 0.0;
            if self.front_face {self.normal = *outward_normal;}else{self.normal = -(*outward_normal);}
        }
    }

    impl Default for HitRecord{
        fn default() -> Self {HitRecord{p: point3::new(0.0,0.0,0.0), normal: vec3::new(0.0,0.0,0.0),
                              t: 0.0, front_face: true}}
    }

    ///////////////////////// Parent trait for all hittable geometry /////////////////////////
    pub trait Hittable: Sync + Send {
        fn hit(&self, ray: &Ray, attenuation: &mut colour, t_min: f32, t_max: f32, hit_record: &mut HitRecord, pixel_data: (usize,usize,usize)) -> Option<Ray>;
    }

    /////////////////////////// Sphere /////////////////////////
    pub struct Sphere{
        pub center: point3,
        pub radius: f32,
        pub material: Box<dyn Material>,
    }

    impl Sphere{
        pub fn new(center: point3, radius: f32, material: Box<dyn Material>) -> Self {
            Self {center: center, radius: radius, material: material}
        }
    }

    impl Hittable for Sphere{
        fn hit(&self, ray: &Ray, attenuation: &mut colour, t_min: f32, t_max: f32, hit_record: &mut HitRecord, pixel_data: (usize,usize,usize)) -> Option<Ray>{
            let oc: vec3 = ray.origin - self.center;

            let a = ray.dir.length_squared();
            let half_b = vec3::dot(&oc, &ray.dir);
            let c = oc.length_squared() - self.radius*self.radius;

            let discriminant = half_b*half_b-a*c;
            if discriminant<0.0{ return None; }
            
            let d_sqrt = discriminant.sqrt();

            //Find nearest root in acceptable range
            let mut root = -(half_b+d_sqrt)/a;
            if root<t_min || t_max<root {
                root = (-half_b+d_sqrt)/a;
                if root<t_min||t_max<root{
                    return None;
                }
            }

            hit_record.t = root;
            hit_record.p = ray.at(hit_record.t);
            hit_record.normal = (hit_record.p-self.center)/self.radius;

            let outward_normal = (hit_record.p - self.center)/self.radius;
            hit_record.set_face_normal(ray, &outward_normal);

            // TODO: Optimize unnecessary cloning
            let mut r_out = ray.clone();
            self.material.scatter(ray, &mut r_out, hit_record, attenuation, pixel_data);
            
            Some(r_out)            
        }
    }

    /////////////////////////// Plane /////////////////////////
    pub struct Plane{
        pub normal: vec3,
        pub point: point3,
        pub material: Box<dyn Material>,

        pub single_sided: bool
    }

    impl Plane{
        pub fn new(normal: vec3, point: point3, material: Box<dyn Material>, single_sided: bool) -> Self {
            Self {normal: normal, point: point, material: material, single_sided: single_sided}
        }
    }

    impl Hittable for Plane{
        fn hit(&self, ray: &Ray, attenuation: &mut colour, t_min: f32, t_max: f32, hit_record: &mut HitRecord, pixel_data: (usize,usize,usize)) -> Option<Ray>{
            
            hit_record.t = vec3::dot(&(self.point-ray.origin), &self.normal)/vec3::dot(&self.normal, &ray.dir);

            if hit_record.t < t_min || hit_record.t > t_max {
                return None;
            }

            hit_record.p = ray.at(hit_record.t);
            hit_record.normal = self.normal;

            hit_record.set_face_normal(ray, &hit_record.normal.clone());

            // TODO: Optimize unnecessary cloning
            let mut r_out = ray.clone();
            self.material.scatter(ray, &mut r_out, hit_record, attenuation, pixel_data);

            Some(r_out)            
        }
    }

    /////////////////////////// Cube /////////////////////////
    pub struct Cube{
        pub center: point3,
        pub w: f32,
        pub h: f32,
        pub d: f32,
        pub corner0: vec3,
        pub corner1: vec3,
        // TODO: Allow rotation
        pub material: Box<dyn Material>,
    }

    impl Cube{
        pub fn new(center: point3, w: f32, h: f32, d: f32, material: Box<dyn Material>) -> Self {
            Self {center: center,//(corner1-corner0)/2.0 + corner0,
                corner0: center+vec3::new(-w/2.0, -h/2.0, -d/2.0),
                corner1: center+vec3::new(w/2.0, h/2.0, d/2.0),
                w: w,
                h: h,
                d: d,
                material: material}
        }
    }

    impl Hittable for Cube{
        fn hit(&self, ray: &Ray, attenuation: &mut colour, t_min: f32, t_max: f32, hit_record: &mut HitRecord, pixel_data: (usize,usize,usize)) -> Option<Ray>{
            // Uses Smit's Algorithm
            let mut tmin;
            let mut tmax;
            let tymin;
            let tymax;
            let tzmin;
            let tzmax;

            if ray.dir.x >= 0.0{
                tmin = (self.corner0.x - ray.origin.x)/ray.dir.x;
                tmax = (self.corner1.x - ray.origin.x)/ray.dir.x
            } else {
                tmin = (self.corner1.x - ray.origin.x)/ray.dir.x;
                tmax = (self.corner0.x - ray.origin.x)/ray.dir.x;
            }

            if ray.dir.y >= 0.0{
                tymin = (self.corner0.y - ray.origin.y)/ray.dir.y;
                tymax = (self.corner1.y - ray.origin.y)/ray.dir.y;
            } else {
                tymin = (self.corner1.y - ray.origin.y)/ray.dir.y;
                tymax = (self.corner0.y - ray.origin.y)/ray.dir.y;
            }

            if (tmin > tymax) || (tymin > tmax){
                return None;
            }

            if tymin>tmin {tmin = tymin;}
            if tymax<tmax {tmax = tymax;}

            if ray.dir.z >= 0.0 {
                tzmin = (self.corner0.z - ray.origin.z)/ray.dir.z;
                tzmax = (self.corner1.z - ray.origin.z)/ray.dir.z;
            } else {
                tzmin = (self.corner1.z - ray.origin.z)/ray.dir.z;
                tzmax = (self.corner0.z - ray.origin.z)/ray.dir.z;
            }

            if (tmin > tzmax) || (tzmin > tmax){
                return None;
            }

            if tzmin>tmin {tmin = tzmin;}
            if tzmax<tmax {tmax = tzmax;}

            if tmin < t_max && tmax > t_min{
                // TODO: Improve inefficient cloning
                let mut r_out = ray.clone();
                hit_record.t = tmin;
                hit_record.p = ray.at(hit_record.t);
                let eps = 1.0001;

                // Note: Need integer division (not floor) to deal with negative numbers properly
                hit_record.normal.x = (eps*(hit_record.p-self.center).x/(self.w/2.0)) as i32 as f32;
                hit_record.normal.y = (eps*(hit_record.p-self.center).y/(self.h/2.0)) as i32 as f32;
                hit_record.normal.z = (eps*(hit_record.p-self.center).z/(self.d/2.0)) as i32 as f32;
                hit_record.normal = vec3::unit_vector(hit_record.normal);
                self.material.scatter(ray, &mut r_out, hit_record, attenuation, pixel_data);

                return Some(r_out);
            }

            None            
        }
    }

    ///////////////////////////// Hittable List ///////////////////////////////
    
    pub struct HittableList{
        pub list: Vec<Box<dyn Hittable>>,
    }

    impl HittableList{
        pub fn new() -> Self {
            Self {list: Vec::new()}
        }
        pub fn add(&mut self, hittable: Box<dyn Hittable + Send + Sync>){
            self.list.push(hittable);
        }
        // Hit is not derived from Hittable trait, it's just another method called that
        pub fn hit(&self, ray: &Ray, attenuation: &mut colour, t_min: f32, t_max: f32, hit_record: &mut HitRecord, pixel_data: (usize,usize,usize)) -> Option<Ray>{
            let mut temp_hr = HitRecord{..Default::default()};
            let mut closest_so_far = t_max;
            let mut current_ray = None;

            for object in self.list.iter(){
                if let Some(r) = object.hit(ray, attenuation, t_min, closest_so_far, &mut temp_hr, pixel_data){
                    closest_so_far = temp_hr.t;

                    hit_record.p = temp_hr.p;
                    hit_record.normal = temp_hr.normal;
                    hit_record.t = temp_hr.t;
                    hit_record.front_face = temp_hr.front_face;

                    current_ray = Some(r);
                }
            }
            current_ray
        }
    }
    unsafe impl Send for HittableList {}
    unsafe impl Sync for HittableList {}

    // Material Class
    pub trait Material: Send + Sync{
        fn scatter(&self, r_in: &Ray, r_out: &mut Ray, hit_record: &HitRecord, attenuation: &mut colour, pixel_data: (usize,usize,usize));
    }

    pub struct Metal{
        pub albedo: colour,
        pub fuzz: f32, //Must be <1

    }

    impl Material for Metal{
        fn scatter(&self, r_in: &Ray, r_out: &mut Ray, hit_record: &HitRecord, attenuation: &mut colour, pixel_data: (usize,usize,usize)) {
            let reflected = vec3::reflect(vec3::unit_vector(r_in.dir), hit_record.normal);

            *attenuation = self.albedo;
            *r_out = Ray::new(hit_record.p, reflected + BlueNoise::random_in_unit_sphere(pixel_data)*self.fuzz );//+ BlueNoise::blue_noise_cleanup(pixel_data)*self.fuzz);
        }
    }

    pub struct Lambertian{
        pub albedo: colour
    }

    impl Material for Lambertian{
        fn scatter(&self, _r_in: &Ray, r_out: &mut Ray,hit_record: &HitRecord, attenuation: &mut colour, pixel_data: (usize,usize,usize)){

            let  mut scatter_dir = hit_record.normal + BlueNoise::random_in_unit_sphere(pixel_data);// + BlueNoise::blue_noise_cleanup(pixel_data);

            if scatter_dir.is_near_zero(){
                scatter_dir = hit_record.normal;
            }

            *attenuation = self.albedo;
            *r_out = Ray::new(hit_record.p, scatter_dir);               
        }
    }

    pub struct Dielectric{
        pub albedo: colour,
        pub index_of_refraction: f32,
    }

    impl Dielectric{
        fn refract(&self, uv: vec3, n: vec3, etai_over_etat: f32) -> vec3{
            let cos_theta = vec3::dot(&-uv,&n).min(1.0);
            let r_out_perp = (uv+n*cos_theta)*etai_over_etat;
            let r_out_parallel = -n*(1.0-r_out_perp.length_squared()).abs().sqrt();
            r_out_perp+r_out_parallel
        }
        fn reflectance(&self, cosine: f32, ref_idx: f32) -> f32{
            // Uses Schlick's approx. for reflectance
            let r0 = ((1.0-ref_idx)/(1.0+ref_idx)).powi(2);
            r0 + (1.0-r0)*((1.0-cosine)).powi(5)
        }
        fn should_reflect(&self, cosine: f32, ref_idx: f32) ->bool{
            self.reflectance(cosine, ref_idx) > rand::thread_rng().gen()
        }
    }

    impl Material for Dielectric{
        fn scatter(&self, r_in: &Ray, r_out: &mut Ray,hit_record: &HitRecord, attenuation: &mut colour, pixel_data: (usize,usize,usize)){

            *attenuation = self.albedo;

            let refraction_ratio = if hit_record.front_face {1.0/self.index_of_refraction} else{self.index_of_refraction};
            let unit_dir = vec3::unit_vector((*r_in).dir);

            // Calculate total internal reflection
            let cos_theta = vec3::dot(&-unit_dir, &hit_record.normal).min(1.0);
            let sin_theta = (1.0-cos_theta*cos_theta).sqrt();

            let dir;
            if (refraction_ratio*sin_theta > 1.0) || (self.should_reflect(cos_theta, refraction_ratio)) {
                //Reflect, internally or externally (cannot refract)
                dir = vec3::reflect(unit_dir, hit_record.normal);
            } else {
                // Refract
                dir = self.refract(unit_dir, hit_record.normal, refraction_ratio);
            }

            *r_out = Ray::new(hit_record.p, dir);       
        }
    }

}