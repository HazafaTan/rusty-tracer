// defining a new struct
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}
type color = Vec3;
type point = Vec3;
// implementing the struct
impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn length(v: Vec3) -> f64 {
        return Self::square_length(v).sqrt();
    }

    pub fn square_length(v: Vec3) -> f64 {
        return v.x * v.x + v.y * v.y + v.z * v.z;
    }

    pub fn add(v1: Vec3, v2: Vec3) -> Vec3 {
        return Vec3 {
            x: v1.x + v2.x,
            y: v1.y + v2.y,
            z: v1.z + v2.z,
        };
    }

    pub fn div(v1: Vec3, v2: Vec3) -> Vec3 {
        return Vec3 {
            x: v1.x / v2.x,
            y: v1.y / v2.y,
            z: v1.z / v2.z,
        };
    }

    pub fn times(v1: Vec3, v2: Vec3) -> Vec3 {
        return Vec3 {
            x: v1.x * v2.x,
            y: v1.y * v2.y,
            z: v1.z * v2.z,
        };
    }

    pub fn sub(v1: Vec3, v2: Vec3) -> Vec3 {
        return Vec3 {
            x: (v1.x - v2.x),
            y: (v1.y - v2.y),
            z: (v1.z - v2.z),
        };
    }

    pub fn cross(v1: Vec3, v2: Vec3) -> Vec3 {
        return Vec3 {
            x: v1.y * v2.z - v1.z * v2.y,
            y: v1.z * v2.x - v1.x * v2.z,
            z: v1.x * v2.y - v1.y * v2.x,
        };
    }
    pub fn dot(v1: Vec3, v2: Vec3) -> f64 {
        return v1.x * v2.x + v1.y * v2.y + v1.z * v2.z;
    }

    pub fn test() {
        let color: Vec3 = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 4.0,
        };
        println!("{}", color.x);
    }
}
