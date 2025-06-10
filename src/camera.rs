use nalgebra::Vector3;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{
    hitable::{Hitable, HitableList},
    ray::{FactorRange, Ray},
};

#[derive(Debug)]
pub struct Camera {
    image_width: i32,
    image_height: i32,
    center: Vector3<f64>,
    pixel00_loc: Vector3<f64>,
    pixel_delta_u: Vector3<f64>,
    pixel_delta_v: Vector3<f64>,
    samples_per_pixel: i32,
    pixel_samples_scale: f64,
    max_depth: i32,
    defocus_angle: f64,
    defocus_disk_u: Vector3<f64>,
    defocus_disk_v: Vector3<f64>,
}

pub struct CameraParams {
    pub aspect_ratio: f64,
    pub image_width: i32,
    pub samples_per_pixel: i32,
    pub max_depth: i32,
    pub vertical_fov: f64,
    pub look_from: Vector3<f64>,
    pub look_at: Vector3<f64>,
    pub view_up: Vector3<f64>,
    pub defocus_angle: f64,
    pub focus_distance: f64,
}

impl Camera {
    pub fn new(params: CameraParams) -> Self {
        let CameraParams {
            aspect_ratio,
            image_width,
            samples_per_pixel,
            max_depth,
            vertical_fov,
            look_from,
            look_at,
            view_up,
            defocus_angle,
            focus_distance,
        } = params;

        // Calculate the image height, and ensure that it's at least 1.
        let image_height = (image_width as f64 / aspect_ratio).floor() as i32;
        let image_height = if image_height < 1 { 1 } else { image_height };

        // Determine viewport dimensions.
        let center = look_from;
        let theta = vertical_fov.to_radians();
        let viewport_height = 2.0 * (theta / 2.0).tan() * focus_distance;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let w = (look_from - look_at).normalize();
        let u = view_up.cross(&w).normalize();
        let v = w.cross(&u);

        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left = center - (focus_distance * w) - 0.5 * (viewport_u + viewport_v);
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        // Calculate the scale of the samples.
        let pixel_samples_scale = 1.0 / samples_per_pixel as f64;

        // Calculate the camera defocus disk basis vectors.
        let defocus_radius = focus_distance * (defocus_angle / 2.0).to_radians().tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Self {
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            samples_per_pixel,
            pixel_samples_scale,
            max_depth,
            defocus_angle,
            defocus_disk_u,
            defocus_disk_v,
        }
    }

    pub fn render(&self, world: &HitableList) {
        let render_row = |j: i32| -> Vec<u8> {
            let mut row_buffer = Vec::new();

            for i in 0..self.image_width {
                let pixel_color = (0..self.samples_per_pixel)
                    .map(|_| {
                        let ray = self.get_ray(i as f64, j as f64);
                        Self::ray_color(&ray, self.max_depth, world)
                    })
                    .sum::<Vector3<f64>>()
                    * self.pixel_samples_scale;

                let pixel_color = Vector3::new(
                    pixel_color.x.sqrt().clamp(0.0, 0.999),
                    pixel_color.y.sqrt().clamp(0.0, 0.999),
                    pixel_color.z.sqrt().clamp(0.0, 0.999),
                );

                row_buffer.push((256.0 * pixel_color.x) as u8);
                row_buffer.push((256.0 * pixel_color.y) as u8);
                row_buffer.push((256.0 * pixel_color.z) as u8);
            }

            row_buffer
        };

        let image_buffer = (0..self.image_height)
            .into_par_iter()
            .map(render_row)
            .flatten()
            .collect::<Vec<u8>>();

        image::save_buffer(
            "output.png",
            &image_buffer,
            self.image_width as u32,
            self.image_height as u32,
            image::ColorType::Rgb8,
        )
        .unwrap();
    }

    fn get_ray(&self, i: f64, j: f64) -> Ray {
        let pixel_sample = self.pixel00_loc
            + (i + rand::random::<f64>() - 0.5) * self.pixel_delta_u
            + (j + rand::random::<f64>() - 0.5) * self.pixel_delta_v;

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            let disk_sample = loop {
                let sample_square = Vector3::new(
                    rand::random::<f64>() * 2.0 - 1.0,
                    rand::random::<f64>() * 2.0 - 1.0,
                    0.0,
                );
                if sample_square.norm_squared() < 1.0 {
                    break sample_square;
                }
            };

            self.center
                + (disk_sample.x * self.defocus_disk_u)
                + (disk_sample.y * self.defocus_disk_v)
        };

        Ray::new(ray_origin, pixel_sample - ray_origin)
    }

    fn ray_color(ray: &Ray, depth: i32, world: &HitableList) -> Vector3<f64> {
        if depth <= 0 {
            return Vector3::new(0.0, 0.0, 0.0);
        }

        let range = FactorRange::new(0.001, f64::INFINITY);
        if let Some(record) = world.hit(ray, range) {
            return match record.material.scatter(ray, &record) {
                Some((scattered, attenuation)) => {
                    let ray_color = Self::ray_color(&scattered, depth - 1, world);
                    attenuation.component_mul(&ray_color)
                }
                None => Vector3::new(0.0, 0.0, 0.0),
            };
        }

        let unit_direction = ray.direction().normalize();
        let ratio_y = 0.5 * (unit_direction.y + 1.0);
        (1.0 - ratio_y) * Vector3::new(1.0, 1.0, 1.0) + ratio_y * Vector3::new(0.5, 0.7, 1.0)
    }
}
