#[derive(Copy, Clone)]
pub struct Radian {
    raw: f32
}

#[derive(Copy, Clone)]
pub struct Degree {
    raw: f32
}

impl Degree {
    pub fn from_radian(r: &Radian) -> Degree {
        Degree { raw: r.raw.to_degrees() }
    }
}

impl Radian {
    pub fn from_degree(d: &Degree) -> Radian {
        Radian { raw: d.raw.to_radians() }
    }
}