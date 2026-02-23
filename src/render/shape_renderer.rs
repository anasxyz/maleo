use std::mem;
use wgpu;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Instance {
    pos_size: [f32; 4],     //  0
    params: [f32; 4],       //  1  [radius, border_w, aa_width, _pad]
    fill_color: [f32; 4],   //  2
    border_color: [f32; 4], //  3
    clip: [f32; 4],         //  4  [cx, cy, cx2, cy2], all-zero = disabled
    screen_size: [f32; 4],  //  5  [sw, sh, 0, 0]
}

const INSTANCE_ATTRS: &[wgpu::VertexAttribute] = &[
    wgpu::VertexAttribute {
        offset: 0,
        shader_location: 0,
        format: wgpu::VertexFormat::Float32x4,
    },
    wgpu::VertexAttribute {
        offset: 16,
        shader_location: 1,
        format: wgpu::VertexFormat::Float32x4,
    },
    wgpu::VertexAttribute {
        offset: 32,
        shader_location: 2,
        format: wgpu::VertexFormat::Float32x4,
    },
    wgpu::VertexAttribute {
        offset: 48,
        shader_location: 3,
        format: wgpu::VertexFormat::Float32x4,
    },
    wgpu::VertexAttribute {
        offset: 64,
        shader_location: 4,
        format: wgpu::VertexFormat::Float32x4,
    },
    wgpu::VertexAttribute {
        offset: 80,
        shader_location: 5,
        format: wgpu::VertexFormat::Float32x4,
    },
];

pub struct ShapeRenderer {
    pipeline: wgpu::RenderPipeline,
    instance_buffer: wgpu::Buffer,
    instances: Vec<Instance>,
    screen_width: f32,
    screen_height: f32,
}

impl ShapeRenderer {
    pub fn new(
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        width: f32,
        height: f32,
    ) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("SDF Shape Shader"),
            source: wgpu::ShaderSource::Wgsl(
                include_str!("../../shaders/rounded_rect.wgsl").into(),
            ),
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("SDF Shape Pipeline"),
            layout: None,
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: mem::size_of::<Instance>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Instance,
                    attributes: INSTANCE_ATTRS,
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
                ..Default::default()
            },
            depth_stencil: None,
            // MSAA disabled: analytical AA in the fragment shader is strictly
            // better for SDF shapes — MSAA only samples the geometric quad edge,
            // not the SDF edge where it actually matters.
            multisample: wgpu::MultisampleState {
                count: 4,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        });

        let cap = 1024;
        let instance_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("SDF Instance Buffer"),
            size: (cap * mem::size_of::<Instance>()) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Self {
            pipeline,
            instance_buffer,
            instances: Vec::with_capacity(cap),
            screen_width: width,
            screen_height: height,
        }
    }

    // ── public API (names unchanged) ──────────────────────────────────────

    #[inline(always)]
    pub fn clear(&mut self) {
        self.instances.clear();
    }

    /// Axis-aligned rectangle with optional border. No rounding.
    #[inline]
    pub fn draw_rect(
        &mut self,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        color: [f32; 4],
        outline_color: [f32; 4],
        outline_thickness: f32,
    ) {
        self.push(
            x,
            y,
            w,
            h,
            0.0,
            outline_thickness,
            color,
            outline_color,
            [0.0; 4],
        );
    }

    /// rectangle clipped to [cx, cy, cx2, cy2]
    #[inline]
    pub fn draw_rect_clipped(
        &mut self,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        color: [f32; 4],
        clip: [f32; 4],
    ) {
        self.push_clipped(x, y, w, h, 0.0, 0.0, color, [0.0; 4], clip);
    }

    /// rounded rectangle with optional border
    #[inline]
    pub fn draw_rounded_rect(
        &mut self,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        radius: f32,
        color: [f32; 4],
        outline_color: [f32; 4],
        outline_thickness: f32,
    ) {
        let r = radius.min(w * 0.5).min(h * 0.5);
        self.push(
            x,
            y,
            w,
            h,
            r,
            outline_thickness,
            color,
            outline_color,
            [0.0; 4],
        );
    }

    /// circle. radius is the outer radius; the circle is centered at (cx, cy)
    #[inline]
    pub fn draw_circle(
        &mut self,
        cx: f32,
        cy: f32,
        radius: f32,
        color: [f32; 4],
        outline_color: [f32; 4],
        outline_thickness: f32,
    ) {
        let d = radius * 2.0;
        self.push(
            cx - radius,
            cy - radius,
            d,
            d,
            radius,
            outline_thickness,
            color,
            outline_color,
            [0.0; 4],
        );
    }

    // internal push helpers 

    #[inline(always)]
    fn push(
        &mut self,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        radius: f32,
        border_w: f32,
        fill: [f32; 4],
        border: [f32; 4],
        clip: [f32; 4],
    ) {
        self.instances.push(Instance {
            pos_size: [x, y, w, h],
            params: [radius, border_w, 1.0, 0.0],
            fill_color: fill,
            border_color: border,
            clip,
            screen_size: [self.screen_width, self.screen_height, 0.0, 0.0],
        });
    }

    #[inline(always)]
    fn push_clipped(
        &mut self,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        radius: f32,
        border_w: f32,
        fill: [f32; 4],
        border: [f32; 4],
        clip: [f32; 4],
    ) {
        // early out if rect is fully outside clip
        let [cx, cy, cx2, cy2] = clip;
        if x + w <= cx || y + h <= cy || x >= cx2 || y >= cy2 {
            return;
        }
        self.instances.push(Instance {
            pos_size: [x, y, w, h],
            params: [radius, border_w, 1.0, 0.0],
            fill_color: fill,
            border_color: border,
            clip,
            screen_size: [self.screen_width, self.screen_height, 0.0, 0.0],
        });
    }

    // render 

    pub fn render<'pass>(
        &'pass mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        pass: &mut wgpu::RenderPass<'pass>,
    ) {
        if self.instances.is_empty() {
            return;
        }

        let data = bytemuck::cast_slice(&self.instances);

        if data.len() as u64 > self.instance_buffer.size() {
            let new_size = (data.len() as u64 * 2).max(data.len() as u64);
            self.instance_buffer = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("SDF Instance Buffer"),
                size: new_size,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
        }

        queue.write_buffer(&self.instance_buffer, 0, data);
        pass.set_pipeline(&self.pipeline);
        pass.set_vertex_buffer(0, self.instance_buffer.slice(..));
        pass.draw(0..6, 0..self.instances.len() as u32);
    }

    pub fn resize(&mut self, width: f32, height: f32) {
        self.screen_width = width;
        self.screen_height = height;
    }
}
