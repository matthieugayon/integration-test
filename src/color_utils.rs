use palette::{Srgb, Hsl, FromColor};
use iced_core::Color;

fn to_hsl(color: Color) -> Hsl {
    Hsl::from_color(Srgb::from(color))
}

fn from_hsl(hsl: Hsl) -> Color {
    Srgb::from_color(hsl).into()
}

pub fn lighten(color: Color, amount: f32) -> Color {
    let mut hsl = to_hsl(color);

    hsl.lightness = if hsl.lightness + amount > 1.0 {
        1.0
    } else {
        hsl.lightness + amount
    };

    from_hsl(hsl)
}

pub fn darken(color: Color, amount: f32) -> Color {
    let mut hsl = to_hsl(color);

    hsl.lightness = if hsl.lightness - amount < 0.0 {
        0.0
    } else {
        hsl.lightness - amount
    };

    from_hsl(hsl)
}