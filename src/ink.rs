use macroquad::prelude::*;
pub struct InkDrop {
    centre: Vec2,
    perimeter_points: Vec<Vec2>,
    color: Color,
}

impl InkDrop {
    const NUM_POINTS: usize = 100;
    pub fn new(centre: Vec2, radius: f64, color: Color) -> Self {
        let mut perimeter_points = Vec::with_capacity(Self::NUM_POINTS);
        let angle_step = std::f64::consts::TAU / (Self::NUM_POINTS as f64);

        for i in 0..Self::NUM_POINTS {
            let angle = i as f64 * angle_step;
            let x = radius * angle.cos();
            let y = radius * angle.sin();
            let pos = Vec2::new(x as f32, y as f32);
            perimeter_points.push(pos + centre);
        }
        Self {
            centre,
            perimeter_points,
            color,
        }
    }

    pub fn draw(&self) {
        crate::draw_shape(&self.perimeter_points, self.centre, self.color);
    }

    pub fn be_marbled_by(&mut self, other: &Self, other_radius: f64) {
        // C + (P-C) * sqrt(1+  (r^2 / |P-C|^2))
        let radius_squared = other_radius * other_radius;
        for (point) in self.perimeter_points.iter_mut() {
            let relative_vec = *point - other.centre;
            let length_sq = relative_vec.length_squared();
            let multiplier = (1.0 + (radius_squared as f32 / length_sq)).sqrt();

            let new_point = other.centre + relative_vec * multiplier;
            *point = new_point;
        }
    }

    pub fn be_tine_lined(&mut self, tine_line: &TineLine) {
        // B, a point on the tine line.
        // z and v are displacement and sharpness scalars
        // P = P + z * u^d * M
        // d = |(P-B) dot N|
        for (point) in self.perimeter_points.iter_mut() {
            let d = (*point - tine_line.point).dot(tine_line.perp()).abs();
            let modifier = tine_line.u().powf(d) * tine_line.displacement * tine_line.direction;
            *point = *point + modifier;
        }
    }
    pub fn scale_points(&mut self) {
        // Naiive idea, add extra points between existing ones if they are >some_distance apart
        // TODO: Implement me
    }
}

pub struct TineLine {
    /// Any point along the line,
    point: Vec2,
    /// Unit vector for direction
    direction: Vec2,
    /// aka z
    displacement: f32,
    /// aka c
    sharpness: f32,
}
impl TineLine {
    pub fn new(at: Vec2, direction: Vec2, displacement: f32, sharpness: f32) -> Self {
        Self {
            point: at,
            direction: direction.normalize(),
            displacement,
            sharpness,
        }
    }
    fn perp(&self) -> Vec2 {
        self.direction.perp()
    }

    fn u(&self) -> f32 {
        (0.5_f32).powf(1.0 / self.sharpness)
    }
}
