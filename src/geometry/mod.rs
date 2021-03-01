pub use self::geometry::HitRecord;
pub use self::geometry::Hittable;
pub use self::geometry::Sphere;
pub use self::geometry::HittableList;

pub use self::geometry::Material;
pub use self::geometry::Metal;
// pub use self::geometry::Diffuse;

mod geometry{
    use crate::vector::vec3;
    use crate::ray::Ray;
    use vec3 as point3;
    use vec3 as colour;

    ///////////////////////// Store information about ray hits /////////////////////////
    pub struct HitRecord {
        pub p: point3,
        pub normal: vec3,
        pub t: f64,
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
    pub trait Hittable {
        fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> Option<Ray>;
    }

    /////////////////////////// Sphere /////////////////////////
    pub struct Sphere{
        pub center: point3,
        pub radius: f64,
        pub material: Box<dyn Material>,
    }

    impl Sphere{
        pub fn new(center: point3, radius: f64, material: Box<dyn Material>) -> Self {
            Self {center: center, radius: radius, material: material}
        }
    }

    impl Hittable for Sphere{
        fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> Option<Ray>{
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

            let target: point3 = hit_record.p + hit_record.normal + vec3::random_unit_vector();
            return Some(Ray::new(ray.at(root), target-hit_record.p).to_owned());
            
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
        pub fn add(&mut self, hittable: Box<dyn Hittable>){
            self.list.push(hittable);
        }
    }

    impl Hittable for HittableList{
        fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> Option<Ray>{
            let mut temp_hr = HitRecord{..Default::default()};
            let mut closest_so_far = t_max;
            let mut current_ray = None;

            for object in self.list.iter(){
                if let Some(r) = object.hit(ray, t_min, closest_so_far, &mut temp_hr){
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

    // Material Class
    pub trait Material{
        fn scatter(&self, r_in: &Ray, hit_record: &HitRecord, attenuation: colour, scattered: &mut Ray) -> &'static bool;
    }

    pub struct Metal{}

    impl Material for Metal{
        fn scatter(&self, r_in: &Ray, hit_record: &HitRecord, attenuation: colour, scattered: &mut Ray) ->  &'static bool{
            &true
        }
    }

    pub struct Lambertian{
        pub albedo: colour
    }

    // impl Material for Lambertian{
    //     fn scatter(&self, r_in: &Ray, hit_record: &HitRecord, attenuation: colour, scattered: &mut Ray) ->  &'static bool{

    //         let scatter_dir = hit_record.normal + vec3::random_unit_vector();
    //         Ray::new(hit_record.p, scatter_dir);
            
    //     }
    // }

}