use std::sync::Arc;

use nalgebra::Vector3;

use crate::{
    materials::{diffuse::DiffuseMaterial, metal::MetalMaterial},
    shapes::sphere::Sphere,
};

use super::{
    camera::{self, Camera},
    renderer::{self, Renderer},
    scene::Scene,
};

pub struct Application {
    pub name: &'static str,
    pub version: &'static str,
    pub is_running: bool,

    frames: u32,

    scene: Scene,
    camera: Camera,
    renderer: Renderer,
}

pub fn create(name: &'static str, version: &'static str) -> Application {
    let scene = Scene {
        name: "simple",
        spheres: vec![
            Sphere {
                position: Vector3::new(0.0, 0.2, 0.0),
                radius: 0.3,
                mat: Some(Arc::new(DiffuseMaterial {
                    albedo: Vector3::new(1.0, 1.0, 1.0),
                })),
            },
            Sphere {
                position: Vector3::new(-0.8, 0.0, 0.0),
                radius: 0.5,
                mat: Some(Arc::new(MetalMaterial {
                    albedo: Vector3::new(1.0, 0.0, 1.0),
                    fuzz: 0.0,
                })),
            },
            Sphere {
                position: Vector3::new(0.8, 0.0, 0.0),
                radius: 0.5,
                mat: Some(Arc::new(DiffuseMaterial {
                    albedo: Vector3::new(0.3, 0.0, 1.0),
                })),
            },
            Sphere {
                position: Vector3::new(0.0, 100.5, 0.0),
                radius: 100.0,
                mat: Some(Arc::new(DiffuseMaterial {
                    albedo: Vector3::new(0.5, 0.5, 0.5),
                })),
            },
        ],
    };

    let app = Application {
        name: name,
        version: version,
        is_running: true,
        scene: scene,
        frames: 1,
        camera: camera::create(45.0, 0.1, 100.0),
        renderer: renderer::create(),
    };

    return app;
}

impl Application {
    pub fn start(&mut self) {
        println!("Using '{}' scene", self.scene.name);
    }

    pub fn run(&mut self) {
        self.renderer.render(&self.scene, &mut self.camera);

        self.frames = self.frames - 1;
        if self.frames == 0 {
            self.is_running = false;
        }
    }

    pub fn shutdown(&mut self) {
        self.renderer.serialize();
    }
}
