use rand::Rng;

pub fn random_float() -> f64 {
    // Returns a random real in [0,1).
    rand::random::<f64>()
}

pub fn random_floats(min: f64, max: f64) -> f64 {
    // Returns a random real in [min,max).
    min + (max - min) * random_float()
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
