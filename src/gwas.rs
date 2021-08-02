use wasm_bindgen::prelude::*;

use wgpu::util::DeviceExt;

use anyhow::Result;

use nalgebra_glm as glm;

use std::collections::HashMap;

use crate::geometry::Vertex;
use crate::view::{View, ViewportDims};

pub struct GwasPipeline {
    vs: wgpu::ShaderModule,
    fs: wgpu::ShaderModule,

    pub bind_group_layout: wgpu::BindGroupLayout,
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

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bind_group_layout],
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

        Ok(Self {
            vs,
            fs,

            bind_group_layout,
            pipeline_layout,
            render_pipeline,
        })
    }

    pub fn draw(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        frame: &wgpu::SwapChainTexture,
        vertex_buf: wgpu::BufferSlice<'_>,
        bind_group: &wgpu::BindGroup,
        vertex_count: usize,
        clear: bool,
    ) {
        let load_op = if clear {
            wgpu::LoadOp::Clear(wgpu::Color::BLACK)
        } else {
            wgpu::LoadOp::Load
        };

        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[wgpu::RenderPassColorAttachment {
                view: &frame.view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: load_op,
                    store: true,
                },
            }],
            depth_stencil_attachment: None,
        });
        rpass.push_debug_group("Prepare data for draw.");
        rpass.set_pipeline(&self.render_pipeline);
        rpass.set_bind_group(0, bind_group, &[]);
        rpass.set_vertex_buffer(0, vertex_buf);
        rpass.pop_debug_group();
        rpass.insert_debug_marker("Draw!");
        rpass.draw(0..(vertex_count as u32), 0..1);
    }
}

pub struct GwasUniforms {
    pub uniform_bufs: HashMap<String, wgpu::Buffer>,

    pub bind_groups: HashMap<String, wgpu::BindGroup>,
}

impl GwasUniforms {
    pub fn new<'a>(
        device: &wgpu::Device,
        bind_group_layout: &wgpu::BindGroupLayout,
        chr_names: impl Iterator<Item = &'a str>,
    ) -> Self {
        let mut bufs: HashMap<String, wgpu::Buffer> = HashMap::default();
        let mut bind_groups: HashMap<String, wgpu::BindGroup> = HashMap::default();

        let default_view = View::default();
        let matrix = default_view.to_scaled_matrix();
        let uniform_contents = [crate::view::mat4_to_array(&matrix)];

        for name in chr_names {
            let label = format!("Chr {} Uniform", name);
            let label = Some(label.as_str());

            let uniform_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label,
                contents: bytemuck::cast_slice(&uniform_contents),
                usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
            });

            let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout: bind_group_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform_buf.as_entire_binding(),
                }],
                label: None,
            });

            bufs.insert(name.to_string(), uniform_buf);

            bind_groups.insert(name.to_string(), bind_group);
        }

        GwasUniforms {
            uniform_bufs: bufs,
            bind_groups,
        }
    }

    pub fn write_uniforms(
        &mut self,
        _device: &wgpu::Device,
        queue: &wgpu::Queue,
        offsets: &HashMap<String, usize>,
        view: View,
        vertical_offset: f32,
    ) {
        for (name, buf) in self.uniform_bufs.iter() {
            let mut offset_view = view;
            let offset = *offsets.get(name.as_str()).unwrap();
            offset_view.center -= offset as f32;

            let matrix = offset_view.to_scaled_matrix();
            let matrix = matrix.append_translation(&glm::vec3(0.0, vertical_offset, 0.0));

            let data = [crate::view::mat4_to_array(&matrix)];

            queue.write_buffer(buf, 0, bytemuck::cast_slice(&data));
        }
    }
}

pub struct GwasDataChrs {
    pub vertex_buffers: HashMap<String, wgpu::Buffer>,
    pub vertex_counts: HashMap<String, usize>,

    pub chr_sizes: HashMap<String, usize>,

    pub data: HashMap<String, Vec<JsValue>>,
}

