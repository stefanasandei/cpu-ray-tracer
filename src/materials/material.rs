use nalgebra::Vector3;

use crate::shapes::ray::Ray;

pub struct MaterialPayload {
    pub attenuation: Vector3<f64>,
    pub scattered: Ray,
}

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        intersection_point: &Vector3<f64>,
        normal: &Vector3<f64>,
    ) -> Option<MaterialPayload>;
}
