use std::fmt::Debug;

mod graphics;
pub mod style;
mod utils;
pub mod normal;

use style::StyleSheet;
use normal::{Normal, NormalParam};
use graphics::*;
use utils::*;

use crate::theme::{DEFAULT_HANDLE_SIZE, DEFAULT_RAIL_HEIGHT, DEFAULT_TEXT_MARKER_HEIGHT};
use crate::speed::MarkWeight;

use iced_core::{
    event, keyboard, layout, mouse::{self, Cursor}, Vector,
    Clipboard, Element, Event, Layout, Length, Point, Rectangle, Shell, Size,
    Renderer as _,
};

// most generic iced renderer
use iced::Renderer;

use iced_widget::canvas::{self, Frame, Cache};
use iced::advanced::{
    renderer,
    widget::{tree, Tree, Widget},
};

static DEFAULT_HEIGHT: u16 = 14;
static DEFAULT_SCALAR: f32 = 0.9575;
static DEFAULT_WHEEL_SCALAR: f32 = 0.01;
static DEFAULT_MODIFIER_SCALAR: f32 = 0.02;

#[allow(missing_debug_implementations)]
pub struct HSlider<'a, Message, Theme>
where
    Theme: StyleSheet,
{
    normal_param: NormalParam,
    on_change: Box<dyn Fn(Normal, Option<usize>) -> Message + 'a>,
    scalar: f32,
    wheel_scalar: f32,
    modifier_scalar: f32,
    modifier_keys: keyboard::Modifiers,
    width: Length,
    height: Length,
    style: <Theme as StyleSheet>::Style,
    geometry_cache: Cache,
    snap_normals: Option<(Vec<f32>, usize)>,
    markers: Option<&'a [(Normal, Option<String>, Option<MarkWeight>)]>,
    handle_size: Size,
    text_mark_height: f32,
    rail_height: f32,
}

impl<'a, Message, Theme> HSlider<'a, Message, Theme>
where
    Message: Clone,
    Theme: StyleSheet,
{
    pub fn new<F>(normal_param: NormalParam, on_change: F) -> Self
    where
        F: 'static + Fn(Normal, Option<usize>) -> Message,
    {
        HSlider {
            normal_param,
            on_change: Box::new(on_change),
            scalar: DEFAULT_SCALAR,
            wheel_scalar: DEFAULT_WHEEL_SCALAR,
            modifier_scalar: DEFAULT_MODIFIER_SCALAR,
            modifier_keys: keyboard::Modifiers::CTRL,
            width: Length::Fill,
            height: Length::from(Length::Fixed(DEFAULT_HEIGHT as f32)),
            style: Default::default(),
            geometry_cache: canvas::Cache::default(),
            snap_normals: None,
            markers: None,
            handle_size: DEFAULT_HANDLE_SIZE,
            text_mark_height: DEFAULT_TEXT_MARKER_HEIGHT,
            rail_height: DEFAULT_RAIL_HEIGHT,
        }
    }

    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }

    pub fn style(mut self, style: impl Into<<Theme as StyleSheet>::Style>) -> Self {
        self.style = style.into();
        self
    }

    pub fn modifier_keys(mut self, modifier_keys: keyboard::Modifiers) -> Self {
        self.modifier_keys = modifier_keys;
        self
    }

    pub fn scalar(mut self, scalar: f32) -> Self {
        self.scalar = scalar;
        self
    }

    pub fn wheel_scalar(mut self, wheel_scalar: f32) -> Self {
        self.wheel_scalar = wheel_scalar;
        self
    }

    pub fn modifier_scalar(mut self, scalar: f32) -> Self {
        self.modifier_scalar = scalar;
        self
    }

    pub fn snap_to_normals(mut self, snap_normals: Option<(Vec<f32>, usize)>) -> Self {
        self.snap_normals = snap_normals;
        self
    }

    pub fn markers(
        mut self,
        markers: Option<&'a [(Normal, Option<String>, Option<MarkWeight>)]>,
    ) -> Self {
        self.markers = markers;
        self
    }

    pub fn handle_size(mut self, handle_size: Size) -> Self {
        self.handle_size = handle_size;
        self
    }

    pub fn text_mark_height(mut self, text_mark_height: f32) -> Self {
        self.text_mark_height = text_mark_height;
        self
    }

    pub fn rail_height(mut self, rail_height: f32) -> Self {
        self.rail_height = rail_height;
        self
    }

    fn move_virtual_slider(
        &mut self,
        state: &mut State,
        messages: &mut Shell<'_, Message>,
        slider_move: SliderMove,
    ) {
        match slider_move {
            SliderMove::Default => {
                self.normal_param.value = self.normal_param.default;
                messages.publish((self.on_change)(self.normal_param.value, None));
            }
            SliderMove::Relative(delta) => match self.try_move_virtual_slider(state, delta) {
                (SliderStatus::Moved, Some(index)) => {
                    messages.publish((self.on_change)(self.normal_param.value, Some(index)));
                }
                (SliderStatus::Moved, None) => {
                    messages.publish((self.on_change)(self.normal_param.value, None));
                }
                _ => {}
            },
        }
    }

    fn try_move_virtual_slider(
        &mut self,
        state: &mut State,
        delta: f32,
    ) -> (SliderStatus, Option<usize>) {
        let mut normal_delta = delta;

        if normal_delta.abs() < f32::EPSILON {
            return (SliderStatus::Unchanged, None);
        }

        if state.pressed_modifiers.contains(self.modifier_keys) {
            normal_delta *= self.modifier_scalar;
        }

        let next_normal = Normal::from_clipped(state.continuous_normal - normal_delta);
        state.continuous_normal = next_normal.as_f32();

        return match &self.snap_normals {
            Some((normals, _)) => {
                let (snap_index, &snap_normal) = find_closest(next_normal.as_f32(), normals);

                // if snap value exists and we're not already snapped to it, snap to it
                if state.last_snapped_normal.is_none()
                    || state.last_snapped_normal.unwrap() != snap_normal
                {
                    self.normal_param.value.set_clipped(snap_normal);

                    state.last_snapped_normal = Some(snap_normal);
                    return (SliderStatus::Moved, Some(snap_index));
                }

                (SliderStatus::Unchanged, None)
            }
            None => {
                self.normal_param.update(next_normal);
                (SliderStatus::Moved, None)
            }
        };
    }
}

