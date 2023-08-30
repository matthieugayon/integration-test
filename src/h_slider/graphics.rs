
use iced_core::{Size, Rectangle, Vector, Point, alignment::{Horizontal, Vertical}, text::{Shaping, LineHeight}};
use iced_widget::canvas::{
    path::Path, Frame, Fill, Text, Style
};

use super::style::Appearance;
use super::normal::Normal;
use crate::theme::RAIL_HANDLE_MARGIN;
use crate::speed::MarkWeight;

pub fn draw_marks(
    frame: &mut Frame,
    size: Size,
    appearance: Appearance,
    handle_size: Size,
    text_mark_height: f32,
    rail_height: f32,
    markers: Option<&[(Normal, Option<String>, Option<MarkWeight>)]>,
) {
    match markers {
        Some(markers) => {
            let rail_bounds: Rectangle = get_frame_rail_bounds(size, handle_size, text_mark_height, rail_height);
            let marks_bounds = Rectangle {
                x: rail_bounds.x - appearance.mark_width * 0.5,
                y: rail_bounds.y,
                width: rail_bounds.width,
                height: rail_bounds.height
            };

            for (normal, _text, weight) in markers {
                match weight {
                    Some(weight) => {
                        let mark_bounds = Rectangle {
                            x: marks_bounds.x + normal.scale(marks_bounds.width),
                            y: marks_bounds.y,
                            width: appearance.mark_width,
                            height: marks_bounds.height
                        };

                        let mark = Path::rectangle(mark_bounds.position(), mark_bounds.size());

                        let mark_fill_color = match weight {
                            MarkWeight::Normal => appearance.mark_color_normal,
                            MarkWeight::Bold => appearance.mark_color_bold
                        };

                        let mark_fill = Fill {
                            style: Style::Solid(mark_fill_color),
                            ..Fill::default()
                        };

                        frame.fill(&mark, mark_fill);
                    },
                    None => {}
                }
            }
        },
        None => {}
    }
}

pub fn draw_text_marks(
    frame: &mut Frame,
    size: Size,
    appearance: Appearance,
    handle_size: Size,
    text_mark_height: f32,
    rail_height: f32,
    markers: Option<&[(Normal, Option<String>, Option<MarkWeight>)]>
) {
    match markers {
        Some(markers) => {
            let rail_bounds = get_frame_rail_bounds(size, handle_size, text_mark_height, rail_height);

            for (normal, text, _) in markers {
                let mark_offset: f32 = rail_bounds.x + normal.scale(rail_bounds.width);

                // println!("========== normal: {:?}", normal);

                match text {
                    Some(text) => {
                        let text_mark = Text {
                            content: text.to_string(),
                            position: Point {
                                x: mark_offset,
                                y: 0.
                            },
                            color: appearance.text_mark_color,
                            size: appearance.text_mark_size,
                            font: appearance.text_mark_font,
                            horizontal_alignment: Horizontal::Center,
                            vertical_alignment: Vertical::Top,
                            line_height: LineHeight::default(),
                            shaping: Shaping::default()
                        };

                        frame.fill_text(text_mark);
                    },
                    None => {}
                }
            }
        },
        None => {}
    }
}

fn get_frame_rail_bounds (
    size: Size,
    handle_size: Size,
    text_mark_height: f32,
    rail_height: f32
) -> Rectangle {
    Rectangle {
        x: handle_size.width * 0.5,
        y: text_mark_height,
        width: size.width - handle_size.width,
        height: rail_height
    }
}

pub fn get_text_and_rail_bounds (
    bounds: Rectangle,
    handle_size: Size,
    text_mark_height: f32,
    rail_height: f32
) -> Rectangle {
    let frame_rail_bounds = get_frame_rail_bounds(bounds.size(), handle_size, text_mark_height, rail_height);
    Rectangle {
        x: bounds.x + frame_rail_bounds.x,
        y: bounds.y,
        width: frame_rail_bounds.width,
        height: frame_rail_bounds.height + text_mark_height
    }
}

pub fn draw_slider_rail(
    frame: &mut Frame,
    size: Size,
    appearance: Appearance,
    handle_size: Size,
    text_mark_height: f32,
    rail_height: f32
) {
    let rail_bounds = get_frame_rail_bounds(size, handle_size, text_mark_height, rail_height);

    let rail = Path::rectangle(rail_bounds.position(), rail_bounds.size());
    let rail_fill = Fill {
        style: Style::Solid(appearance.rail_color),
        ..Fill::default()
    };

    frame.fill(
        &rail,
        rail_fill
    );
}

pub fn get_handle_position(
    size: Size,
    value: Normal,
    handle_size: Size,
    text_mark_height: f32,
    rail_height: f32
) -> Vector {
    let rail_bounds = get_frame_rail_bounds(size, handle_size, text_mark_height, rail_height);

    let handle_offset = value
        .scale(rail_bounds.width);

    Vector {
        x: rail_bounds.x + handle_offset,
        y: rail_bounds.y + rail_bounds.height + RAIL_HANDLE_MARGIN
    }
}

pub fn get_handle_bounds(
    bounds: Rectangle,
    value: Normal,
    handle_size: Size,
    text_mark_height: f32,
    rail_height: f32
) -> Rectangle {
    let handle_position = get_handle_position(bounds.size(), value, handle_size, text_mark_height, rail_height);

    Rectangle {
        x: bounds.x + handle_position.x - handle_size.width * 0.5,
        y: bounds.y + handle_position.y,
        width: handle_size.width,
        height: handle_size.height
    }
}

pub fn draw_handle(
    frame: &mut Frame,
    size: Size,
    value: Normal,
    appearance: Appearance,
    handle_size: Size,
    text_mark_height: f32,
    rail_height: f32
) {
    let handle_position = get_handle_position(size, value, handle_size, text_mark_height, rail_height);

    frame.with_save(|frame| {
        frame.translate(Vector {
            x: handle_position.x,
            y: handle_position.y
        });

        let handle =  Path::new(|f| {
            f.line_to(Point { x: handle_size.width * 0.5, y: handle_size.height });
            f.line_to(Point { x: handle_size.width * -0.5, y: handle_size.height });
            f.line_to(Point { x: 0., y: 0. });
        });

        let handle_fill = Fill {
            style: Style::Solid(appearance.handle_color),
            ..Fill::default()
        };

        frame.fill(
            &handle,
            handle_fill
        );
    });
}
