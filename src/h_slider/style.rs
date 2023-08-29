use iced::Font;
use iced_core::Color;

#[derive(Debug, Clone, Copy)]
pub struct Appearance {
    pub background_color: Color,
    pub rail_color: Color,
    pub handle_color: Color,
    pub mark_color_normal: Color,
    pub mark_color_bold: Color,
    pub mark_width: f32,
    pub text_mark_color: Color,
    pub text_mark_font: Font,
    pub text_mark_size: f32,
}

pub trait StyleSheet {
    type Style: Default + Copy;

    fn active(&self, style: Self::Style) -> Appearance;

    fn hovered(&self, style: Self::Style) -> Appearance;
}