use iced::application;
use iced_core::{Color, Size, Font, Background, BorderRadius};
use crate::h_slider::style::{StyleSheet, Appearance};
use crate::color_utils::{darken, lighten};
use iced_widget::{text, checkbox};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Palette {
    pub background: Color,
    pub text: Color,
    pub primary: Color
}

impl Palette {
    pub const LIGHT: Self = Self {
        background: Color::WHITE,
        text: Color::BLACK,
        primary: Color::from_rgb(
            0x5E as f32 / 255.0,
            0x7C as f32 / 255.0,
            0xE2 as f32 / 255.0,
        )
    };

    pub const DARK: Self = Self {
        background: Color::from_rgb(
            0x20 as f32 / 255.0,
            0x22 as f32 / 255.0,
            0x25 as f32 / 255.0,
        ),
        text: Color::from_rgb(0.90, 0.90, 0.90),
        primary: Color::from_rgb(
            0x5E as f32 / 255.0,
            0x7C as f32 / 255.0,
            0xE2 as f32 / 255.0,
        )
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Theme {
    Light,
    Dark,
}

impl Theme {
    pub fn palette(self) -> Palette {
        match self {
            Self::Light => Palette::LIGHT,
            Self::Dark => Palette::DARK,
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::Dark
    }
}

/**
 * h_slider
 */

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HSliderStyleType {
    Classic
}

impl Default for HSliderStyleType {
    fn default() -> Self {
        Self::Classic
    }
}

pub const DEFAULT_TEXT_MARKER_HEIGHT: f32 = 18.0;
pub const DEFAULT_RAIL_HEIGHT: f32 = 8.0;
pub const DEFAULT_HANDLE_SIZE: Size = Size::new(24., 14.);
pub const RAIL_HANDLE_MARGIN: f32 = 3.;

impl StyleSheet for Theme {
    type Style = HSliderStyleType;

    fn active(&self, style: Self::Style) -> Appearance {
        let palette = self.palette();

        let appearance = Appearance {
            background_color: darken(palette.background, 0.3),
            rail_color: darken(palette.background, 0.1),
            handle_color: palette.primary,
            mark_color_normal: palette.background,
            mark_color_bold: lighten(palette.background, 0.2),
            mark_width: 2.0,
            text_mark_color: Color::WHITE,
            text_mark_font: Font::default(),
            text_mark_size: 12.
        };

        match style {
            HSliderStyleType::Classic => appearance,
        }
    }

    fn hovered(&self, style: Self::Style) -> Appearance {
        let palette = self.palette();

        Appearance {
            handle_color: lighten(palette.primary, 0.1),
            ..self.active(style)
        }
    }
}

/**
 * text
 */

#[derive(Clone, Copy)]
pub enum TextStyle {
    Default
}

impl Default for TextStyle {
    fn default() -> Self {
        Self::Default
    }
}

impl text::StyleSheet for Theme {
    type Style = TextStyle;

    fn appearance(&self, style: Self::Style) -> text::Appearance {
        match style {
            TextStyle::Default => Default::default()
        }
    }
}

/**
 * checkbox
 */

#[derive(Clone, Copy)]
pub enum CheckboxStyle {
    Default
}

impl Default for CheckboxStyle {
    fn default() -> Self {
        Self::Default
    }
}

impl checkbox::StyleSheet for Theme {
    type Style = CheckboxStyle;

    fn hovered(&self, _style: &Self::Style, _is_checked: bool) -> checkbox::Appearance {
        let palette = self.palette();

        checkbox::Appearance {
            background: Background::Color(palette.background),
            icon_color: lighten(palette.primary, 0.1),
            border_radius: BorderRadius::from(0.),
            border_width: 2.,
            border_color: lighten(palette.background, 0.2),
            text_color: Some(palette.text),
        }
    }

    fn active(&self, _style: &Self::Style, _is_checked: bool) -> checkbox::Appearance {
        let palette = self.palette();

        checkbox::Appearance {
            background: Background::Color(palette.background),
            icon_color: palette.primary,
            border_radius: BorderRadius::from(0.),
            border_width: 2.,
            border_color: lighten(palette.background, 0.2),
            text_color: Some(darken(palette.text, 0.2)),
        }
    }
}

