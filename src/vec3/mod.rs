use std::ops::{Add, Neg, Sub, Mul, Div};
use std::ops::{AddAssign, SubAssign};
use std::ops::{MulAssign, DivAssign};
use std::borrow::Borrow;

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub e: [f64; 3]
}

impl Vec3 {
    pub fn new(e1: f64, e2: f64, e3: f64) -> Vec3 {
        Vec3 {
            e: [e1, e2, e3]
        }
    }

    pub fn dot(v1: &Vec3, v2: &Vec3) -> f64 {
        v1.e[0] * v2.e[0] + v1.e[1] * v2.e[1] + v1.e[2] * v2.e[2]
    }

    pub fn cross(v1: &Vec3, v2: &Vec3) -> Vec3 {
        Vec3 {
            e: [ v1.e[1]*v2.e[2] - v1.e[2]*v2.e[1],
                 v1.e[2]*v2.e[0] - v1.e[0]*v2.e[2],
                 v1.e[0]*v2.e[1] - v1.e[1]*v2.e[0]
            ]
        }
    }

    pub fn normalize(&mut self) {
        let k = 1.0 / self.length();
        self.e[0] *= k;
        self.e[1] *= k;
        self.e[2] *= k;
    }

    pub fn unit_vector(&self) -> Vec3 {
        let k = 1. / self.length();
        Vec3 {
            e: [ self.e[0] * k,
                 self.e[1] * k,
                 self.e[2] * k
            ]
        }
    }

    pub fn squared_length(&self) -> f64 {
        let e = self.e;
        e[0]*e[0] +e[1]*e[1] + e[2]*e[2]
    }
    
    pub fn length(&self) -> f64 {
        self.squared_length().sqrt()
    }
}

impl<T> Add<T> for Vec3
where
    T: Borrow<Vec3>
{
    type Output = Vec3;

    fn add(self, other: T) -> Vec3 {
        let new_e1 = self.e[0] + other.borrow().e[0];
        let new_e2 = self.e[1] + other.borrow().e[1];
        let new_e3 = self.e[2] + other.borrow().e[2];
        Vec3 {
            e: [ new_e1, new_e2, new_e3]
        }
    }
}

impl<'a, T> Add<T> for &'a Vec3
where
    T: Borrow<Vec3>
{
    type Output = Vec3;

    fn add(self, other: T) -> Vec3 {
        let new_e1 = self.e[0] + other.borrow().e[0];
        let new_e2 = self.e[1] + other.borrow().e[1];
        let new_e3 = self.e[2] + other.borrow().e[2];
        Vec3 {
            e: [ new_e1, new_e2, new_e3]
        }
    }
}

impl<'a, T> Add<T> for &'a mut Vec3
where
    T: Borrow<Vec3>
{
    type Output = Vec3;

    fn add(self, other: T) -> Vec3 {
        let new_e1 = self.e[0] + other.borrow().e[0];
        let new_e2 = self.e[1] + other.borrow().e[1];
        let new_e3 = self.e[2] + other.borrow().e[2];
        Vec3 {
            e: [ new_e1, new_e2, new_e3]
        }
    }
}

impl<T> AddAssign<T> for Vec3
where
    T: Borrow<Vec3>
{
    fn add_assign(&mut self, other: T) {
        *self = Vec3 {
            e: [ self.e[0] + other.borrow().e[0],
                 self.e[1] + other.borrow().e[1],
                 self.e[2] + other.borrow().e[2]
            ]
        }
    }
}

impl<T> Sub<T> for Vec3
where
    T: Borrow<Vec3>
{
    type Output = Vec3;

    fn sub(self, other: T) -> Vec3 {
        let new_e1 = self.e[0] - other.borrow().e[0];
        let new_e2 = self.e[1] - other.borrow().e[1];
        let new_e3 = self.e[2] - other.borrow().e[2];
        Vec3 {
            e: [ new_e1, new_e2, new_e3]
        }
    }
}

impl<'a, T> Sub<T> for &'a mut Vec3
where
    T: Borrow<Vec3>
{
    type Output = Vec3;

    fn sub(self, other: T) -> Vec3 {
        let new_e1 = self.e[0] - other.borrow().e[0];
        let new_e2 = self.e[1] - other.borrow().e[1];
        let new_e3 = self.e[2] - other.borrow().e[2];
        Vec3 {
            e: [ new_e1, new_e2, new_e3]
        }
    }
}

impl<T> SubAssign<T> for Vec3
where
    T: Borrow<Vec3>
{
    fn sub_assign(&mut self, other: T) {
        *self = Vec3 {
            e: [self.e[0] - other.borrow().e[0],
                self.e[1] - other.borrow().e[1],
                self.e[2] - other.borrow().e[2]
            ]
        }
    }
}

impl<T> Mul<T> for Vec3
where
    T: Borrow<Vec3>
{
    type Output = Vec3;

    fn mul(self, other: T) -> Vec3 {
        let new_e1 = self.e[0] * other.borrow().e[0];
        let new_e2 = self.e[1] * other.borrow().e[1];
        let new_e3 = self.e[2] * other.borrow().e[2];
        Vec3 {
            e: [ new_e1, new_e2, new_e3]
        }
    }
}

