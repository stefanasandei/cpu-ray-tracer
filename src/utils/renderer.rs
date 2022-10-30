use indicatif::{ProgressBar, ProgressStyle};
use nalgebra::Vector3;

use crate::shapes::{
    hittable::{empty_record, HitRecord, Hittable},
    ray::Ray,
};

use super::{
    camera::Camera,
    framebuffer::{self, Framebuffer},
    scene::Scene,
};

pub struct Renderer {
    fb: Framebuffer,
}

pub fn create() -> Renderer {
    return Renderer {
        fb: framebuffer::create(1280, 720),
    };
}

impl Renderer {
    pub fn render(&mut self, scene: &Scene, camera: &mut Camera) {
        let pb = ProgressBar::new(self.fb.height.into());
        pb.set_style(
            ProgressStyle::with_template(
                "{msg:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len}",
            )
            .unwrap()
            .progress_chars("#>-"),
        );

        camera.set_size(self.fb.width, self.fb.height);

        for j in 0..self.fb.height {
            for i in 0..self.fb.width {
                let mut r = 0.0;
                let mut g = 0.0;
                let mut b = 0.0;
                let samples = 60.0;
                let gamma = 2.0;

                for _ in 0..(samples as u32) {
                    let pixel = self.per_pixel(i, j, scene, camera);
                    r += pixel[0];
                    g += pixel[1];
                    b += pixel[2];
                }

                r = (r / samples).powf(1.0 / gamma);
                g = (g / samples).powf(1.0 / gamma);
                b = (b / samples).powf(1.0 / gamma);

                self.fb.data[(i, j)] = image::Rgb([
                    (r.clamp(0.0, 0.999) * 255.0) as u8,
                    (g.clamp(0.0, 0.999) * 255.0) as u8,
                    (b.clamp(0.0, 0.999) * 255.0) as u8,
                ]);
            }

            pb.inc(1);
            pb.set_message(format!(
                "{}%",
                (j as f64 / self.fb.height as f64 * 100.0) as u32
            ));
        }

        pb.finish_with_message("100%");
    }

    pub fn serialize(&self) {
        self.fb.serialize("simple");
    }

    fn per_pixel(
        &self,
        x: u32,
        y: u32,
        active_scene: &Scene,
        active_camera: &Camera,
    ) -> Vector3<f64> {
        let mut ray = Ray {
            direction: active_camera.ray_dirs[(x + y * self.fb.width) as usize],
            origin: active_camera.position_vec(),
        };

        self.ray_color(&mut ray, active_scene, 50)
    }

    fn trace_ray(&self, ray: &Ray, rec: &mut HitRecord, active_scene: &Scene) -> bool {
        let mut hit_distance = std::f64::MAX;
        let mut hit_anything = false;

        for i in 0..active_scene.spheres.len() {
            if active_scene.spheres[i].hit(&ray, 0.001, hit_distance, rec) {
                hit_anything = true;
                if rec.hit_distance < hit_distance {
                    hit_distance = rec.hit_distance;
                    rec.mat = active_scene.spheres[i].mat.clone();
                }
            }
        }

        return hit_anything;
    }

    fn ray_color(&self, ray: &mut Ray, active_scene: &Scene, depth: u32) -> Vector3<f64> {
        let mut rec = empty_record();

        if depth < 1 {
            return Vector3::new(0.0, 0.0, 0.0);
        }

        if self.trace_ray(&ray, &mut rec, active_scene) {
            let mat_payload = rec
                .mat
                .unwrap()
                .scatter(ray, &rec.intersection_point, &rec.normal);

            if mat_payload.is_some() {
                let mut payload = mat_payload.unwrap();
                let r = self.ray_color(&mut payload.scattered, active_scene, depth - 1);
                let attenuation = payload.attenuation.as_ref();
                return Vector3::new(
                    r.x * attenuation[0],
                    r.y * attenuation[1],
                    r.z * attenuation[2],
                );
            }

            return Vector3::new(0.0, 0.0, 0.0);
        }

        let t = 0.5 * (ray.direction.y + 1.0);
        return (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0);
    }
}
