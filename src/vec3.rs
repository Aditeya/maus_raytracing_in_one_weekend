#![allow(dead_code)]

use std::fmt;
use std::ops;
use rand::prelude::*;

pub type Point3 = Vec3;
pub type Color = Vec3;

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    pub fn new() -> Self {
        Self { e: [0.0, 0.0, 0.0] }
    }

    pub fn with_value(e: f64) -> Self {
        Self { e: [e, e, e] }
    }
    
    pub fn with_values(e1: f64, e2: f64, e3: f64) -> Self {
        Self { e: [e1, e2, e3] }
    }

    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            e: [
                rng.gen::<f64>(),
                rng.gen::<f64>(),
                rng.gen::<f64>(),
            ]
        }
    }

    pub fn random_range(min: f64, max: f64) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            e: [
                rng.gen_range(min..=max),
                rng.gen_range(min..=max),
                rng.gen_range(min..=max),
            ]
        }
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        (self.e[0] * self[0]) + (self.e[1] * self[1]) + (self.e[2] * self[2])
    }

    pub fn unit_vector(&self) -> Vec3 {
        *self / self.length()
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        (self.e[0].abs() < s) &&
        (self.e[1].abs() < s) &&
        (self.e[2].abs() < s)
    }
}

pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
    u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
}

pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
    Vec3::with_values(
        u.e[1] * v.e[2] - u.e[2] * v.e[1],
        u.e[2] * v.e[0] - u.e[0] * v.e[2],
        u.e[0] * v.e[1] - u.e[1] * v.e[0],
    )
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    v - 2.0 * dot(v,n) * n
}

pub fn refract(uv: &Vec3, n: &Vec3, eta_i_over_eta_t: f64) -> Vec3 {
    let cos_theta = 1.0f64.min(dot(&(-*uv), n));
    let r_out_perp = eta_i_over_eta_t * (uv + cos_theta * n);
    let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * n;

    r_out_perp + r_out_parallel
}

