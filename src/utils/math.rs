use nalgebra::Vector3;
use rand::Rng;

pub fn len_squared(v: &Vector3<f64>) -> f64 {
    return v[0] * v[0] + v[1] * v[1] + v[2] * v[2];
}

#[allow(dead_code)]
pub fn reflect(v: &Vector3<f64>, n: &Vector3<f64>) -> Vector3<f64> {
    return v - 2.0 * v.dot(n) * n;
}

#[allow(dead_code)]
pub fn rand_float() -> f64 {
    let mut rng = rand::thread_rng();
    return rng.gen_range(-1.0..1.0);
}

#[allow(dead_code)]
pub fn rand_range(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    return rng.gen_range(min..max);
}

pub fn rand_vec(min: f64, max: f64) -> Vector3<f64> {
    let mut rng = rand::thread_rng();
    Vector3::new(
        rng.gen_range(min..max),
        rng.gen_range(min..max),
        rng.gen_range(min..max),
    )
}

pub fn unit_rand_vec() -> Vector3<f64> {
    loop {
        let p = rand_vec(-1.0, 1.0);
        if len_squared(&p) < 1.0 {
            return p;
        }
    }
}

#[allow(dead_code)]
pub fn rand_in_hemisphere(normal: &Vector3<f64>) -> Vector3<f64> {
    let in_unit_sphere = unit_rand_vec();
    if in_unit_sphere.dot(normal) > 0.0 {
        return in_unit_sphere;
    }
    return -in_unit_sphere;
}
