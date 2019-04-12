pub struct Vector {
    pub x: f32,
    pub y: f32,
}

impl Vector {
    pub fn normalize(&self) -> Vector {
        let total = self.x + self.y;
        Vector {
            x: self.x / total,
            y: self.y / total,
        }
    }

    pub fn new_normalized(x: f32, y: f32) -> Vector {
        Vector { x, y }.normalize()
    }

    pub fn reflect(&self, normal: &Vector) -> Vector {
        let norm_normal = normal.normalize();
        let dp = Vector::dot(&self, &norm_normal);
        let rhs = Vector {
            x: 2.0 * dp * norm_normal.x,
            y: 2.0 * dp * norm_normal.y,
        };

        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }

    pub fn dot(v1: &Vector, v2: &Vector) -> f32 {
        v1.x * v2.x + v1.y * v2.y
    }
}
