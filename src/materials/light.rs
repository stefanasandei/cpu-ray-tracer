use nalgebra::Vector3;

#[allow(unused_imports)]
use crate::{
    shapes::ray::Ray,
    utils::math::{rand_in_hemisphere, unit_rand_vec},
};

use super::material::{Material, MaterialPayload};

#[derive(Clone)]
pub struct LightMaterial {
    pub emit: Vector3<f64>,
}

impl Material for LightMaterial {
    fn scatter(
        &self,
        _r_in: &Ray,
        _intersection_point: &Vector3<f64>,
        _normal: &Vector3<f64>,
    ) -> Option<MaterialPayload> {
        None
    }

    fn emitted(&self, _p: Vector3<f64>) -> Vector3<f64> {
        self.emit
    }
}
