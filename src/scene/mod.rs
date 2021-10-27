pub use self::scene::Scene;


mod scene{
    use crate::vector::vec3;
    use crate::geometry::*;
    use crate::geometry::HittableList;
    use std::fs;

    use vec3 as colour;
    use vec3 as point3;


    pub struct Scene {}
    impl Scene{
        pub fn get_scene() -> HittableList{
            let mut scene = HittableList::new();


            // Yellow fuzzy metal sphere
            let m1 = Box::new(Metal{albedo: colour::new(0.8, 0.6, 0.2), fuzz: 0.25});
            let radius = 0.5;
            scene.add(Box::new(Sphere::new(point3::new(0.80, radius, -1.0), radius, m1)));

            // Red diffuse sphere
            let m2: Box<dyn Material> = Box::new( Lambertian{albedo: colour::new(0.7, 0.3, 0.3)});
            let radius = 0.25;
            scene.add(Box::new(Sphere::new(point3::new(-0.10, radius, -0.10), radius, m2)));

            // Shiny metal sphere
            let m3: Box<dyn Material> = Box::new( Metal{albedo: colour::new(0.8, 0.8, 0.8), fuzz: 0.0});
            let radius = 0.5;
            scene.add(Box::new(Sphere::new(point3::new(-0.80, radius, -1.0), radius, m3)));

            // Solid glass sphere
            let m4: Box<dyn Material> = Box::new( Dielectric{albedo: colour::new(1.0,1.0,1.0), index_of_refraction: 1.5});
            let radius = 0.1;
            scene.add(Box::new(Sphere::new(point3::new(0.25, 0.75, -0.5), radius, m4)));

            // Hollow glass sphere
            let m5: Box<dyn Material> = Box::new( Dielectric{albedo: colour::new(0.95,0.95,1.0), index_of_refraction: 1.5});
            scene.add(Box::new(Sphere::new(point3::new(-0.25, 0.75, -0.42), 0.14, m5)));
            let m5: Box<dyn Material> = Box::new( Dielectric{albedo: colour::new(0.95,0.95,1.0), index_of_refraction: 1.5});
            scene.add(Box::new(Sphere::new(point3::new(-0.25, 0.75, -0.42), -0.13, m5)));

            // Cube!
            let m6: Box<dyn Material> = Box::new( Lambertian{albedo: colour::new(0.7, 0.3, 0.7)});
            let w = 0.50; let h = 0.50; let d = 0.50;
            scene.add(Box::new(Cube::new(point3::new(0.0, 0.5, -1.0), w,h,d, m6)));

            // Plane
            let m6: Box<dyn Material> = Box::new( Lambertian{albedo: colour::new(0.3, 0.3, 0.3)});
            let single_sided = true;
            scene.add(Box::new(Plane::new(point3::new(0.0,1.0,0.0), point3::new(0.0,0.0,0.0), m6, single_sided)));

            let offset = vec3::new(1.25,0.5,0.1);
            let scale = 0.25;
            Scene::from_obj(String::from("assets/diamond.obj"), &mut scene, offset, scale);

            scene
        }

        pub fn vec3_from_txt(file_content: &String, symbol: char) -> Vec<vec3> {
            let vecs: Vec<vec3> = file_content.split("\n")
                .filter(|line| line.chars().nth(0) != None && line.chars().nth(0).unwrap() == symbol)
                .map(|line| line.split_whitespace()
                    .filter(|v| v.chars().nth(0).unwrap() != symbol)
                    .map(|v: &str| v.parse::<f64>().unwrap()).collect() )
                .map(|v| vec3::new_from_Vec(v))
                .collect();
        
            vecs
        }
        pub fn face_from_txt(file_content: &String, symbol: char) -> Vec<Vec<usize>> {
            let faces: Vec<Vec<usize>> = file_content.split("\n")
                .filter(|line| line.chars().nth(0) != None && line.chars().nth(0).unwrap() == symbol)
                .map(|line| line.split_whitespace()
                    .filter(|v| v.chars().nth(0).unwrap() != symbol)
                    .map(|v: &str| v.parse::<usize>().unwrap()).collect() )
                .collect();
        
            faces
        }
        pub fn add_Triangles(vertices: Vec<vec3>, faces: Vec<Vec<usize>>, scene: &mut HittableList, offset: vec3, scale: f64) {
            let single_sided = true;
            for f in faces{
                let material: Box<dyn Material> = Box::new( Lambertian{albedo: colour::new(0.1, 0.7, 0.1)});

                let v1 = vertices[f[0]-1]*scale + offset;
                let v2 = vertices[f[1]-1]*scale + offset;
                let v3 = vertices[f[2]-1]*scale + offset;
                scene.add(Box::new(Triangle::new(v1, v2, v3, material, single_sided)));
            }
        }
        pub fn from_obj(path: String, scene: &mut HittableList, offset: vec3, scale: f64) {

            let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
            
            let path = "/home/ojferro/Projects/raytracer/assets/monkey.obj";
            let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
        
            let vertices: Vec<vec3> = Scene::vec3_from_txt(&contents, 'v');
            let faces: Vec<Vec<usize>> = Scene::face_from_txt(&contents, 'f');

            Scene::add_Triangles(vertices, faces, scene, offset, scale);
        }
    }
}