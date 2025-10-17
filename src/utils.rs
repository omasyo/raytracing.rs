use glam::{Vec3, vec3};

pub fn random_vector() -> Vec3 {
    Vec3 {
        x: rand::random_range(0.0..=1.0),
        y: rand::random_range(0.0..=1.0),
        z: rand::random_range(0.0..=1.0),
    }
}

pub fn random_vector_range(min: f32, max: f32) -> Vec3 {
    Vec3 {
        x: rand::random_range(min..=max),
        y: rand::random_range(min..=max),
        z: rand::random_range(min..=max),
    }
}

pub fn random_unit_vector() -> Vec3 {
    loop {
        let p = random_vector_range(-1.0, 1.0);
        let len_sq = p.length_squared();
        if (f32::EPSILON..=1.0).contains(&len_sq) {
            return p / len_sq.sqrt();
        }
    }
}

pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = Vec3 {
            x: rand::random_range(0.0..=1.0),
            y: rand::random_range(0.0..=1.0),
            z: 0.0,
        };
        let len_sq = p.length_squared();
        if len_sq < 1.0 {
            return p;
        }
    }
}

pub fn near_zero(v: Vec3) -> bool {
    v.x.abs() < f32::EPSILON && v.y.abs() < f32::EPSILON && v.z.abs() < f32::EPSILON
}

// pub fn random_on_hemisphere(normal: Vec3) -> Vec3 {
//     let on_unit_sphere = random_unit_vector();
//     if on_unit_sphere.dot(normal) > 0.0 {
//         on_unit_sphere
//     } else {
//         -on_unit_sphere
//     }
// }
