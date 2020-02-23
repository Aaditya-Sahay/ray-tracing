use std::ops::{Add, Sub, Mul, Div};
use std::cmp::PartialEq;
use std::f64;

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub dimensions: [f64; 3]
}

impl Vec3 {
    pub fn new(d1:f64, d2:f64, d3:f64) -> Self {
        Vec3{
            dimensions: [d1, d2, d3],
        }
    }
    pub fn x(&self) -> f64 {
        self.dimensions[0]
    }
    pub fn y(&self) -> f64 {
        self.dimensions[1]
    }
    pub fn z(&self) -> f64 {
        self.dimensions[2]
    }
    // if its a color we do this 
    // if its a bs we do that 
    pub fn r(&self) -> f64 {
        self.dimensions[0]
    }
    pub fn g(&self) -> f64 {
        self.dimensions[1]
    }
    pub fn b(&self) -> f64 {
        self.dimensions[2]
    }

    pub fn print(&self) -> () {
        println!("{:?}", self);
    }
    pub fn length(&self) -> f64 {
        (self.x() * self.x() + self.y() * self.y() + self.z() * self.z()).sqrt()
    }

    pub fn unit_vector(&self) -> Vec3 {
        Vec3{dimensions: [self.x() / self.length(),
            self.y() / self.length(),
            self.z() / self.length()]}
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        (self.x() * other.x()) + (self.y() * other.y()) + (self.z() * other.z())
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {dimensions: [self.y() * other.z() - self.z() * other.y(),
            self.z() * other.x() - self.x() * other.z(),
            self.x() * other.y() - self.y() * other.x()]}
    }

    /// Converts floats to ints, colors are
    /// supposed to be integer values between 0 and 255.
    pub fn colorize(&mut self) -> () {
        self.dimensions[0] = self.dimensions[0].floor();
        self.dimensions[1] = self.dimensions[1].floor();
        self.dimensions[2] = self.dimensions[2].floor();
    }

}


impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        //creates a new vector from addition
        Vec3 {
            dimensions: [self.x() + other.x(), self.y() + other.y(), self.z() + other.z()]
        }
    }
}
// substraction of vector 

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {dimensions: [self.x() - other.x(),
            self.y() - other.y(),
            self.z() - other.z()]}
    }
}

//multiplication of vector with vector 
impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {dimensions: [self.x() * other.x(),
            self.y() * other.y(),
            self.z() * other.z()]}
    }
}
//multiplcatoin of vector with a float 

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f64) -> Vec3 {
        Vec3 {dimensions: [self.x() * other,
            self.y() * other,
            self.z() * other]}
    }
}
//multiplication of a float with vector 


impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {dimensions: [self * other.x(), self * other.y(), self * other.z()]}
    }
}

// division by a number

impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, other: f64) -> Vec3 {
        if other == 0.0 {
            return Vec3 {dimensions: [f64::MAX, f64::MAX, f64::MAX]};
        }
        Vec3 {dimensions: [self.x() / other, self.y() / other, self.z() / other]}
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Vec3) -> bool {
        self.x() == other.x() && self.y() == other.y() && self.z() == other.z()
    }
}