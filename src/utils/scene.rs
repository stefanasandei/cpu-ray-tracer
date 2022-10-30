use crate::shapes::sphere::Sphere;

#[derive(Clone)]
pub struct Scene {
    pub name: &'static str,
    pub spheres: Vec<Sphere>,
}
