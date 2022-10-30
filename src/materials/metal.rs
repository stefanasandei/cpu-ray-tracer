use nalgebra::Vector3;

use crate::{
    shapes::ray::Ray,
    utils::math::{reflect, unit_rand_vec},
};

use super::material::{Material, MaterialPayload};

#[derive(Clone)]
pub struct MetalMaterial {
    pub albedo: Vector3<f64>,
    pub fuzz: f64,
}

impl Material for MetalMaterial {
    fn scatter(
        &self,
        r_in: &Ray,
        intersection_point: &Vector3<f64>,
        normal: &Vector3<f64>,
    ) -> Option<MaterialPayload> {
        let reflected = reflect(&r_in.direction.normalize(), normal);
        let dir = reflected + self.fuzz * unit_rand_vec();

        let payload = Some(MaterialPayload {
            scattered: Ray {
                origin: *intersection_point,
                direction: dir,
            },
            attenuation: self.albedo,
        });

        if dir.dot(normal) > 0.0 {
            return payload;
        }
        None
    }
}
