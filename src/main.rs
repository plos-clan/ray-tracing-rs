use nalgebra::Vector3;
use ray_tracing::camera::{Camera, CameraParams};
use ray_tracing::hitable::{HitableList, Sphere};
use ray_tracing::material::Material;

fn main() {
    let mut world = HitableList::default();

    let material_ground = Material::Lambertian {
        albedo: Vector3::new(1.0, 0.4, 0.4),
    };
    world.push(Sphere::new(
        Vector3::new(0.0, -100000.0, -1.0),
        100000.0,
        material_ground,
    ));

    let material1 = Material::Dielectric {
        refraction_index: 1.5,
    };
    world.push(Sphere::new(Vector3::new(0.0, 1.0, 0.0), 1.0, material1));
    let material1_1 = Material::Dielectric {
        refraction_index: 1.0 / 1.5,
    };
    world.push(Sphere::new(Vector3::new(0.0, 1.0, 0.0), 0.95, material1_1));
    let material2 = Material::Lambertian {
        albedo: Vector3::new(0.1, 0.2, 0.5),
    };
    world.push(Sphere::new(Vector3::new(-4.0, 1.0, 0.0), 1.0, material2));
    let material3 = Material::Metal {
        albedo: Vector3::new(0.8, 0.6, 0.2),
        fuzz: 0.1,
    };
    world.push(Sphere::new(Vector3::new(4.0, 1.0, 0.0), 1.0, material3));

    for a in -12..6 {
        for b in -10..6 {
            let choose_mat = rand::random::<f64>();
            let random_size = 0.17 + 0.08 * rand::random::<f64>();

            let center = Vector3::new(
                1.8 * a as f64 + 1.5 * rand::random::<f64>(),
                random_size,
                1.8 * b as f64 + 1.5 * rand::random::<f64>(),
            );

            let random_color = |min: f64, max: f64| {
                Vector3::new(
                    min + (max - min) * rand::random::<f64>(),
                    min + (max - min) * rand::random::<f64>(),
                    min + (max - min) * rand::random::<f64>(),
                )
            };

            if (center - Vector3::new(4.0, 0.2, 0.0)).norm() > 0.9 {
                let sphere_material = if choose_mat < 0.8 {
                    Material::Lambertian {
                        albedo: random_color(0.3, 1.0),
                    }
                } else if choose_mat < 0.95 {
                    Material::Metal {
                        albedo: random_color(0.3, 1.0),
                        fuzz: rand::random::<f64>() * 0.5,
                    }
                } else {
                    Material::Dielectric {
                        refraction_index: 1.5,
                    }
                };

                world.push(Sphere::new(center, random_size, sphere_material));
            }
        }
    }

    let params = CameraParams {
        aspect_ratio: 16.0 / 9.0,
        image_width: 1920,
        samples_per_pixel: 500,
        max_depth: 100,
        vertical_fov: 25.0,
        look_from: Vector3::new(13.0, 5.0, 4.0),
        look_at: Vector3::new(0.0, 0.0, 0.0),
        view_up: Vector3::new(0.0, 1.0, 0.0),
        defocus_angle: 0.6,
        focus_distance: 14.5,
    };

    Camera::new(params).render(&world);
}
