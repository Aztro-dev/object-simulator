use bevy::prelude::*;
use std::f32::consts::PI;

pub fn quat_to_euler(q: Quat) -> Vec3 {
    let q1 = q.normalize();
    let test = q1.x * q1.y + q1.z * q1.w;
    let mut output: Vec3 = Vec3::default();
    if test > 0.499 {
        // singularity at north pole
        output.x = PI / 2.0;
        output.y = 2.0 * q1.x.atan2(q1.w);
        output.z = 0.0;
        return output;
    }
    if test < -0.499 {
        // singularity at south pole
        output.y = -2.0 * q1.x.atan2(q1.w);
        output.x = -PI / 2.0;
        output.z = 0.0;
        return output;
    }
    let sqx: f32 = q1.x * q1.x;
    let sqy: f32 = q1.y * q1.y;
    let sqz: f32 = q1.z * q1.z;
    output.y = (2.0 * q1.y * q1.w - 2.0 * q1.x * q1.z).atan2(1.0 - 2.0 * sqy - 2.0 * sqz);
    output.x = (2.0 * test).asin();
    output.z = (2.0 * q1.x * q1.w - 2.0 * q1.y * q1.z).atan2(1.0 - 2.0 * sqx - 2.0 * sqz);
    return output;
}
