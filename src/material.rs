use crate::{hitable::HitRecord, ray::Ray};
use nalgebra::Vector3;

#[derive(Clone)]
pub enum Material {
    Lambertian { albedo: Vector3<f64> },
    Metal { albedo: Vector3<f64>, fuzz: f64 },
    Dielectric { refraction_index: f64 },
}

impl Default for Material {
    fn default() -> Self {
        Material::Lambertian {
            albedo: Vector3::new(0.0, 0.0, 0.0),
        }
    }
}

impl Material {
    pub fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Ray, Vector3<f64>)> {
        match self {
            Material::Lambertian { albedo } => {
                let mut direction = loop {
                    let random_in_unit_sphere = Vector3::new(
                        rand::random::<f64>() * 2.0 - 1.0,
                        rand::random::<f64>() * 2.0 - 1.0,
                        rand::random::<f64>() * 2.0 - 1.0,
                    );
                    if random_in_unit_sphere.norm_squared() < 1.0 {
                        break random_in_unit_sphere.normalize();
                    }
                };

                if direction.norm() < 0.0001 {
                    direction = record.normal;
                }

                let reflected = record.normal + direction;
                let scattered = Ray::new(record.source_point, reflected);

                Some((scattered, *albedo))
            }
            Material::Metal { albedo, fuzz } => {
                let reflect = |v: Vector3<f64>, n: Vector3<f64>| v - 2.0 * v.dot(&n) * n;

                let reflected = {
                    let reflected = reflect(ray.direction(), record.normal);

                    let direction = loop {
                        let random_in_unit_sphere = Vector3::new(
                            rand::random::<f64>() * 2.0 - 1.0,
                            rand::random::<f64>() * 2.0 - 1.0,
                            rand::random::<f64>() * 2.0 - 1.0,
                        );
                        if random_in_unit_sphere.norm_squared() < 1.0 {
                            break random_in_unit_sphere.normalize();
                        }
                    };

                    reflected.normalize() + *fuzz * direction
                };

                let scattered = Ray::new(record.source_point, reflected);

                Some((scattered, *albedo))
            }
            Material::Dielectric { refraction_index } => {
                let reflect = |v: Vector3<f64>, n: Vector3<f64>| v - 2.0 * v.dot(&n) * n;

                let refract = |uv: Vector3<f64>, n: Vector3<f64>, etai_over_etat: f64| {
                    let cos_theta = (-uv).dot(&n).min(1.0);
                    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
                    let diff_abs = (1.0 - r_out_perp.norm_squared()).abs();
                    let r_out_parallel = -diff_abs.sqrt() * n;
                    r_out_perp + r_out_parallel
                };

                let reflectance = |cosine: f64, refraction_index: f64| {
                    let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
                    let r0 = r0 * r0;
                    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
                };

                let refraction_index = if record.front_face {
                    1.0 / refraction_index
                } else {
                    *refraction_index
                };

                let unit_direction = ray.direction().normalize();

                let cos_theta = (-unit_direction).dot(&record.normal).min(1.0);
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
                let cannot_refract = (refraction_index * sin_theta) > 1.0;

                let direction = if cannot_refract
                    || reflectance(cos_theta, refraction_index) > rand::random()
                {
                    reflect(unit_direction, record.normal)
                } else {
                    refract(unit_direction, record.normal, refraction_index)
                };

                let scattered = Ray::new(record.source_point, direction);

                Some((scattered, Vector3::new(1.0, 1.0, 1.0)))
            }
        }
    }
}