#[derive(Debug, Clone)]
pub struct State {
    is_dragging: bool,
    prev_drag_x: f32,
    continuous_normal: f32,
    last_snapped_normal: Option<f32>,
    pressed_modifiers: keyboard::Modifiers,
    last_click: Option<mouse::Click>,
}

// #[derive(Debug, Clone, Copy)]
// enum DragSource {
//     Handle,
//     Rail,
// }

impl State {
    pub fn new(normal_param: NormalParam) -> Self {
        Self {
            is_dragging: false,
            prev_drag_x: 0.0,
            continuous_normal: normal_param.value.as_f32(),
            last_snapped_normal: None,
            pressed_modifiers: Default::default(),
            last_click: None,
        }
    }
}


impl<'a, Message, Theme> Widget<Message, Renderer<Theme>> for HSlider<'a, Message, Theme>
where
    Message: Clone,
    Theme: StyleSheet,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::new(self.normal_param))
    }

    fn width(&self) -> Length {
        Length::Shrink
    }

    fn height(&self) -> Length {
        self.height
    }

    fn layout(&self, _renderer: &Renderer<Theme>, limits: &layout::Limits) -> layout::Node {
        let limits = limits.width(self.width).height(self.height);

        let size = limits.resolve(Size::ZERO);

        layout::Node::new(size)
    }

    fn on_event(
        &mut self,
        tree: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &Renderer<Theme>,
        _clipboard: &mut dyn Clipboard,
        messages: &mut Shell<'_, Message>,
        _: &Rectangle,
    ) -> event::Status {
        let state = tree.state.downcast_mut::<State>();

        match event {
            Event::Mouse(mouse_event) => match mouse_event {
                mouse::Event::CursorMoved { .. } => match state.is_dragging {
                    true => {
                        if let Some(cursor_position) = cursor.position() {
                            let bounds_width = layout.bounds().width;

                            if bounds_width > 0.0 {
                                let normal_delta = (cursor_position.x - state.prev_drag_x)
                                    / bounds_width
                                    * -self.scalar;

                                state.prev_drag_x = cursor_position.x;

                                self.move_virtual_slider(
                                    state,
                                    messages,
                                    SliderMove::Relative(normal_delta),
                                );

                                return event::Status::Captured;
                            }
                        }
                    }
                    false => {}
                },
                mouse::Event::WheelScrolled { delta } => {
                    if self.wheel_scalar == 0.0 {
                        return event::Status::Ignored;
                    }

                    if cursor.position_over(layout.bounds()).is_some() {
                        let lines = match delta {
                            mouse::ScrollDelta::Lines { y, .. } => y,
                            mouse::ScrollDelta::Pixels { y, .. } => {
                                if y > 0.0 {
                                    1.0
                                } else if y < 0.0 {
                                    -1.0
                                } else {
                                    0.0
                                }
                            }
                        };

                        if lines != 0.0 {
                            let normal_delta = -lines * self.wheel_scalar;
                            self.move_virtual_slider(
                                state,
                                messages,
                                SliderMove::Relative(normal_delta),
                            );
                            return event::Status::Captured;
                        }
                    }
                }
                mouse::Event::ButtonPressed(mouse::Button::Left) => {
                    let bounds = layout.bounds();
                    let rail_bounds = get_text_and_rail_bounds(
                        bounds,
                        self.handle_size,
                        self.text_mark_height,
                        self.rail_height,
                    );
                    let handle_bounds = get_handle_bounds(
                        bounds,
                        self.normal_param.value,
                        self.handle_size,
                        self.text_mark_height,
                        self.rail_height,
                    );

                    if let Some(cursor_position) = cursor.position_over(handle_bounds) {
                        let click = mouse::Click::new(cursor_position, state.last_click);

                        match click.kind() {
                            mouse::click::Kind::Single => {
                                state.is_dragging = true;
                                state.prev_drag_x = cursor_position.x;
                            }
                            _ => {
                                state.is_dragging = false;
                                self.move_virtual_slider(state, messages, SliderMove::Default);
                            }
                        }

                        state.last_click = Some(click);

                        return event::Status::Captured;
                    } else if let Some(cursor_position) = cursor.position_over(rail_bounds) {
                        let normal_delta = state.continuous_normal
                                - ((cursor_position.x - rail_bounds.x) / rail_bounds.width);
                            self.move_virtual_slider(
                                state,
                                messages,
                                SliderMove::Relative(normal_delta),
                            );
                        return event::Status::Captured;
                    }
                }
                mouse::Event::ButtonReleased(mouse::Button::Left) => {
                    state.is_dragging = false;
                    state.continuous_normal = self.normal_param.value.as_f32();

                    return event::Status::Captured;
                }
                _ => {}
            },
            Event::Keyboard(keyboard_event) => match keyboard_event {
                keyboard::Event::KeyPressed { modifiers, .. } => {
                    state.pressed_modifiers = modifiers;

                    return event::Status::Captured;
                }
                keyboard::Event::KeyReleased { modifiers, .. } => {
                    state.pressed_modifiers = modifiers;

                    return event::Status::Captured;
                }
                keyboard::Event::ModifiersChanged(modifiers) => {
                    state.pressed_modifiers = modifiers;

                    return event::Status::Captured;
                }
                _ => {}
            },
            _ => {}
        }

        event::Status::Ignored
    }

    fn draw(
        &self,
        _tree: &Tree,
        renderer: &mut Renderer<Theme>,
        theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        let size = bounds.size();
        let is_mouse_over = cursor.position_over(bounds).is_some();

        let appearance = if is_mouse_over {
            theme.hovered(self.style)
        } else {
            theme.active(self.style)
        };

        let static_primitives = self.geometry_cache.draw(renderer, size, |frame| {
            draw_text_marks(frame, size, appearance, self.handle_size, self.text_mark_height, self.rail_height, self.markers);
            draw_slider_rail(frame, size, appearance, self.handle_size, self.text_mark_height, self.rail_height);
            draw_marks(frame, size, appearance, self.handle_size, self.text_mark_height, self.rail_height, self.markers);
        });

        // frame for dynamic primitives
        let mut dynamic_frame = Frame::new(renderer, size);

        draw_handle(
            &mut dynamic_frame,
            size,
            self.normal_param.value,
            appearance,
            self.handle_size,
            self.text_mark_height,
            self.rail_height
        );

        renderer.with_translation(
            Vector::new(bounds.x, bounds.y),
            |renderer| {
                use iced::advanced::graphics::geometry::Renderer as _;

                renderer.draw(vec![
                    static_primitives,
                    dynamic_frame.into_geometry()
                ]);
            },
        );
    }

    fn diff(&self, _tree: &mut Tree) {}

    fn mouse_interaction(
        &self,
        _state: &Tree,
        layout: Layout<'_>,
        cursor: Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer<Theme>,
    ) -> mouse::Interaction {
        let bounds = layout.bounds();
        let cursor_position = cursor.position().unwrap_or(Point::ORIGIN);
        if bounds.contains(cursor_position) {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::default()
        }
    }
}

impl<'a, Message, Theme> From<HSlider<'a, Message, Theme>> for Element<'a, Message, Renderer<Theme>>
where
    Message: 'a + Clone,
    Theme: 'a + StyleSheet,
{
    fn from(h_slider: HSlider<'a, Message, Theme>) -> Self {
        Element::new(h_slider)
    }
}
