use anyhow::Result;
use wgpu::util::DeviceExt;

use bytemuck::{Pod, Zeroable};

// use egui_wgpu_backend::{RenderPass, ScreenDescriptor};
use egui_winit_platform::{Platform, PlatformDescriptor};

use crate::geometry::Point;

pub mod egui_wgpu;

use egui_wgpu::*;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct GuiVertex {
    pub position: [f32; 2],
    pub uv: [f32; 2],
    pub color: [f32; 4],
}

impl GuiVertex {
    pub fn new(pos: Point, uv: Point, color: [f32; 4]) -> Self {
        Self {
            position: [pos.x, pos.y],
            uv: [uv.x, uv.y],
            color,
        }
    }
}

pub struct Gui {
    pub ctx: egui::CtxRef,

    pub egui_rpass: RenderPass,
    pub screen_descriptor: ScreenDescriptor,
}

impl Gui {
    pub fn new(
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        width: u32,
        height: u32,
    ) -> Self {
        let screen_descriptor = ScreenDescriptor {
            physical_width: width,
            physical_height: height,
            scale_factor: 1.0,
        };

        let egui_rpass = RenderPass::new(&device, format, 1);

        let ctx = egui::CtxRef::default();

        let font_defs = {
            use egui::FontFamily as Family;
            use egui::TextStyle as Style;

            let mut font_defs = egui::FontDefinitions::default();
            let fam_size = &mut font_defs.family_and_size;

            fam_size.insert(Style::Small, (Family::Proportional, 12.0));
            fam_size.insert(Style::Body, (Family::Proportional, 16.0));
            fam_size.insert(Style::Button, (Family::Proportional, 18.0));
            fam_size.insert(Style::Heading, (Family::Proportional, 22.0));
            font_defs
        };
        ctx.set_fonts(font_defs);

        Self {
            ctx,
            egui_rpass,
            screen_descriptor,
        }
    }

    /*
    pub fn begin_frame(&mut self, width: u32, height: u32) {
        self.ctx.begin_frame
    }

    pub fn end_frame(&mut self) -> Vec<egui::ClippedMesh> {
        let (_output, shapes) = self.ctx.end_frame();

        self.ctx.tessellate(shapes)
    }
    */
    // pub fn
}

/*
pub struct Gui {
    pub ctx: egui::CtxRef,
    // frame_input: FrameInput,
    _vs: wgpu::ShaderModule,
    _fs: wgpu::ShaderModule,

    pub vert_uniform: wgpu::Buffer,
    pub frag_uniform: wgpu::Buffer,

    pub vert_bind_group: wgpu::BindGroup,
    pub frag_bind_group: wgpu::BindGroup,
    // pub bind_group: wgpu::BindGroup,
    pub vert_bind_group_layout: wgpu::BindGroupLayout,
    pub frag_bind_group_layout: wgpu::BindGroupLayout,
    pub pipeline_layout: wgpu::PipelineLayout,

    pub render_pipeline: wgpu::RenderPipeline,
}

impl Gui {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Result<Self> {
        let vs_mod = crate::include_shader!("gwas.vert.spv");
        let vs = device.create_shader_module(&vs_mod);

        let fs_mod = crate::include_shader!("gwas.frag.spv");
        let fs = device.create_shader_module(&fs_mod);

        let vertex_size = std::mem::size_of::<GuiVertex>();

        let vert_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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

        let frag_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: None,
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        // min_binding_size: wgpu::BufferSize::new(64),
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        let vert_uniform_contents = [0.0, 0.0];
        let frag_uniform_contents = [0.0, 0.0];

        let vert_uniform = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("GUI Vertex Stage Uniform"),
            contents: bytemuck::cast_slice(&vert_uniform_contents),
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
        });

        let frag_uniform = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("GUI Fragment Stage Uniform"),
            contents: bytemuck::cast_slice(&frag_uniform_contents),
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
        });

        let vert_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &vert_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: vert_uniform.as_entire_binding(),
            }],
            label: None,
        });

        let frag_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &frag_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: frag_uniform.as_entire_binding(),
            }],
            label: None,
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&vert_bind_group_layout, &frag_bind_group_layout],
            push_constant_ranges: &[],
        });

        let vertex_buffers = [wgpu::VertexBufferLayout {
            array_stride: vertex_size as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x2,
                    offset: 0,
                    shader_location: 0,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x2,
                    offset: 2,
                    shader_location: 1,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x2,
                    offset: 4,
                    shader_location: 2,
                },
            ],
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

        let ctx = egui::CtxRef::default();

        let font_defs = {
            use egui::FontFamily as Family;
            use egui::TextStyle as Style;

            let mut font_defs = egui::FontDefinitions::default();
            let fam_size = &mut font_defs.family_and_size;

            fam_size.insert(Style::Small, (Family::Proportional, 12.0));
            fam_size.insert(Style::Body, (Family::Proportional, 16.0));
            fam_size.insert(Style::Button, (Family::Proportional, 18.0));
            fam_size.insert(Style::Heading, (Family::Proportional, 22.0));
            font_defs
        };
        ctx.set_fonts(font_defs);

        Ok(Self {
            ctx,

            _vs: vs,
            _fs: fs,

            vert_uniform,
            frag_uniform,

            vert_bind_group,
            frag_bind_group,

            vert_bind_group_layout,
            frag_bind_group_layout,
            pipeline_layout,
            render_pipeline,
        })
    }
}
*/
