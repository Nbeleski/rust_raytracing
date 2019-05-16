use std::ops::*;
//use rand::{Rand, Rng, random};

#[derive(Clone, Copy, Debug)]
pub struct Vec3(pub f32, pub f32, pub f32);

impl Vec3 {
    pub fn x(&self) -> f32 { self.0 }
    pub fn y(&self) -> f32 { self.1 }
    pub fn z(&self) -> f32 { self.2 }
    pub fn r(&self) -> f32 { self.0 }
    pub fn g(&self) -> f32 { self.1 }
    pub fn b(&self) -> f32 { self.2 }

    pub fn dot(&self, other: Vec3) -> f32 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    pub fn squared_length(self) -> f32 { self.dot(self) }
    pub fn length(self) -> f32 { self.squared_length().sqrt() }

    pub fn make_unit_vector(&self) -> Vec3{
        *self / self.length()
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

// elementwise mul
impl Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, v: Vec3) -> Vec3 {
        Vec3(self.0*v.0, self.1*v.1, self.2*v.2)
    }
}

// multiple by constant
impl Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, m: f32) -> Vec3 {
        Vec3(self.0*m, self.1*m, self.2*m)
    }
}
impl Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, v: Vec3) -> Vec3 {
        Vec3(self * v.0, self * v.1, self * v.2)
    }
}

// divide by constant
impl Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, d: f32) -> Vec3 {
        (1.0 / d) * self
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Ray(pub Vec3, pub Vec3);

impl Ray {
    pub fn o(&self) -> Vec3 { self.0 }
    pub fn d(&self) -> Vec3 { self.1 }
    pub fn origin(&self) -> Vec3 { self.0 }
    pub fn direction(&self) -> Vec3 { self.1 }

    pub fn point_at_parameter(&self, t: f32) -> Vec3 {
        self.0 + t * self.1
    }

    pub fn p(&self, t: f32) -> Vec3 {
        self.0 + t * self.1
    }
}