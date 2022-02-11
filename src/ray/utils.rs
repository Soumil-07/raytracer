use std::io::{self, Write};

use super::{Color, Vec3};

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    return *v - (*n * 2.0 * v.dot(n));
}

pub fn refract(uv: &Vec3, n: &Vec3, refraction_ratio: f64) -> Vec3 {
    let cos_theta = f64::min(-uv.dot(n), 1.0);
    let r_out_perp = (*uv + *n * cos_theta) * refraction_ratio;
    let r_out_parallel = *n * -(1.0 - r_out_perp.length_squared()).sqrt();
    r_out_perp + r_out_parallel
}

pub fn write_color(w: &mut impl Write, color: Color, samples: u32) -> io::Result<()> {
    let (mut r, mut g, mut b) = (color.x(), color.y(), color.z());
    let scale = 1.0 / samples as f64;
    r = (scale * r).sqrt();
    g = (scale * g).sqrt();
    b = (scale * b).sqrt();

    write!(
        w,
        "{} {} {}\n",
        (256.0 * r.clamp(0.0, 0.999)) as u32,
        (256.0 * g.clamp(0.0, 0.999)) as u32,
        (256.0 * b.clamp(0.0, 0.999)) as u32,
    )
}
