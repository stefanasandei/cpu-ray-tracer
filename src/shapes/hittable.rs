use nalgebra::Vector3;

use super::ray::Ray;
use crate::materials::material::Material;
use std::sync::Arc;

pub struct HitRecord {
    pub intersection_point: Vector3<f64>,
    pub normal: Vector3<f64>,
    pub hit_distance: f64,
    pub mat: Option<Arc<dyn Material + Send + Sync>>,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

pub fn empty_record() -> HitRecord {
    return HitRecord {
        intersection_point: Vector3::new(0.0, 0.0, 0.0),
        normal: Vector3::new(0.0, 0.0, 0.0),
        hit_distance: 0.0,
        mat: None,
    };
}
