use crate::vec3::Vec3;

pub struct Ray {
    origin: Vec3,
    direction: Vec3
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Ray {
            origin,
            direction
        }
    }
    pub fn origin(&self) -> Vec3{
        self.origin
    }
    pub fn direction(&self) -> Vec3{
        self.direction
    }
    pub fn point_to_parameter(&self, t: f64) -> Vec3 {
        self.origin + t * self.direction
    }
}


// if origin is equal and directoin is equal then the vectors are equal
impl PartialEq for Ray {
    fn eq(&self, other: &Ray) -> bool {
        self.origin() == other.origin() && self.direction() == other.direction()
    }
}
