use std::io::{self, Write};

use super::Color;

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
