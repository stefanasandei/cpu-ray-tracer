use std::sync::Arc;

use nalgebra::Vector3;

use crate::{materials::material::Material, utils::math::len_squared};

use super::{
    hittable::{HitRecord, Hittable},
    ray::Ray,
};

#[derive(Clone)]
pub struct Sphere {
    pub position: Vector3<f64>,
    pub radius: f64,
    pub mat: Option<Arc<dyn Material + 'static>>,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let origin = ray.origin - self.position;

        let a = len_squared(&ray.direction);
        let half_b = origin.dot(&ray.direction);
        let c = len_squared(&origin) - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let d = discriminant.sqrt();
        let mut root = (-half_b - d) / a;
        if root < t_min || t_max < root {
            root = (-half_b + d) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        let hit_point = origin + ray.direction * root;
        let normal = hit_point.normalize();

        rec.hit_distance = root;
        rec.intersection_point = ray.origin + ray.direction * rec.hit_distance;
        rec.normal = normal;

        true
    }
}
