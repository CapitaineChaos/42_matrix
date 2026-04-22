// Un vecteur 3D simple pour commencer
// Debug permet d'afficher avec {:?}
#[derive(Debug)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vector { x, y, z }
    }

    pub fn add(&self, other: &Vector) -> Vector {
        Vector::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }

    pub fn scale(&self, scalar: f32) -> Vector {
        Vector::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}
