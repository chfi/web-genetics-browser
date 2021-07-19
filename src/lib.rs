// mod coordinates;
// mod animation;
mod geometry;
mod gwas;
mod state;
mod utils;
mod view;

use state::SharedState;
use wasm_bindgen::prelude::*;

use wgpu::util::DeviceExt;

use geometry::Vertex;

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

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, rust-genetics-browser!");
}

use std::borrow::Cow;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

async fn run(event_loop: EventLoop<()>, window: Window) {
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

    let mut gwas_pipeline = gwas::GwasPipeline::new(&device, swapchain_format).unwrap();

    let mut t = instant::Instant::now();
    let mut fired = false;

    let state = SharedState {
        view: Default::default(),
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
                web_sys::console::log_1(&format!("sec: {}", t.elapsed().as_secs_f64()).into());
                if t.elapsed().as_secs_f32() > 1.0 {
                    fired = true;

                    t = instant::Instant::now();

                    web_sys::console::log_1(&"firing event".into());
                }
                // }

                // Event::RedrawRequested(_) => {
                web_sys::console::log_1(&"rendering".into());
                let frame = swap_chain
                    .get_current_frame()
                    .expect("Failed to acquire next swap chain texture")
                    .output;

                let view = state.view.load();
                gwas_pipeline.write_uniform(&device, &queue, view.scale);

                let mut encoder =
                    device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

                gwas_pipeline.draw(&mut encoder, &frame);

                queue.submit(Some(encoder.finish()));
            }
            Event::WindowEvent {
                event: WindowEvent::MouseWheel { delta, .. },
                ..
            } => {
                let mut view = state.view.load();

                match delta {
                    winit::event::MouseScrollDelta::LineDelta(_, y) => {
                        view.scale += y / 100.0;
                    }
                    winit::event::MouseScrollDelta::PixelDelta(p) => {
                        view.scale += (p.y / 100.0) as f32;
                    }
                }
                web_sys::console::log_1(&format!("zooming to {}", view.scale).into());
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
