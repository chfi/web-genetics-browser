use wasm_bindgen::prelude::*;

use wgpu::util::DeviceExt;

use anyhow::Result;

use nalgebra_glm as glm;

use crate::geometry::Vertex;
use crate::view::{View, ViewportDims};

pub struct GwasPipeline {
    vs: wgpu::ShaderModule,
    fs: wgpu::ShaderModule,

    pub vertex_buf: wgpu::Buffer,
    pub vertex_count: usize,

    pub uniform_buf: wgpu::Buffer,

    pub bind_group_layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,

    pub pipeline_layout: wgpu::PipelineLayout,

    pub render_pipeline: wgpu::RenderPipeline,
}

impl GwasPipeline {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Result<Self> {
        let vs_mod = crate::include_shader!("gwas.vert.spv");
        let vs = device.create_shader_module(&vs_mod);

        let fs_mod = crate::include_shader!("gwas.frag.spv");
        let fs = device.create_shader_module(&fs_mod);

        let vertex_size = std::mem::size_of::<Vertex>();

        let vertex_data = crate::geometry::example_vertices();

        let vertex_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertices"),
            contents: bytemuck::cast_slice(&vertex_data),
            usage: wgpu::BufferUsage::VERTEX,
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStage::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    // min_binding_size: wgpu::BufferSize::new(64),
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let default_view = View::default();
        let matrix = default_view.to_scaled_matrix();
        let uniform_contents = [crate::view::mat4_to_array(&matrix)];
        // let uniform_contents = [glm::value_ptr(&matrix)];
        // let uniform_contents_test = [0.0, 0.0, 0.0, 0.0];
        let uniform_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::cast_slice(&uniform_contents),
            // contents: bytemuck::cast_slice(mx_ref),
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buf.as_entire_binding(),
            }],
            label: None,
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bind_group_layout],
            // bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let vertex_buffers = [wgpu::VertexBufferLayout {
            array_stride: vertex_size as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &[wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x2,
                offset: 0,
                shader_location: 0,
            }],
        }];

        let primitive_state = wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            ..wgpu::PrimitiveState::default()
        };

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &vs,
                entry_point: "main",
                buffers: &vertex_buffers,
            },
            fragment: Some(wgpu::FragmentState {
                module: &fs,
                entry_point: "main",
                targets: &[wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent {
                            operation: wgpu::BlendOperation::Add,
                            src_factor: wgpu::BlendFactor::SrcAlpha,
                            dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                        },
                        alpha: wgpu::BlendComponent {
                            operation: wgpu::BlendOperation::Add,
                            src_factor: wgpu::BlendFactor::SrcAlpha,
                            dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                        },
                    }),
                    write_mask: wgpu::ColorWrite::ALL,
                }],
            }),
            primitive: primitive_state,
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
        });

        // TODO use the GWAS data point count
        let vertex_count = vertex_data.len();

        Ok(Self {
            vs,
            fs,

            vertex_buf,
            vertex_count,

            uniform_buf,

            bind_group_layout,
            bind_group,

            pipeline_layout,
            render_pipeline,
        })
    }

    pub fn draw(&self, encoder: &mut wgpu::CommandEncoder, frame: &wgpu::SwapChainTexture) {
        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[wgpu::RenderPassColorAttachment {
                view: &frame.view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: true,
                },
            }],
            depth_stencil_attachment: None,
        });
        rpass.push_debug_group("Prepare data for draw.");
        rpass.set_pipeline(&self.render_pipeline);
        rpass.set_bind_group(0, &self.bind_group, &[]);
        rpass.set_vertex_buffer(0, self.vertex_buf.slice(..));
        rpass.pop_debug_group();
        rpass.insert_debug_marker("Draw!");
        rpass.draw(0..(self.vertex_count as u32), 0..1);
    }

    pub fn write_uniform(&mut self, _device: &wgpu::Device, queue: &wgpu::Queue, new_view: View) {
        let matrix = new_view.to_scaled_matrix();
        let data = [crate::view::mat4_to_array(&matrix)];

        queue.write_buffer(&self.uniform_buf, 0, bytemuck::cast_slice(&data));
    }
}
