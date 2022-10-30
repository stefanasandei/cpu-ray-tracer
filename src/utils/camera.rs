use nalgebra::{Matrix4, Perspective3, Point3, Vector2, Vector3, Vector4};

#[derive(Clone)]
pub struct Camera {
    vertical_fov: f64,
    near_clip: f64,
    far_clip: f64,

    viewport_width: u32,
    viewport_height: u32,
    aspect_ratio: f64,

    projection: Perspective3<f64>,
    inverse_projection: Matrix4<f64>,

    view: Matrix4<f64>,
    inverse_view: Matrix4<f64>,

    pub position: Point3<f64>,
    forward_dir: Point3<f64>,

    pub ray_dirs: Vec<Vector3<f64>>,
}

pub fn create(vertical_fov: f64, near_clip: f64, far_clip: f64) -> Camera {
    return Camera {
        vertical_fov: vertical_fov,
        near_clip: near_clip,
        far_clip: far_clip,

        viewport_width: 0,
        viewport_height: 0,
        aspect_ratio: 0.0,

        projection: Perspective3::new(16.0 / 9.0, 0.0, 0.0, 100.0),
        inverse_projection: Matrix4::zeros(),

        view: Matrix4::zeros(),
        inverse_view: Matrix4::zeros(),

        position: Point3::new(0.0, 0.0, 2.0),
        forward_dir: Point3::new(0.0, 0.0, 1.0),

        ray_dirs: vec![],
    };
}

impl Camera {
    pub fn set_size(&mut self, width: u32, height: u32) {
        if width == self.viewport_width && height == self.viewport_height {
            return;
        }

        self.viewport_width = width;
        self.viewport_height = height;
        self.aspect_ratio = (width as f64) / (height as f64);

        self.calculate_projection();
        self.calculate_view();
        self.calculate_ray_directions();
    }

    fn calculate_projection(&mut self) {
        self.projection = Perspective3::new(
            self.aspect_ratio,
            self.vertical_fov,
            self.near_clip,
            self.far_clip,
        );
        self.inverse_projection = self.projection.inverse();
    }

    fn calculate_view(&mut self) {
        self.view = Matrix4::look_at_lh(
            &self.position,
            &Self::add_points(&self.position, &self.forward_dir),
            &Vector3::new(0.0, 1.0, 0.0),
        );
        self.inverse_view = self.view.try_inverse().unwrap();
    }

    pub fn position_vec(&self) -> Vector3<f64> {
        Vector3::new(self.position.x, self.position.y, self.position.z)
    }

    pub fn _get_ray_direction(&self, x: u32, y: u32) -> Vector3<f64> {
        let mut coord = Vector2::new(
            (x as f64) / (self.viewport_width as f64),
            (y as f64) / (self.viewport_height as f64),
        );
        coord = coord * 2.0 - Vector2::new(1.0, 1.0);

        let target = self.inverse_projection * Vector4::new(coord.x, coord.y, 1.0, 1.0);
        let a = Vector3::from(target.xyz() / target.w).normalize();
        let b = self.inverse_view * Vector4::new(a.x, a.y, a.z, 0.0);

        let ray_dir = Vector3::new(b.x, b.y, b.z);

        return ray_dir;
    }

    fn calculate_ray_directions(&mut self) {
        self.ray_dirs.clear();

        for y in 0..self.viewport_height {
            for x in 0..self.viewport_width {
                let coord_x = (x as f64) / (self.viewport_width as f64) * 2.0 - 1.0;
                let coord_y = (y as f64) / (self.viewport_height as f64) * 2.0 - 1.0;

                let target = self.inverse_projection * Vector4::new(coord_x, coord_y, 1.0, 1.0);
                let a = Vector3::from(target.xyz() / target.w).normalize();
                let target = self.inverse_view * Vector4::new(a.x, a.y, a.z, 0.0);

                self.ray_dirs.push(target.xyz());
            }
        }
    }

    fn add_points(p1: &Point3<f64>, p2: &Point3<f64>) -> Point3<f64> {
        return Point3::new(p1.x + p2.x, p1.y + p2.y, p1.z + p2.z);
    }
}
