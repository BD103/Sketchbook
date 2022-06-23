use bevy_inspector_egui::Inspectable;

const FULL_ROTATION: f32 = 2.0 * std::f32::consts::PI;

#[derive(Inspectable, Debug, Copy, Clone, Eq, PartialEq)]
pub enum Gravity {
    Up,
    Down,
    Left,
    Right,
}

impl Gravity {
    pub fn rotation(&self) -> f32 {
        use Gravity::*;

        match self {
            Down => 0.0,
            Right => FULL_ROTATION * 0.25,
            Up => FULL_ROTATION * 0.5,
            Left => -FULL_ROTATION * 0.75,
        }
    }
}

impl Default for Gravity {
    fn default() -> Self {
        Gravity::Down
    }
}
