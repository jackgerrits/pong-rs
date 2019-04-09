
pub struct Vector(pub f32, pub f32);

impl Vector {
    pub fn normalize(&mut self) {
        let total = self.0 + self.1;

        self.0 = self.0 / total;
        self.1 = self.1 / total;
    }

    pub fn new_normalized(a: f32, b: f32) -> Vector {
        let mut v = Vector(a, b);
        v.normalize();
        v
    }

    pub fn reflect(&mut self, normal: &mut Vector) {
        normal.normalize();
        let dp = Vector::dot(&self, &normal);
        let rhs = Vector(2.0 * dp * normal.0, 2.0 * dp * normal.1);
        self.0 = self.0 - rhs.0;
        self.1 = self.1 - rhs.1;
    }

    pub fn dot(v1: &Vector, v2: &Vector) -> f32 {
        v1.0 * v2.0 + v1.1 * v2.1
    }
}