impl<'a, T> Mul<T> for &'a mut Vec3
where
    T: Borrow<Vec3>
{
    type Output = Vec3;

    fn mul(self, other: T) -> Vec3 {
        let new_e1 = self.e[0] * other.borrow().e[0];
        let new_e2 = self.e[1] * other.borrow().e[1];
        let new_e3 = self.e[2] * other.borrow().e[2];
        Vec3 {
            e: [ new_e1, new_e2, new_e3]
        }
    }
}

impl<T: Borrow<Vec3>> MulAssign<T> for Vec3
{
    fn mul_assign(&mut self, other: T) {
        *self = Vec3 {
            e: [self.e[0] * other.borrow().e[0],
                self.e[1] * other.borrow().e[1],
                self.e[2] * other.borrow().e[2]
            ]
        }
    }
}

impl Mul<f64> for Vec3
{
    type Output = Vec3;

    fn mul(self, other: f64) -> Vec3 {
        let new_e1 = self.e[0] * other;
        let new_e2 = self.e[1] * other;
        let new_e3 = self.e[2] * other;
        Vec3 {
            e: [ new_e1, new_e2, new_e3]
        }
    }
}

impl<'a> Mul<f64> for &'a Vec3
{
    type Output = Vec3;

    fn mul(self, other: f64) -> Vec3 {
        let new_e1 = self.e[0] * other;
        let new_e2 = self.e[1] * other;
        let new_e3 = self.e[2] * other;
        Vec3 {
            e: [ new_e1, new_e2, new_e3]
        }
    }
}

impl<'a> Mul<f64> for &'a mut Vec3
{
    type Output = Vec3;

    fn mul(self, other: f64) -> Vec3 {
        let new_e1 = self.e[0] * other;
        let new_e2 = self.e[1] * other;
        let new_e3 = self.e[2] * other;
        Vec3 {
            e: [ new_e1, new_e2, new_e3]
        }
    }
}

impl MulAssign<f64> for Vec3
{
    fn mul_assign(&mut self, other: f64) {
        *self = Vec3 {
            e: [self.e[0] * other,
                self.e[1] * other,
                self.e[2] * other
            ]
        }
    }
}

impl<'a> MulAssign<&'a f64> for Vec3
{
    fn mul_assign(&mut self, other: &f64) {
        *self = Vec3 {
            e: [self.e[0] * other,
                self.e[1] * other,
                self.e[2] * other
            ]
        }
    }
}

impl<T> Div<T> for Vec3
where
    T: Borrow<Vec3>
{
    type Output = Vec3;

    fn div(self, other: T) -> Vec3 {
        let new_e1 = self.e[0] / other.borrow().e[0];
        let new_e2 = self.e[1] / other.borrow().e[1];
        let new_e3 = self.e[2] / other.borrow().e[2];
        Vec3 {
            e: [ new_e1, new_e2, new_e3]
        }
    }
}

impl<'a, T> Div<T> for &'a mut Vec3
where
    T: Borrow<Vec3>
{
    type Output = Vec3;

    fn div(self, other: T) -> Vec3 {
        let new_e1 = self.e[0] / other.borrow().e[0];
        let new_e2 = self.e[1] / other.borrow().e[1];
        let new_e3 = self.e[2] / other.borrow().e[2];
        Vec3 {
            e: [ new_e1, new_e2, new_e3]
        }
    }
}

impl<T: Borrow<Vec3>> DivAssign<T> for Vec3
{
    fn div_assign(&mut self, other: T) {
        *self = Vec3 {
            e: [self.e[0] / other.borrow().e[0],
                self.e[1] / other.borrow().e[1],
                self.e[2] / other.borrow().e[2]
            ]
        }
    }
}

impl Div<f64> for Vec3
{
    type Output = Vec3;

    fn div(self, other: f64) -> Vec3 {
        let new_e1 = self.e[0] / other;
        let new_e2 = self.e[1] / other;
        let new_e3 = self.e[2] / other;
        Vec3 {
            e: [ new_e1, new_e2, new_e3]
        }
    }
}

impl<'a> Div<f64> for &'a mut Vec3
{
    type Output = Vec3;

    fn div(self, other: f64) -> Vec3 {
        let new_e1 = self.e[0] / other;
        let new_e2 = self.e[1] / other;
        let new_e3 = self.e[2] / other;
        Vec3 {
            e: [ new_e1, new_e2, new_e3]
        }
    }
}

impl DivAssign<f64> for Vec3
{
    fn div_assign(&mut self, other: f64) {
        *self = Vec3 {
            e: [self.e[0] / other,
                self.e[1] / other,
                self.e[2] / other
            ]
        }
    }
}

impl<'a> DivAssign<&'a f64> for Vec3
{
    fn div_assign(&mut self, other: &f64) {
        *self = Vec3 {
            e: [self.e[0] / other,
                self.e[1] / other,
                self.e[2] / other
            ]
        }
    }
}

impl Neg for Vec3
{
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {
            e: [-self.e[0], -self.e[1], -self.e[2]]
        }
    }
}