impl GwasDataChrs {
    pub async fn fetch_and_parse(device: &wgpu::Device, url: &str) -> Result<Self> {
        use wasm_bindgen::JsCast;
        use wasm_bindgen_futures::JsFuture;
        use web_sys::{Request, RequestInit, Response};

        let window = web_sys::window().unwrap();

        let mut opts = RequestInit::new();
        opts.method("GET");

        // TODO handle errors correctly
        let request = Request::new_with_str_and_init(&url, &opts).unwrap();

        let resp_value = JsFuture::from(window.fetch_with_request(&request))
            .await
            .unwrap();

        let resp: Response = resp_value.dyn_into().unwrap();
        let json = JsFuture::from(resp.json().unwrap()).await.unwrap();
        let json_array: js_sys::Array = json.dyn_into().ok().unwrap();

        let mut objects: HashMap<String, Vec<JsValue>> = HashMap::default();
        let mut vertex_datas: HashMap<String, Vec<Vertex>> = HashMap::default();

        // TODO should use a coordinate system file rather than just
        // the max position in each chromosome
        let mut chr_sizes: HashMap<String, usize> = HashMap::default();

        for value in json_array.iter() {
            let chr = js_sys::Reflect::get(&value, &"chr".into()).unwrap();
            let chr = chr.as_string().unwrap();

            let pos = js_sys::Reflect::get(&value, &"ps".into()).unwrap();
            let p = js_sys::Reflect::get(&value, &"p_wald".into()).unwrap();

            let pos = pos.as_f64().unwrap();
            let p = p.as_f64().unwrap();

            objects.entry(chr.clone()).or_default().push(value);

            let size = chr_sizes.entry(chr.clone()).or_default();
            *size = (*size).max(pos as usize);

            let vertices = vertex_datas.entry(chr).or_default();

            let vertex = Vertex::new(pos as f32, p as f32);
            vertices.push(vertex);
            vertices.push(vertex);
            vertices.push(vertex);
        }

        let mut vertex_buffers: HashMap<String, wgpu::Buffer> = HashMap::default();
        let mut vertex_counts: HashMap<String, usize> = HashMap::default();

        for chr in objects.keys() {
            let vertex_data = vertex_datas.get(chr).unwrap();

            let vertex_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(&format!("Vertices, chr {}", chr)),
                contents: bytemuck::cast_slice(&vertex_data),
                usage: wgpu::BufferUsage::VERTEX,
            });

            let vertex_count = vertex_data.len();

            vertex_buffers.insert(chr.to_owned(), vertex_buf);
            vertex_counts.insert(chr.to_owned(), vertex_count);
        }

        Ok(Self {
            vertex_buffers,
            vertex_counts,

            chr_sizes,

            data: objects,
        })
    }

    pub fn chr_offsets(&self, padding: usize) -> HashMap<String, usize> {
        let mut res: HashMap<String, usize> = HashMap::default();

        // TODO use coordinate system order
        let mut keys = self.chr_sizes.keys().collect::<Vec<_>>();
        keys.sort();

        let mut offset = 0;

        for chr in keys {
            let size = *self.chr_sizes.get(chr).unwrap();

            res.insert(chr.to_string(), offset);

            offset += size + padding;
        }

        res
    }
}

pub struct GwasData {
    pub vertex_buf: wgpu::Buffer,
    pub vertex_count: usize,

    data: Box<[JsValue]>,
}

impl GwasData {
    pub async fn fetch_and_parse(device: &wgpu::Device, url: &str) -> Result<Self> {
        use wasm_bindgen::JsCast;
        use wasm_bindgen_futures::JsFuture;
        use web_sys::{Request, RequestInit, Response};

        let window = web_sys::window().unwrap();

        let mut opts = RequestInit::new();
        opts.method("GET");

        // TODO handle errors correctly
        let request = Request::new_with_str_and_init(&url, &opts).unwrap();

        let resp_value = JsFuture::from(window.fetch_with_request(&request))
            .await
            .unwrap();

        let resp: Response = resp_value.dyn_into().unwrap();
        let json = JsFuture::from(resp.json().unwrap()).await.unwrap();
        let json_array: js_sys::Array = json.dyn_into().ok().unwrap();

        /*
          {
            "chr": "1",
            "rs": "rs3683945",
            "ps": 3197400,
            "n_miss": 0,
            "allele1": "A",
            "allele0": "G",
            "af": 0.443,
            "beta": -0.07788665,
            "se": 0.06193502,
            "logl_H1": -1582.163,
            "l_remle": 4.317993,
            "p_wald": 0.2087616
          },

        */

        let mut objects: Vec<JsValue> = Vec::new();
        let mut vertex_data: Vec<Vertex> = Vec::new();

        for value in json_array.iter() {
            let chr = js_sys::Reflect::get(&value, &"chr".into()).unwrap();
            let chr = chr.as_string().unwrap();

            if chr == "1" {
                let pos = js_sys::Reflect::get(&value, &"ps".into()).unwrap();
                let p = js_sys::Reflect::get(&value, &"p_wald".into()).unwrap();

                let pos = pos.as_f64().unwrap();
                let p = p.as_f64().unwrap();

                objects.push(value);
                let vertex = Vertex::new(pos as f32, p as f32);
                vertex_data.push(vertex);
                vertex_data.push(vertex);
                vertex_data.push(vertex);
            }
        }

        let data = objects.into_boxed_slice();

        let vertex_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertices"),
            contents: bytemuck::cast_slice(&vertex_data),
            usage: wgpu::BufferUsage::VERTEX,
        });

        let vertex_count = vertex_data.len();

        Ok(Self {
            vertex_buf,
            vertex_count,

            data,
        })
    }
}
