use std::clone;
use std::string::ToString;
use colorful::Colorful;

#[derive(Clone)]
pub enum ShapeMath {
    Sin,
    Squ,
    Saw,
    Tri
}

impl ShapeMath {
    pub fn compute(&self, freq: f32, time: f32) -> f32 {
        const PI: f32 = std::f32::consts::PI;
        let angle = 2.0 * PI * freq * time; // the amount of distance around the circle
        match self {
            ShapeMath::Sin => angle.sin(),
            ShapeMath::Squ => angle.sin().signum(),
            ShapeMath::Saw => 2.0 * (freq * time - freq * time.floor()) - 1.0,
            ShapeMath::Tri => (2.0 * (freq * time - 0.5)).abs() - 1.0,
        }
    }
}


macro_rules! osc_math {
    ($shape:expr, $freq:expr, $secs:expr) => {
        $shape.compute($freq, $secs)
    };
}

pub fn test_osc_math() {
    let shape = ShapeMath::Sin;
    let amp = osc_math!(shape, 440.0, 5.0);
    let amp_to_string = amp.to_string();
    let amp_colorful = amp_to_string.green();
    println!("{}", amp_colorful)
}