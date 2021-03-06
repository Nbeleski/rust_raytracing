extern crate rand;     // generate random numbers
extern crate lodepng; // output PNG image files

mod vec;
mod hitable;
mod camera;
mod material;

use rand::Rng;
use vec::{Vec3, Ray};
use hitable::{HitRecord, Hitable, Sphere, random_in_unit_sphere};
use camera::{Camera};
use material::{Lambertian, Metal};

fn color(r: Ray, world: &Vec<Box<Hitable>>, mut depth: i32) -> Vec3 {
    match(world.hit(r, 0.00001, 10000.0)) {
        Some(rec) => { // cor quando acerta um objeto
            if depth < 50 {
                let target: Vec3 = rec.p + rec.normal + random_in_unit_sphere();
                return 0.5 * color(Ray::new(rec.p, target - rec.p), world, depth+1)
            } else {
                return Vec3(0.0, 0.0, 0.0)
            }
        }
        None      => { // background color (ta correto)
            let unit_direction = r.direction().make_unit_vector();
            let t: f32 = 0.5 * (unit_direction.y() + 1.0);
            return (1.0 - t) * Vec3(1.0, 1.0, 1.0) + t * Vec3(0.5, 0.7, 1.0)
        }
    }
}

fn main() {
    let lower_left_corner: Vec3 = Vec3(-2.0, -1.0, -1.0);
    let horizontal: Vec3 = Vec3(4.0, 0.0, 0.0);
    let vertical: Vec3   = Vec3(0.0, 2.0, 0.0);
    let origin: Vec3     = Vec3(0.0, 0.0, 0.0);

    let nx: i32 = 800;
    let ny: i32 = 400;
    let ns: i32 = 200;

    let mut spheres: Vec<Sphere> = vec![
        Sphere{center: Vec3(0.0, 0.0, -1.0), radius: 0.5, material: Box::new(Lambertian{albedo: Vec3(0.8, 0.3, 0.3)})},
        Sphere{center: Vec3(0.0, -100.5, -1.0), radius: 100.0, material: Box::new(Lambertian{albedo: Vec3(0.8, 0.8, 0.0)})},
        Sphere{center: Vec3(1.0, 0.0, -1.0), radius: 0.5, material: Box::new(Metal{albedo: Vec3(0.8, 0.6, 0.2), fuzz: 0.0})},
        Sphere{center: Vec3(-1.0, 0.0, -1.0), radius: 0.5, material: Box::new(Metal{albedo: Vec3(0.8, 0.8, 0.8), fuzz: 0.0})},
    ];

    // WTF IS THIS? HARDEST PART TO EXPLAIN.
    let world: Vec<Box<Hitable>> = spheres.into_iter().map(|s| Box::new(s) as Box<Hitable>).collect();

    let cam: Camera = Default::default();
    let mut rng = rand::thread_rng();
    println!("P3\n{} {}\n255", nx, ny);
    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3(0.0, 0.0, 0.0);
            for s in 0..ns {
                let u: f32 = ((i as f32) + rng.gen::<f32>()) / nx as f32;
                let v: f32 = ((j as f32) + rng.gen::<f32>()) / ny as f32;
                let r: Ray = cam.get_ray(u, v);
                col = col + color(r, &world, 0);
            }
            
            col = col / ns as f32;
            col = Vec3(col.x().sqrt(), col.y().sqrt(), col.z().sqrt());
            let ir: i32 = (255.99*col.r()) as i32;
            let ig: i32 = (255.99*col.g()) as i32;
            let ib: i32 = (255.99*col.b()) as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }   
}
    // println!("Testing vec: {} {} {}", v3.x(), v3.y(), v3.z());
    // println!("Testing vec: {} {} {}", dt, sq, le);
    // let v1: Vec3 = Vec3(1.0, 2.0, 3.0);
    // let v2: Vec3 = Vec3(5.0, 3.0, 1.0);
    // let r1: Ray = Ray(v1, v2);
    // let v3: Vec3 = r1.p(0.2);
    // println!("Testing vec: {} {} {}", v3.x(), v3.y(), v3.z());

    // cap 4
    // fn color(r: Ray) -> Vec3 {
    //     let t: f32 = hit_sphere(Vec3(0.0, 0.0, -1.0), 0.5, r);
    //     if t > 0.0 {
    //         let n: Vec3 = (r.point_at_parameter(t) - Vec3(0.0, 0.0, -1.0)).make_unit_vector();
    //         return 0.5 * (n + Vec3(1.0, 1.0, 1.0))
    //     }
    //     let unit_vector: Vec3 = r.d().make_unit_vector();
    //     let t: f32 = 0.5 * (unit_vector.y() + 1.0);
    //     return (1.0 - t) * Vec3(1.0, 1.0, 1.0) + t * Vec3(0.5, 0.7, 1.0)
    // }

    // fn hit_sphere(center: Vec3, radius: f32, r: Ray) -> f32 {
    //     let oc = r.origin() - center;
    //     let a: f32 = r.direction().dot(r.direction());
    //     let b: f32 = 2.0 * oc.dot(r.direction());
    //     let c: f32 = oc.dot(oc) - radius*radius;
    //     let discriminant: f32 = b*b - 4.0*a*c;
    //     if discriminant < 0.0 {
    //         return -1.0
    //     }
    //     else {
    //         return (-b - discriminant.sqrt() ) / (2.0 * a)
    //     }
    // }