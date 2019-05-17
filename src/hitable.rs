extern crate rand;
use crate::vec::{Vec3, Ray};
use crate::material::{Scatter, Material};


use rand::Rng;

#[derive(Clone, Copy)]
pub struct HitRecord<'obj> {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: &'obj Material,
}

// impl Default for HitRecord {
//     fn default() -> HitRecord {
//         HitRecord{t: 0.0, p: Vec3(0.0, 0.0, 0.0), normal: Vec3(0.0, 0.0, 0.0)}
//     }
// }

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Box<Material>
}

pub fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    let mut p = 2.0*Vec3(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>()) - Vec3(1.0, 1.0, 1.0);
    while p.squared_length() >= 1.0 {
        p = 2.0*Vec3(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>()) - Vec3(1.0, 1.0, 1.0);
    }
    return p
}

pub trait Hitable {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

impl Hitable for Sphere {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a: f32 = r.direction().dot(r.direction());
        let b: f32 = 2.0 * oc.dot(r.direction());
        let c: f32 = oc.dot(oc) - self.radius*self.radius;
        let discriminant: f32 = b*b - 4.0*a*c;
        if discriminant > 0.0 {
            let mut temp: f32 = (-b - discriminant.sqrt()) / (2.0 * a);
            if temp < t_max && temp > t_min {
                let pemp = r.point_at_parameter(temp);
                return Some(HitRecord {
                    t: temp,
                    p: pemp,
                    normal: (pemp - self.center) / self.radius,
                    material: &*self.material
                })
            }

            temp = (-b + discriminant.sqrt()) / (2.0 * a);
            if temp < t_max && temp > t_min {
                let pemp = r.point_at_parameter(temp);
                return Some(HitRecord {
                    t: temp,
                    p: pemp,
                    normal: (pemp - self.center) / self.radius,
                    material: &*self.material
                })
            }
        }
        // retorna false
        None
    }
}

impl Hitable for Vec<Box<Hitable>> {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut best = None;
        for child in self {
            if let Some(hit) = child.hit(r, t_min, t_max) {
                match best {
                    None => best = Some(hit),
                    Some(prev) => if hit.t < prev.t {
                        best = Some(hit)
                    }
                }
            }
        }
        best
    }
}