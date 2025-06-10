use nalgebra::Vector3;

use crate::{
    material::Material,
    ray::{FactorRange, Ray},
};

#[derive(Default)]
pub struct HitRecord {
    pub source_point: Vector3<f64>,
    pub ray_factor: f64,
    pub normal: Vector3<f64>,
    pub material: Material,
    pub front_face: bool,
}

pub trait Hitable: Sync {
    fn hit(&self, ray: &Ray, range: FactorRange) -> Option<HitRecord>;
}

#[derive(Default)]
pub struct HitableList {
    list: Vec<Box<dyn Hitable>>,
}

impl HitableList {
    pub fn push(&mut self, hitable: impl Hitable + 'static) {
        self.list.push(Box::new(hitable))
    }
}

impl Hitable for HitableList {
    fn hit(&self, ray: &Ray, range: FactorRange) -> Option<HitRecord> {
        let mut closest_so_far = range.max;
        let mut record = None;

        for hitable in &self.list {
            let range = FactorRange::new(range.min, closest_so_far);
            if let Some(hit_record) = hitable.hit(ray, range) {
                closest_so_far = hit_record.ray_factor;
                record = Some(hit_record);
            }
        }

        record
    }
}

pub struct Sphere {
    center: Vector3<f64>,
    radius: f64,
    material: Material,
}

impl Sphere {
    pub fn new(center: Vector3<f64>, radius: f64, material: Material) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, range: FactorRange) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().dot(&ray.direction());
        let half_b = oc.dot(&ray.direction());
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;

        if !range.surrounds(root) {
            root = (-half_b + sqrtd) / a;
            if !range.surrounds(root) {
                return None;
            }
        }

        let mut record = HitRecord {
            ray_factor: root,
            source_point: ray.point_at_parameter(root),
            material: self.material.clone(),
            ..Default::default()
        };

        let normal = (record.source_point - self.center) / self.radius;
        let front_face = ray.direction().dot(&normal) <= 0.0;
        let normal = if front_face { normal } else { -normal };

        record.normal = normal;
        record.front_face = front_face;

        Some(record)
    }
}
