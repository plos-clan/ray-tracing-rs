use nalgebra::Vector3;

#[derive(Copy, Clone)]
pub struct Ray {
    origin: Vector3<f64>,
    direction: Vector3<f64>,
}

impl Ray {
    pub fn new(origin: Vector3<f64>, direction: Vector3<f64>) -> Ray {
        Ray { origin, direction }
    }

    pub fn origin(&self) -> Vector3<f64> {
        self.origin
    }

    pub fn direction(&self) -> Vector3<f64> {
        self.direction
    }

    pub fn point_at_parameter(&self, t: f64) -> Vector3<f64> {
        self.origin + self.direction * t
    }
}

pub struct FactorRange {
    pub max: f64,
    pub min: f64,
}

impl FactorRange {
    pub const fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub fn clamp(&self, value: f64) -> f64 {
        value.max(self.min).min(self.max)
    }

    pub fn surrounds(&self, value: f64) -> bool {
        self.min < value && value < self.max
    }
}
