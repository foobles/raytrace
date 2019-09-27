use super::Vec3;

#[derive(Copy, Clone)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Ray {origin, direction}
    }

    pub fn origin(self) -> Vec3 {
        self.origin
    }

    pub fn direction(self) -> Vec3 {
        self.direction
    }

    pub fn position_at(self, t: f64) -> Vec3 {
         self.origin + self.direction * t
    }
}