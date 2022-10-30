use nalgebra::Vector3;

#[allow(unused_imports)]
use crate::{
    shapes::ray::Ray,
    utils::math::{rand_in_hemisphere, unit_rand_vec},
};

use super::material::{Material, MaterialPayload};

#[derive(Clone)]
pub struct DiffuseMaterial {
    pub albedo: Vector3<f64>,
}

impl Material for DiffuseMaterial {
    fn scatter(
        &self,
        _r_in: &Ray,
        intersection_point: &Vector3<f64>,
        normal: &Vector3<f64>,
    ) -> Option<MaterialPayload> {
        // let scatter_dir = normal + unit_rand_vec();
        let scatter_dir = rand_in_hemisphere(normal);

        Some(MaterialPayload {
            scattered: Ray {
                origin: intersection_point.clone(),
                direction: scatter_dir,
            },
            attenuation: self.albedo,
        })
    }
}
