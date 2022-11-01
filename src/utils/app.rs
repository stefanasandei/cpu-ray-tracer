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
    let scene = Scene::from_file("scenes/demo.yaml");

    let app = Application {
        name: name,
        version: version,
        is_running: true,
        scene: scene.clone(),
        frames: 1,
        camera: camera::create(45.0, 0.1, 100.0),
        renderer: renderer::create(scene.size[0], scene.size[1], scene.samples, scene.gamma),
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
