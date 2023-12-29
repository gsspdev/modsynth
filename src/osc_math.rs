use std::clone;
#[derive(Clone)]
pub enum ShapeMath {
    SinMath,
    SquMath,
    SawMath,
    TriMath
}

impl ShapeMath {
    pub fn compute(&self, freq: f32, time: f32) -> f32 {
        const pi: f32 = std::f32::consts::PI;
        let circ_dist = 2.0 * pi * freq * time; // the amount of distance around the circle
        match self {
            ShapeMath::SinMath => circ_dist.sin(),
            ShapeMath::SquMath => circ_dist.sin().signum(),
            ShapeMath::SawMath => 2.0 * (freq * time - freq * time.floor()) - 1.0,
            ShapeMath::TriMath => (2.0 * (freq * time - 0.5)).abs() - 1.0,
        }
    }
}