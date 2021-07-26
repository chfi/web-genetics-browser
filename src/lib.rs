// mod coordinates;
// mod animation;
mod geometry;
mod gwas;
mod state;
mod utils;
mod view;

use gwas::GwasData;
use state::SharedState;
use wasm_bindgen::prelude::*;

use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

use geometry::{Point, Vertex};

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
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default(),
            },
            None,
        )
        .await
        .expect("Failed to create device");

    let swapchain_format = adapter.get_swap_chain_preferred_format(&surface).unwrap();

    let gwas_data = GwasData::fetch_and_parse(&device, "http://localhost:8080/gwas.json")
        .await
        .unwrap();

    let mut gwas_pipeline = gwas::GwasPipeline::new(&device, swapchain_format).unwrap();

    let state = SharedState {
        view: Default::default(),
        mouse_pos: Default::default(),
    };

    let mut sc_desc = wgpu::SwapChainDescriptor {
        usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
        format: swapchain_format,
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::Mailbox,
    };

    let mut swap_chain = device.create_swap_chain(&surface, &sc_desc);

    event_loop.run(move |event, _, control_flow| {
        // Have the closure take ownership of the resources.
        // `event_loop.run` never returns, therefore we must do this to ensure
        // the resources are properly cleaned up.
        // let _ = (&instance, &adapter, &vs, &fs, &pipeline_layout);
        let _ = (&instance, &adapter, &gwas_pipeline);

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
                let frame = swap_chain
                    .get_current_frame()
                    .expect("Failed to acquire next swap chain texture")
                    .output;

                let view = state.view.load();
                gwas_pipeline.write_uniform(&device, &queue, view);

                let mut encoder =
                    device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

                let buf = gwas_data.vertex_buf.slice(..);
                gwas_pipeline.draw(&mut encoder, &frame, buf, gwas_data.vertex_count);

                queue.submit(Some(encoder.finish()));
            }
            Event::WindowEvent {
                event: WindowEvent::KeyboardInput { input, .. },
                ..
            } => {
                // web_sys::console::log_1(&format!("random: {}", val).into());
                use winit::event::VirtualKeyCode as Key;
                match input.virtual_keycode {
                    Some(Key::Left) => {
                        let mut view = state.view.load();
                        view.center -= 0.1 / view.scale;
                        state.view.store(view);
                    }
                    Some(Key::Right) => {
                        let mut view = state.view.load();
                        view.center += 0.1 / view.scale;
                        state.view.store(view);
                    }
                    _ => (),
                }
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
