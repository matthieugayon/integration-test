use crate::h_slider::{HSlider, normal::Normal};
use crate::theme::Theme;
use crate::speed::{
    MarkWeight, SpeedMode, SpeedValue, SpeedRange,
    DEFAULT_QUANTIZED_SPEED_INDEX, QUANTIZED_SPEEDS
};

use iced_wgpu::Renderer;
use iced_widget::{Row, Text, Checkbox};
use iced_winit::core::{Alignment, Color, Element, Length};
use iced_winit::runtime::{Command, Program};
use lazy_static::lazy_static;

pub struct App {
    background_color: Color,
    speed_mode: SpeedMode,
    speed_range: SpeedRange,
    speed: SpeedValue
}

#[derive(Debug, Clone)]
pub enum Message {
    SetSpeed(SpeedValue),
    SetSpeedMode(SpeedMode)
}

impl App {
    pub fn new() -> App {
        App {
            background_color: Color::BLACK,
            speed_mode: SpeedMode::Quantized,
            speed_range: SpeedRange::default(),
            speed: SpeedValue::Quantized(DEFAULT_QUANTIZED_SPEED_INDEX),
        }
    }

    pub fn background_color(&self) -> Color {
        self.background_color
    }
}

impl Program for App {
    type Renderer = Renderer<Theme>;
    type Message = Message;

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::SetSpeed(speed) => {
                self.speed = speed;
            },
            Message::SetSpeedMode(mode) => {
                self.speed_mode = mode;
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<Message, Renderer<Theme>> {
        let range = self.speed_range;

        let (speed_normal, display_value) = match self.speed {
            SpeedValue::Quantized(index) => {
                let quantized_speed = &QUANTIZED_SPEEDS[index];
                let quantized_value = quantized_speed.numerator / quantized_speed.denominator;

                (range.normal_param(quantized_value, 1.), quantized_speed.text.to_string())
            },
            SpeedValue::Unquantized(value) => {
                (range.normal_param(value, 1.), format!("{:0>1.2}", value))
            }
        };

        let get_message = move |normal, opt_index| {
            match opt_index {
                Some(index) => {
                    Message::SetSpeed(SpeedValue::Quantized(index))
                },
                None => {
                    Message::SetSpeed(SpeedValue::Unquantized(range.unmap_to_value(normal)))
                },
            }
        };

        let snappable_option = match self.speed_mode {
            SpeedMode::Quantized => Some((QUANTIZED_SPEED_NORMALS.to_vec(), DEFAULT_QUANTIZED_SPEED_INDEX)),
            SpeedMode::Unquantized => None,
        };

        let quantize_btn = Checkbox::new(
            "Quantised",
            self.speed_mode == SpeedMode::Quantized,
            |val| Message::SetSpeedMode(if val { SpeedMode::Quantized } else { SpeedMode::Unquantized })
        );

        Row::new()
            .spacing(8)
            .push(quantize_btn)
            .push(
                HSlider::new(
                    speed_normal,
                    move |normal, index| get_message(normal, index),
                )
                .snap_to_normals(snappable_option)
                .markers(Some(MARKERS.as_slice()))
                .height(Length::Fill)
                .width(Length::Fill)
            )
            .push(
                Text::new(display_value)
                    .size(14)
                    .width(Length::Fixed(26.))
                    .horizontal_alignment(iced::alignment::Horizontal::Center)
            )
            .align_items(Alignment::Center)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

fn generate_speed_normals() -> Vec<f32> {
    QUANTIZED_SPEEDS
        .into_iter()
        .map(|speed_value| {
            SpeedRange::default().map_to_normal(
                speed_value.numerator / speed_value.denominator
            ).as_f32()
        })
        .collect()
}

fn generate_markers() -> Vec<(Normal, Option<String>, Option<MarkWeight>)> {
    QUANTIZED_SPEEDS
        .into_iter()
        .map(|quantized_speed| {
            (
                SpeedRange::default().map_to_normal(
                    quantized_speed.numerator / quantized_speed.denominator
                ),
                { if quantized_speed.text_mark.is_none() { None } else { Some(quantized_speed.text_mark.unwrap().to_string())} },
                quantized_speed.mark_weight
            )
        })
        .collect()
}

fn generate_tick_normals(numbers: Vec<f32>) -> Vec<Normal> {
    numbers.into_iter()
        .map(|value| {
            // println!("value: {}", value);
            // Normal::from_clipped(value)
            Normal::new(value)
        })
        .collect()
}

lazy_static! {
    #[derive(Debug)]
    pub static ref QUANTIZED_SPEED_NORMALS: Vec<f32> = generate_speed_normals();
    #[derive(Debug)]
    pub static ref TICK_NORMALS: Vec<Normal> = generate_tick_normals(QUANTIZED_SPEED_NORMALS.to_vec());
    #[derive(Debug)]
    pub static ref MARKERS: Vec<(Normal, Option<String>, Option<MarkWeight>)> = generate_markers();
}
