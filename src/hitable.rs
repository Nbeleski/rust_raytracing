use crate::vec::{Vec3, Ray};

#[derive(Clone, Copy)]
pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3
}

// impl Default for HitRecord {
//     fn default() -> HitRecord {
//         HitRecord{t: 0.0, p: Vec3(0.0, 0.0, 0.0), normal: Vec3(0.0, 0.0, 0.0)}
//     }
// }

pub trait Hitable {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32
}

impl Hitable for Sphere {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a: f32 = r.direction().dot(r.direction());
        let b: f32 = 2.0 * oc.dot(r.direction());
        let c: f32 = oc.dot(oc) - self.radius*self.radius;
        let discriminant: f32 = b*b - 4.0*a*c;
        if discriminant > 0.0 {
            let mut temp: f32 = (-b - (b*b - a*c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                let pemp = r.point_at_parameter(temp);
                return Some(HitRecord {
                    t: temp,
                    p: pemp,
                    normal: (pemp - self.center) / self.radius
                })
            }

            temp = (-b + (b*b - a*c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                let pemp = r.point_at_parameter(temp);
                return Some(HitRecord {
                    t: temp,
                    p: pemp,
                    normal: (pemp - self.center) / self.radius
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