use std::{fs, sync::Arc};

use nalgebra::Vector3;
use yaml_rust::YamlLoader;

use crate::{
    materials::{diffuse::DiffuseMaterial, material::Material, metal::MetalMaterial},
    shapes::sphere::Sphere,
};

#[derive(Clone)]
pub struct Scene {
    pub name: String,
    pub spheres: Vec<Sphere>,

    pub output: String,
    pub size: Vec<i64>,
    pub samples: f64,
    pub gamma: f64,
}

impl Scene {
    pub fn from_file(path: &str) -> Scene {
        let contents = fs::read_to_string(path).unwrap();
        let docs = YamlLoader::load_from_str(&contents).unwrap();
        let doc = &docs[0];

        let name = doc["output"].as_str().unwrap().to_owned();
        let mut spheres: Vec<Sphere> = vec![];

        for i in 0..doc["world"].as_vec().unwrap().len() {
            if doc["world"][i]
                .as_hash()
                .unwrap()
                .contains_key(&yaml_rust::Yaml::String("sphere".to_string()))
            {
                let position = Vector3::new(
                    doc["world"][i]["position"][0].as_f64().unwrap(),
                    doc["world"][i]["position"][1].as_f64().unwrap(),
                    doc["world"][i]["position"][2].as_f64().unwrap(),
                );
                let radius = doc["world"][i]["radius"].as_f64().unwrap();

                let mut material: Option<Arc<(dyn Material + 'static)>> = None;

                if doc["world"][i]["material"]["type"]
                    == yaml_rust::Yaml::String("diffuse".to_string())
                {
                    material = Some(Arc::new(DiffuseMaterial {
                        albedo: Vector3::new(
                            doc["world"][i]["material"]["albedo"][0].as_f64().unwrap(),
                            doc["world"][i]["material"]["albedo"][1].as_f64().unwrap(),
                            doc["world"][i]["material"]["albedo"][2].as_f64().unwrap(),
                        ),
                    }));
                } else if doc["world"][i]["material"]["type"]
                    == yaml_rust::Yaml::String("metal".to_string())
                {
                    material = Some(Arc::new(MetalMaterial {
                        albedo: Vector3::new(
                            doc["world"][i]["material"]["albedo"][0].as_f64().unwrap(),
                            doc["world"][i]["material"]["albedo"][1].as_f64().unwrap(),
                            doc["world"][i]["material"]["albedo"][2].as_f64().unwrap(),
                        ),
                        fuzz: doc["world"][i]["material"]["fuzz"].as_f64().unwrap(),
                    }));
                }

                spheres.push(Sphere {
                    position: position,
                    radius: radius,
                    mat: material,
                });
            }
        }

        Scene {
            name: name,
            spheres: spheres,

            output: doc["output"].as_str().unwrap().to_string(),
            size: vec![
                doc["size"][0].as_i64().unwrap(),
                doc["size"][1].as_i64().unwrap(),
            ],
            samples: doc["samples"].as_f64().unwrap(),
            gamma: doc["gamma"].as_f64().unwrap(),
        }
    }
}
