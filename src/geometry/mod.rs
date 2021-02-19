pub use self::geometry::Sphere;
pub use self::geometry::Hittable;
pub use self::geometry::HitRecord;

mod geometry{
    use crate::vector::vec3;
    use crate::ray::Ray;
    use vec3 as point3;

    // Store information about ray hits
    pub struct HitRecord {
        pub p: point3,
        pub normal: vec3,
        pub t: f64,
    }

    // Parent trait for all hittable geometry
    pub trait Hittable {
        fn hit(self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool;
    }

    // Sphere
    pub struct Sphere{
        pub center: point3,
        pub radius: f64,
    }

    impl Sphere{
        pub fn new(center: point3, radius: f64) -> Self {
            Self {center: center, radius: radius}
        }
    }

    impl Hittable for Sphere{
        fn hit(self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool{
            let oc: vec3 = ray.origin - self.center;

            let a = ray.dir.length_squared();
            let half_b = vec3::dot(oc, ray.dir);
            let c = oc.length_squared() - self.radius*self.radius;

            let discriminant = half_b*half_b-a*c;
            if discriminant<0.0{ return false; }
            
            let d_sqrt = discriminant.sqrt();

            //Find nearest root in acceptable range
            let mut root = -(half_b+d_sqrt)/a;
            if root<t_min || t_max<root {
                root = (-half_b+d_sqrt)/a;
                if root<t_min||t_max<root{
                    return false;
                }
            }

            hit_record.t = root;
            hit_record.p = ray.at(hit_record.t);
            hit_record.normal = (hit_record.p-self.center)/self.radius;

            return true;
            
        }
    }
}