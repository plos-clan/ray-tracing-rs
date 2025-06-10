use nalgebra::Vector3;
use ray_tracing::camera::{Camera, CameraParams};
use ray_tracing::hitable::{HitableList, Sphere};
use ray_tracing::material::Material;

fn main() {
    let mut world = HitableList::default();

    world.push(Sphere::new(
        Vector3::new(0.0, -100000.0, -1.0),
        100000.0,
        Material::Lambertian {
            albedo: Vector3::new(1.0, 0.4, 0.4),
        },
    ));

    let main_spheres = [
        (
            Vector3::new(0.0, 1.0, 0.0),
            1.0,
            Material::Dielectric {
                refraction_index: 1.5,
            },
        ),
        (
            Vector3::new(0.0, 1.0, 0.0),
            0.95,
            Material::Dielectric {
                refraction_index: 1.0 / 1.5,
            },
        ),
        (
            Vector3::new(-4.0, 1.0, 0.0),
            1.0,
            Material::Lambertian {
                albedo: Vector3::new(0.1, 0.2, 0.5),
            },
        ),
        (
            Vector3::new(4.0, 1.0, 0.0),
            1.0,
            Material::Metal {
                albedo: Vector3::new(0.8, 0.6, 0.2),
                fuzz: 0.1,
            },
        ),
    ];

    for (center, radius, material) in main_spheres {
        world.push(Sphere::new(center, radius, material));
    }

    let random_color = |min: f64, max: f64| {
        let range = max - min;
        Vector3::new(
            min + range * rand::random::<f64>(),
            min + range * rand::random::<f64>(),
            min + range * rand::random::<f64>(),
        )
    };

    for a in -12..6 {
        for b in -10..6 {
            let choose_mat = rand::random::<f64>();
            let random_size = 0.17 + 0.08 * rand::random::<f64>();

            let center = Vector3::new(
                1.8 * a as f64 + 1.5 * rand::random::<f64>(),
                random_size,
                1.8 * b as f64 + 1.5 * rand::random::<f64>(),
            );

            if (center - Vector3::new(4.0, 0.2, 0.0)).norm() <= 0.9 {
                continue;
            }

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

    let params = CameraParams {
        aspect_ratio: 16.0 / 9.0,
        image_width: 1024,
        samples_per_pixel: 200,
        max_depth: 30,
        vertical_fov: 25.0,
        look_from: Vector3::new(13.0, 5.0, 4.0),
        look_at: Vector3::new(0.0, 0.0, 0.0),
        view_up: Vector3::new(0.0, 1.0, 0.0),
        defocus_angle: 0.6,
        focus_distance: 14.5,
    };

    Camera::new(params).render(&world);
}
