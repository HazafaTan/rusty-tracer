struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}
type color = Vec3;
type point = Vec3;

fn length(v: Vec3) -> f64 {
    return square_length(v).sqrt();
}

fn square_length(v: Vec3) -> f64 {
    return v.x * v.x + v.y * v.y + v.z * v.z;
}

fn add(v1: Vec3, v2: Vec3) -> Vec3 {
    return Vec3 {
        x: v1.x + v2.x,
        y: v1.y + v2.y,
        z: v1.z + v2.z,
    };
}

fn test() {
    let color: Vec3 = Vec3 {
        x: 1.0,
        y: 2.0,
        z: 4.0,
    };
    println!("{}", color.x);
}
