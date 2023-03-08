use rand::Rng;
extern crate rand;

pub fn random_float() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0.0..1.0)
}

pub fn random_floats(min: f64, max: f64) -> i32 {
    rand::thread_rng().gen_range(min..max) as i32
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    } else if x > max {
        return max;
    } else {
        return x;
    }
}
