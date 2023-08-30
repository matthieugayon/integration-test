mod color_utils;
mod controls;
mod h_slider;
pub mod speed;
mod theme;

use controls::Controls;
use theme::Theme;

use iced_wgpu::graphics::Viewport;
use iced_wgpu::{wgpu, Backend, Settings};

use iced_winit::core::mouse;
use iced_winit::core::renderer;
use iced_winit::core::{Color, Size};
use iced_winit::runtime::program;
use iced_winit::runtime::Debug;
use iced_winit::{conversion, futures, winit, Clipboard};

use winit::{
    event::{Event, ModifiersState, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    // Initialize winit
    let event_loop = EventLoop::new();
    let window = winit::window::Window::new(&event_loop)?;

    let physical_size = window.inner_size();
    let mut viewport = Viewport::with_physical_size(
        Size::new(physical_size.width, physical_size.height),
        window.scale_factor(),
    );
    let mut cursor_position = None;
    let mut modifiers = ModifiersState::default();
    let mut clipboard = Clipboard::connect(&window);

    let default_backend = wgpu::Backends::PRIMARY;

    let backend = wgpu::util::backend_bits_from_env().unwrap_or(default_backend);

    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: backend,
        ..Default::default()
    });
    let surface = unsafe { instance.create_surface(&window) }?;

    let (format, (device, queue)) = futures::futures::executor::block_on(async {
        let adapter =
            wgpu::util::initialize_adapter_from_env_or_default(&instance, backend, Some(&surface))
                .await
                .expect("Create adapter");

        let adapter_features = adapter.features();

        let needed_limits = wgpu::Limits::default();

        let capabilities = surface.get_capabilities(&adapter);

        (
            capabilities
                .formats
                .iter()
                .copied()
                .find(wgpu::TextureFormat::is_srgb)
                .or_else(|| capabilities.formats.first().copied())
                .expect("Get preferred format"),
            adapter
                .request_device(
                    &wgpu::DeviceDescriptor {
                        label: None,
                        features: adapter_features & wgpu::Features::default(),
                        limits: needed_limits,
                    },
                    None,
                )
                .await
                .expect("Request device"),
        )
    });

    surface.configure(
        &device,
        &wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: physical_size.width,
            height: physical_size.height,
            present_mode: wgpu::PresentMode::AutoVsync,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![],
        },
    );

    let mut resized = false;

    // Initialize scene and GUI controls
    let controls = Controls::new();

    // Initialize iced
    let mut debug = Debug::new();

    // This is the WGPU renderer
    let renderer =
        iced_wgpu::Renderer::new(Backend::new(&device, &queue, Settings::default(), format));

    // We wrap the renderer in a type that implements the iced renderer trait
    let mut rd = iced_renderer::Renderer::Wgpu(renderer);

    let mut state = program::State::new(
        controls,
        viewport.logical_size(),
        &mut rd, // good type of renderer now
        &mut debug,
    );

    // Run event loop
    event_loop.run(move |event, _, control_flow| {
        // You should change this if you want to render continuosly
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::CursorMoved { position, .. } => {
                        cursor_position = Some(position);
                    }
                    WindowEvent::ModifiersChanged(new_modifiers) => {
                        modifiers = new_modifiers;
                    }
                    WindowEvent::Resized(_) => {
                        resized = true;
                    }
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                    }
                    _ => {}
                }

                // Map window event to iced event
                if let Some(event) =
                    iced_winit::conversion::window_event(&event, window.scale_factor(), modifiers)
                {
                    state.queue_event(event);
                }
            }
            Event::MainEventsCleared => {
                // If there are events pending
                if !state.is_queue_empty() {
                    // We update iced
                    let _ = state.update(
                        viewport.logical_size(),
                        cursor_position
                            .map(|p| conversion::cursor_position(p, viewport.scale_factor()))
                            .map(mouse::Cursor::Available)
                            .unwrap_or(mouse::Cursor::Unavailable),
                        &mut rd,
                        &Theme::Dark,
                        &renderer::Style {
                            text_color: Color::WHITE,
                        },
                        &mut clipboard,
                        &mut debug,
                    );

                    // and request a redraw
                    window.request_redraw();
                }
            }
            Event::RedrawRequested(_) => {
                if resized {
                    let size = window.inner_size();

                    viewport = Viewport::with_physical_size(
                        Size::new(size.width, size.height),
                        window.scale_factor(),
                    );

                    surface.configure(
                        &device,
                        &wgpu::SurfaceConfiguration {
                            format,
                            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                            width: size.width,
                            height: size.height,
                            present_mode: wgpu::PresentMode::AutoVsync,
                            alpha_mode: wgpu::CompositeAlphaMode::Auto,
                            view_formats: vec![],
                        },
                    );

                    resized = false;
                }

                match surface.get_current_texture() {
                    Ok(frame) => {
                        let mut encoder =
                            device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                                label: None,
                            });

                        let program = state.program();

                        let view = frame
                            .texture
                            .create_view(&wgpu::TextureViewDescriptor::default());

                        // We clear the frame
                        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                            label: None,
                            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                                view: &view,
                                resolve_target: None,
                                ops: wgpu::Operations {
                                    load: wgpu::LoadOp::Clear({
                                        let [r, g, b, a] = program.background_color().into_linear();

                                        wgpu::Color {
                                            r: r as f64,
                                            g: g as f64,
                                            b: b as f64,
                                            a: a as f64,
                                        }
                                    }),
                                    store: true,
                                },
                            })],
                            depth_stencil_attachment: None,
                        });

                        if let iced_renderer::Renderer::Wgpu(ref mut inner_renderer) = rd {
                            // And then iced on top
                            inner_renderer.with_primitives(|backend, primitive| {
                                backend.present(
                                    &device,
                                    &queue,
                                    &mut encoder,
                                    None,
                                    &view,
                                    primitive,
                                    &viewport,
                                    &debug.overlay(),
                                );
                            });
                        }

                        // Then we submit the work
                        queue.submit(Some(encoder.finish()));
                        frame.present();

                        // Update the mouse cursor
                        window.set_cursor_icon(iced_winit::conversion::mouse_interaction(
                            state.mouse_interaction(),
                        ));
                    }
                    Err(error) => match error {
                        wgpu::SurfaceError::OutOfMemory => {
                            panic!(
                                "Swapchain error: {error}. \
                                Rendering cannot continue."
                            )
                        }
                        _ => {
                            // Try rendering again next frame.
                            window.request_redraw();
                        }
                    },
                }
            }
            _ => {}
        }
    })
}
