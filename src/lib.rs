// mod coordinates;
// mod animation;
mod geometry;
mod gui;
mod gwas;
mod state;
mod utils;
mod view;

use gwas::{GwasData, GwasDataChrs, GwasUniforms};
use state::SharedState;
use view::View;
use wasm_bindgen::prelude::*;

// use egui_wgpu_backend::{RenderPass, ScreenDescriptor};
use egui_winit_platform::{Platform, PlatformDescriptor};
use epi::*;
use gui::egui_wgpu::{RenderPass, ScreenDescriptor};

use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

use geometry::{Point, Vertex};

use instant::Instant;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[macro_export]
macro_rules! include_shader {
    ($file:expr) => {
        wgpu::include_spirv!(concat!(env!("CARGO_MANIFEST_DIR"), "/shaders/", $file))
    };
}

/// A custom event type for the winit app.
enum AppEvent {
    RequestRedraw,
}

// struct ExampleRepaintSignal(std::sync::Mutex<winit::event_loop::EventLoopProxy<Event>>);
struct ExampleRepaintSignal();

impl epi::RepaintSignal for ExampleRepaintSignal {
    fn request_repaint(&self) {
        // self.0.lock().unwrap().send_event(Event::RequestRedraw).ok();
    }
}

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

async fn run(event_loop: EventLoop<()>, window: Window) {
    /*
    request
        .headers()
        .set("Accept", "application/vnd.github.v3+json")?;
    */

    let size = window.inner_size();
    let instance = wgpu::Instance::new(wgpu::BackendBit::all());
    let surface = unsafe { instance.create_surface(&window) };
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            // Request an adapter which can render to our surface
            compatible_surface: Some(&surface),
        })
        .await
        .expect("Failed to find an appropriate adapter");

    // Create the logical device and command queue
    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: wgpu::Features::default(),
                limits: wgpu::Limits::default(),
            },
            None,
        )
        .await
        .expect("Failed to create device");

    let swapchain_format = adapter.get_swap_chain_preferred_format(&surface).unwrap();

    let gwas_chr_data = GwasDataChrs::fetch_and_parse(&device, "http://localhost:8080/gwas.json")
        .await
        .unwrap();

    let chr_offsets = gwas_chr_data.chr_offsets(50_000_000);

    // not actually the total len, as it doesn't take the length of
    // the last chr into account, but good enough for now
    let total_len: usize = chr_offsets.values().max().copied().unwrap_or(0);

    let mut gwas_pipeline = gwas::GwasPipeline::new(&device, swapchain_format).unwrap();

    let mut uniforms = GwasUniforms::new(
        &device,
        &gwas_pipeline.bind_group_layout,
        gwas_chr_data.chr_sizes.keys().map(|s| s.as_str()),
    );

    let mut init_view = View {
        center: (total_len as f32) / 2.0,
        ..View::default()
    };

    init_view.scale = 0.55 * init_view.base_bp_width * total_len as f32;

    let state = SharedState {
        view: Default::default(),
        mouse_pos: Default::default(),
    };

    state.view.store(init_view);

    let mut sc_desc = wgpu::SwapChainDescriptor {
        usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
        format: swapchain_format,
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::Mailbox,
    };

    let mut swap_chain = device.create_swap_chain(&surface, &sc_desc);

    // let repaint_signal = std::sync::Arc::new(ExampleRepaintSignal(std::sync::Mutex::new(
    let repaint_signal = std::sync::Arc::new(ExampleRepaintSignal());
    // We use the egui_winit_platform crate as the platform.
    let mut platform = Platform::new(PlatformDescriptor {
        physical_width: size.width as u32,
        physical_height: size.height as u32,
        scale_factor: window.scale_factor(),
        font_definitions: egui::FontDefinitions::default(),
        style: Default::default(),
    });

    let start_time = instant::Instant::now();
    let mut previous_frame_time = None;

    web_sys::console::log_1(&"creating gui".into());
    let mut gui = gui::Gui::new(&device, swapchain_format, size.width, size.height);

    event_loop.run(move |event, _, control_flow| {
        // Have the closure take ownership of the resources.
        // `event_loop.run` never returns, therefore we must do this to ensure
        // the resources are properly cleaned up.
        // let _ = (&instance, &adapter, &vs, &fs, &pipeline_layout);
        let _ = (&instance, &adapter, &gwas_pipeline, &gwas_chr_data);

        platform.handle_event(&event);

        *control_flow = ControlFlow::Wait;
        match event {
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                // Recreate the swap chain with the new size
                sc_desc.width = size.width;
                sc_desc.height = size.height;
                swap_chain = device.create_swap_chain(&surface, &sc_desc);
            }
            Event::MainEventsCleared => {
                // Event::RedrawRequested(_) => {

                platform.update_time(start_time.elapsed().as_secs_f64());

                let frame = swap_chain
                    .get_current_frame()
                    .expect("Failed to acquire next swap chain texture")
                    .output;

                let egui_start = instant::Instant::now();
                platform.begin_frame();
                let mut app_output = epi::backend::AppOutput::default();

                let mut gui_frame = epi::backend::FrameBuilder {
                    info: epi::IntegrationInfo {
                        web_info: None,
                        cpu_usage: previous_frame_time,
                        seconds_since_midnight: None,
                        native_pixels_per_point: Some(window.scale_factor() as _),
                        prefer_dark_mode: None,
                    },
                    tex_allocator: &mut gui.egui_rpass,
                    output: &mut app_output,
                    repaint_signal: repaint_signal.clone(),
                }
                .build();

                egui::Window::new("hello world").show(&platform.context(), |ui| {
                    ui.label("Hello world");
                });

                let view = state.view.load();

                uniforms.write_uniforms(&device, &queue, &chr_offsets, view, -0.8);

                let mut encoder =
                    device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

                let mut clear = true;

                for (chr, bind_group) in uniforms.bind_groups.iter() {
                    let buf = gwas_chr_data.vertex_buffers.get(chr).unwrap();
                    let count = gwas_chr_data.vertex_counts.get(chr).unwrap();

                    let buf = buf.slice(..);

                    gwas_pipeline.draw(&mut encoder, &frame, buf, bind_group, *count, clear);
                    clear = false;
                }

                queue.submit(Some(encoder.finish()));

                let (_output, paint_commands) = platform.end_frame();
                let paint_jobs = platform.context().tessellate(paint_commands);

                let frame_time = (Instant::now() - egui_start).as_secs_f64() as f32;
                previous_frame_time = Some(frame_time);
                let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("encoder"),
                });

                // Upload all resources for the GPU.
                let screen_descriptor = ScreenDescriptor {
                    physical_width: sc_desc.width,
                    physical_height: sc_desc.height,
                    scale_factor: window.scale_factor() as f32,
                };

                /*
                gui.egui_rpass
                    .update_texture(&device, &queue, &platform.context().texture());
                gui.egui_rpass.update_user_textures(&device, &queue);
                gui.egui_rpass
                    .update_buffers(&device, &queue, &paint_jobs, &screen_descriptor);

                // Record all render passes.
                gui.egui_rpass.execute(
                    &mut encoder,
                    &frame.view,
                    &paint_jobs,
                    &screen_descriptor,
                    Some(wgpu::Color::BLACK),
                );

                // Submit the commands.
                // queue.submit(iter::once(encoder.finish()));
                queue.submit(Some(encoder.finish()));
                // *control_flow = ControlFlow::Poll;
                */
            }
            Event::WindowEvent {
                event: WindowEvent::KeyboardInput { input, .. },
                ..
            } => {
                // web_sys::console::log_1(&format!("random: {}", val).into());

                use winit::event::VirtualKeyCode as Key;

                let mut view = state.view.load();
                let w = sc_desc.width as f32;

                match input.virtual_keycode {
                    Some(Key::Left) => {
                        view.center -= (5.0 * view.scale) / w;
                    }
                    Some(Key::Right) => {
                        view.center += (5.0 * view.scale) / w;
                    }
                    _ => (),
                }

                state.view.store(view);
            }
            Event::WindowEvent {
                event: WindowEvent::CursorMoved { position, .. },
                ..
            } => {
                state
                    .mouse_pos
                    .store(Point::new(position.x as f32, position.y as f32));
            }
            Event::WindowEvent {
                event: WindowEvent::MouseWheel { delta, .. },
                ..
            } => {
                let mut view = state.view.load();

                match delta {
                    winit::event::MouseScrollDelta::LineDelta(_, y) => {
                        let delta = 1.00 + (-y / 100.0);
                        view.scale *= delta;
                    }
                    winit::event::MouseScrollDelta::PixelDelta(p) => {
                        let delta = 1.00 + (-p.y / 1000.0) as f32;
                        view.scale *= delta;
                    }
                }
                state.view.store(view);
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => {}
        }
    });
}

#[wasm_bindgen]
pub fn main() {
    let event_loop = EventLoop::new();
    /*
    #[cfg(not(target_arch = "wasm32"))]
    {
        env_logger::init();
        // Temporarily avoid srgb formats for the swapchain on the web
        pollster::block_on(run(event_loop, window));
    }
    */
    #[cfg(target_arch = "wasm32")]
    {
        let window = winit::window::Window::new(&event_loop).unwrap();

        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init().expect("could not initialize logger");
        use winit::platform::web::WindowBuilderExtWebSys;
        use winit::platform::web::WindowExtWebSys;
        // On wasm, append the canvas to the document body

        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| doc.body())
            .and_then(|body| {
                window.canvas().set_width(800);
                window.canvas().set_height(600);

                let canvas_elem = window
                    .canvas()
                    .set_attribute("style", "width: 800px; height: 600px;");

                body.append_child(&web_sys::Element::from(window.canvas()))
                    .ok()
            })
            .expect("couldn't append canvas to document body");
        wasm_bindgen_futures::spawn_local(run(event_loop, window));
    }
}
