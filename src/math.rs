use na::Vector3;

pub fn random_vector3_in_unit_sphere() -> Vector3<f32> {
    loop {
        let x = fastrand::f32();
        let y = fastrand::f32();
        let z = fastrand::f32();

        if x * x + y * y + z * z <= 1.0 {
            return Vector3::new(x, y, z);
        }
    }
}

pub fn random_vector3_in_unit_hemisphere() -> Vector3<f32> {
    let mut sph = random_vector3_in_unit_sphere();
    sph.x *= sph.x.signum();
    sph.y *= sph.y.signum();
    sph.z *= sph.z.signum();
    return sph;
}