pub fn random_in_unit_disk() -> Vec3 {
    let mut rng = rand::thread_rng();
    loop {
        let p = Vec3::with_values(
            rng.gen_range(-1.0..=1.0),
            rng.gen_range(-1.0..=1.0),
            0.0
        );
        
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::random_range(-1.0, 1.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

pub fn random_unit_vector() -> Vec3 {
    random_in_unit_sphere().unit_vector()
}

pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
    let in_unit_sphere = random_in_unit_sphere();
    if dot(&in_unit_sphere, normal) > 0.0 {
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}


impl Default for Vec3 {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            e: [-self.e[0], -self.e[1], -self.e[2]],
        }
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}

macro_rules! vec3_vec3_op {
    ($($path:ident)::+, $fn:ident) => {
        impl $($path)::+<Vec3> for Vec3 {
            type Output = Vec3;

            fn $fn(self, other: Vec3) -> Self::Output {
                Vec3 {
                    e: [self.e[0].$fn(other.e[0]),
                        self.e[1].$fn(other.e[1]),
                        self.e[2].$fn(other.e[2]),]
                }
            }
        }

        impl $($path)::+<&Vec3> for &Vec3 {
            type Output = Vec3;

            fn $fn(self, other: &Vec3) -> Self::Output {
                Vec3 {
                    e: [self.e[0].$fn(other.e[0]),
                        self.e[1].$fn(other.e[1]),
                        self.e[2].$fn(other.e[2]),]
                }
            }
        }

        impl $($path)::+<&Vec3> for Vec3 {
            type Output = Vec3;

            fn $fn(self, other: &Vec3) -> Self::Output {
                Vec3 {
                    e: [self.e[0].$fn(other.e[0]),
                        self.e[1].$fn(other.e[1]),
                        self.e[2].$fn(other.e[2]),]
                }
            }
        }

        impl $($path)::+<Vec3> for &Vec3 {
            type Output = Vec3;

            fn $fn(self, other: Vec3) -> Self::Output {
                Vec3 {
                    e: [self.e[0].$fn(other.e[0]),
                        self.e[1].$fn(other.e[1]),
                        self.e[2].$fn(other.e[2]),]
                }
            }
        }
    };
}

/// Generates the operations for vector method assignment. `my_vec += my_other_vec`
/// Handles `Vec3, Vec3` and `Vec3, &Vec3`
/// `vec3_vec3_opassign(ops::AddAssign, add_assign)` (note the camelcase add_assign name)
macro_rules! vec3_vec3_opassign {
    ($($path:ident)::+, $fn:ident) => {
        impl $($path)::+<Vec3> for Vec3 {
            fn $fn(&mut self, other: Vec3) {
                self.e[0].$fn(other.e[0]);
                self.e[1].$fn(other.e[1]);
                self.e[2].$fn(other.e[2]);
            }
        }

        impl $($path)::+<&Vec3> for Vec3 {
            fn $fn(&mut self, other: &Vec3) {
                self.e[0].$fn(other.e[0]);
                self.e[1].$fn(other.e[1]);
                self.e[2].$fn(other.e[2]);
            }
        }
    };
}

/// Generates the operations for method assignment. `my_vec += f64`
/// `vec3_opassign(ops:AddAssign, add_assign)` (note the camelcase add_assign name)
macro_rules! vec3_opassign {
    ($($path:ident)::+, $fn:ident, $ty:ty) => {
        impl $($path)::+<$ty> for Vec3 {
            fn $fn(&mut self, other: $ty) {
                self.e[0].$fn(other);
                self.e[1].$fn(other);
                self.e[2].$fn(other);
            }
        }
    }
}

/// Generates the operations for the method. `let result = my_vec + 4f64`
/// Handles `Vec3, T`, `T, Vec3`, `&Vec3, T`, `T, &Vec3`
/// `vec3_op!(ops:Add, add, f64)`
macro_rules! vec3_op {
    ($($path:ident)::+, $fn:ident, $ty:ty) => {
        // impl ops::Add::add for Vec3
        impl $($path)::+<$ty> for Vec3 {
            type Output = Vec3;

            // fn add(self, other: f64) -> Self::Output
            fn $fn(self, other: $ty) -> Self::Output {
                Vec3 {
                    // e0: self.e0.add(other)
                    e: [self.e[0].$fn(other),
                        self.e[1].$fn(other),
                        self.e[2].$fn(other),]
                }
            }
        }

        impl $($path)::+<$ty> for &Vec3 {
            type Output = Vec3;

            fn $fn(self, other: $ty) -> Self::Output {
                Vec3 {
                    e: [self.e[0].$fn(other),
                        self.e[1].$fn(other),
                        self.e[2].$fn(other),]
                }
            }
        }

        impl $($path)::+<Vec3> for $ty {
            type Output = Vec3;

            fn $fn(self, other: Vec3) -> Self::Output {
                Vec3 {
                    e: [self.$fn(other.e[0]),
                        self.$fn(other.e[1]),
                        self.$fn(other.e[2]),]
                }
            }
        }

        impl $($path)::+<&Vec3> for $ty {
            type Output = Vec3;

            fn $fn(self, other: &Vec3) -> Self::Output {
                Vec3 {
                    e: [self.$fn(other.e[0]),
                        self.$fn(other.e[1]),
                        self.$fn(other.e[2]),]
                }
            }
        }
    }
}

macro_rules! vec3_op_for {
    ($ty: ty) => {
        vec3_op!(ops::Add, add, $ty);
        vec3_op!(ops::Sub, sub, $ty);
        vec3_op!(ops::Mul, mul, $ty);
        vec3_op!(ops::Div, div, $ty);
        vec3_opassign!(ops::AddAssign, add_assign, $ty);
        vec3_opassign!(ops::SubAssign, sub_assign, $ty);
        vec3_opassign!(ops::MulAssign, mul_assign, $ty);
        vec3_opassign!(ops::DivAssign, div_assign, $ty);
    };
}

vec3_vec3_op!(ops::Add, add);
vec3_vec3_op!(ops::Sub, sub);
vec3_vec3_op!(ops::Mul, mul);
vec3_vec3_op!(ops::Div, div);
vec3_vec3_opassign!(ops::AddAssign, add_assign);
vec3_vec3_opassign!(ops::SubAssign, sub_assign);
vec3_vec3_opassign!(ops::MulAssign, mul_assign);
vec3_vec3_opassign!(ops::DivAssign, div_assign);
vec3_op_for!(f64);
