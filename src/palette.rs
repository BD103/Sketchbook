#![allow(dead_code)]

// Original palette:
// https://coolors.co/palette/3b3b3b-efefef-4466dd-e03294-f19a3e

use bevy::prelude::*;

pub struct ColorHues(Color, Color, Color, Color, Color);

/// Works around floating arithmetic being disallowed in constant functions by
/// just inlining it, which is allowed.
macro_rules! rgb {
    ($r:expr, $g:expr, $b:expr) => {
        Color::Rgba {
            red: $r / 255.0,
            green: $g / 255.0,
            blue: $b / 255.0,
            alpha: 1.0,
        }
    };
}

const MONO: ColorHues = ColorHues(
    rgb!(59.0, 59.0, 59.0),
    rgb!(101.0, 101.0, 101.0),
    rgb!(157.0, 157.0, 157.0),
    rgb!(199.0, 199.0, 199.0),
    rgb!(239.0, 239.0, 239.0),
);

const BLUE: ColorHues = ColorHues(
    rgb!(13.0, 25.0, 70.0),
    rgb!(26.0, 51.0, 140.0),
    rgb!(68.0, 102.0, 221.0),
    rgb!(123.0, 146.0, 231.0),
    rgb!(215.0, 222.0, 248.0),
);

const PINK: ColorHues = ColorHues(
    rgb!(68.0, 10.0, 43.0),
    rgb!(171.0, 26.0, 108.0),
    rgb!(224.0, 50.0, 148.0),
    rgb!(238.0, 138.0, 195.0),
    rgb!(246.0, 197.0, 225.0),
);

const ORANGE: ColorHues = ColorHues(
    rgb!(100.0, 55.0, 8.0),
    rgb!(200.0, 111.0, 15.0),
    rgb!(241.0, 154.0, 62.0),
    rgb!(245.0, 182.0, 116.0),
    rgb!(250.0, 219.0, 186.0),
);
