use wgpu;
use std::mem;

// one instance per shadow, passed directly to the vertex shader
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct ShadowInstance {
    rect: [f32; 4],    // x, y, w, h
    color: [f32; 4],   // r, g, b, a
    params: [f32; 4],  // corner_radius, blur, offset_x, offset_y
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct ScreenUniform {
    size: [f32; 2],
    _pad: [f32; 2],
}

pub struct ShadowRenderer {
    pipeline: wgpu::RenderPipeline,
    instance_buffer: wgpu::Buffer,
    screen_buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    instances: Vec<ShadowInstance>,
    screen_width: f32,
    screen_height: f32,
}

impl ShadowRenderer {
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue, format: wgpu::TextureFormat, width: f32, height: f32) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shadow Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../../shaders/shadow.wgsl").into()),
        });

        let screen_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Shadow Screen Uniform"),
            size: mem::size_of::<ScreenUniform>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let screen_uniform = ScreenUniform { size: [width, height], _pad: [0.0; 2] };
        queue.write_buffer(&screen_buffer, 0, bytemuck::bytes_of(&screen_uniform));

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Shadow BGL"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Shadow BG"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: screen_buffer.as_entire_binding(),
            }],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Shadow Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Shadow Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: mem::size_of::<ShadowInstance>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Instance,
                    attributes: &[
                        // rect: location 0
                        wgpu::VertexAttribute {
                            offset: 0,
                            shader_location: 0,
                            format: wgpu::VertexFormat::Float32x4,
                        },
                        // color: location 1
                        wgpu::VertexAttribute {
                            offset: mem::size_of::<[f32; 4]>() as u64,
                            shader_location: 1,
                            format: wgpu::VertexFormat::Float32x4,
                        },
                        // params: location 2
                        wgpu::VertexAttribute {
                            offset: mem::size_of::<[f32; 8]>() as u64,
                            shader_location: 2,
                            format: wgpu::VertexFormat::Float32x4,
                        },
                    ],
                }],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 4,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        });

        let instance_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Shadow Instance Buffer"),
            size: (256 * mem::size_of::<ShadowInstance>()) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Self {
            pipeline,
            instance_buffer,
            screen_buffer,
            bind_group,
            instances: Vec::new(),
            screen_width: width,
            screen_height: height,
        }
    }

    pub fn draw_shadow(&mut self, x: f32, y: f32, w: f32, h: f32, color: [f32; 4], corner_radius: f32, blur: f32, offset_x: f32, offset_y: f32) {
        self.instances.push(ShadowInstance {
            rect: [x, y, w, h],
            color,
            params: [corner_radius, blur, offset_x, offset_y],
        });
    }

    pub fn clear(&mut self) {
        self.instances.clear();
    }

    pub fn resize(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, width: f32, height: f32) {
        self.screen_width = width;
        self.screen_height = height;
        let screen_uniform = ScreenUniform { size: [width, height], _pad: [0.0; 2] };
        queue.write_buffer(&self.screen_buffer, 0, bytemuck::bytes_of(&screen_uniform));
    }

    pub fn render<'pass>(
        &'pass mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        pass: &mut wgpu::RenderPass<'pass>,
    ) {
        if self.instances.is_empty() { return; }

        let data = bytemuck::cast_slice(&self.instances);
        if data.len() as u64 > self.instance_buffer.size() {
            self.instance_buffer = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("Shadow Instance Buffer"),
                size: (data.len() as u64 * 3 / 2).max(data.len() as u64),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
        }

        queue.write_buffer(&self.instance_buffer, 0, data);
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &self.bind_group, &[]);
        pass.set_vertex_buffer(0, self.instance_buffer.slice(..));
        // 6 vertices per instance (two triangles = one quad)
        pass.draw(0..6, 0..self.instances.len() as u32);
    }
}
