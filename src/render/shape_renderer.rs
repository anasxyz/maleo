use wgpu;
use std::mem;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 4],
}

pub struct ShapeRenderer {
    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    vertices: Vec<Vertex>,
    screen_width: f32,
    screen_height: f32,
    vertex_capacity: usize,
    ndc_scale_x: f32,
    ndc_scale_y: f32,
}

impl ShapeRenderer {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat, width: f32, height: f32) -> Self {
        let vertex_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shape Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../../shaders/shape.wgsl").into()),
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Shape Pipeline"),
            layout: None,
            vertex: wgpu::VertexState {
                module: &vertex_shader,
                entry_point: Some("vs_main"),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &[
                        wgpu::VertexAttribute {
                            offset: 0,
                            shader_location: 0,
                            format: wgpu::VertexFormat::Float32x2,
                        },
                        wgpu::VertexAttribute {
                            offset: mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                            shader_location: 1,
                            format: wgpu::VertexFormat::Float32x4,
                        },
                    ],
                }],
            },
            fragment: Some(wgpu::FragmentState {
                module: &vertex_shader,
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

        let vertex_capacity = 4096;
        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Shape Vertex Buffer"),
            size: (vertex_capacity * mem::size_of::<Vertex>()) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let ndc_scale_x = 2.0 / width;
        let ndc_scale_y = 2.0 / height;

        Self {
            pipeline,
            vertex_buffer,
            vertices: Vec::with_capacity(vertex_capacity),
            screen_width: width,
            screen_height: height,
            vertex_capacity,
            ndc_scale_x,
            ndc_scale_y,
        }
    }

    #[inline(always)]
    pub fn clear(&mut self) {
        self.vertices.clear();
    }

    #[inline(always)]
    fn to_ndc(&self, x: f32, y: f32) -> [f32; 2] {
        [
            x * self.ndc_scale_x - 1.0,
            1.0 - y * self.ndc_scale_y,
        ]
    }

    #[inline(always)]
    fn push_quad(&mut self, p1: [f32; 2], p2: [f32; 2], p3: [f32; 2], p4: [f32; 2], color: [f32; 4]) {
        self.vertices.reserve(6);
        
        unsafe {
            let len = self.vertices.len();
            let ptr = self.vertices.as_mut_ptr().add(len);
            
            ptr.write(Vertex { position: p1, color });
            ptr.add(1).write(Vertex { position: p2, color });
            ptr.add(2).write(Vertex { position: p3, color });
            ptr.add(3).write(Vertex { position: p2, color });
            ptr.add(4).write(Vertex { position: p4, color });
            ptr.add(5).write(Vertex { position: p3, color });
            
            self.vertices.set_len(len + 6);
        }
    }

    pub fn rect(&mut self, x: f32, y: f32, w: f32, h: f32, color: [f32; 4], outline_color: [f32; 4], outline_thickness: f32) {
        let p1 = self.to_ndc(x, y);
        let p2 = self.to_ndc(x + w, y);
        let p3 = self.to_ndc(x, y + h);
        let p4 = self.to_ndc(x + w, y + h);
        
        self.push_quad(p1, p2, p3, p4, color);

        if outline_thickness > 0.0 {
            let half = outline_thickness * 0.5;
            self.rect_outline_fast(x, y, w, h, outline_color, half);
        }
    }

    #[inline]
    fn rect_outline_fast(&mut self, x: f32, y: f32, w: f32, h: f32, color: [f32; 4], half: f32) {
        self.push_quad(
            self.to_ndc(x - half, y - half),
            self.to_ndc(x + w + half, y - half),
            self.to_ndc(x - half, y + half),
            self.to_ndc(x + w + half, y + half),
            color
        );

        self.push_quad(
            self.to_ndc(x - half, y + h - half),
            self.to_ndc(x + w + half, y + h - half),
            self.to_ndc(x - half, y + h + half),
            self.to_ndc(x + w + half, y + h + half),
            color
        );

        self.push_quad(
            self.to_ndc(x - half, y + half),
            self.to_ndc(x + half, y + half),
            self.to_ndc(x - half, y + h - half),
            self.to_ndc(x + half, y + h - half),
            color
        );

        self.push_quad(
            self.to_ndc(x + w - half, y + half),
            self.to_ndc(x + w + half, y + half),
            self.to_ndc(x + w - half, y + h - half),
            self.to_ndc(x + w + half, y + h - half),
            color
        );
    }

    #[inline(always)]
    pub fn draw_rect(&mut self, x: f32, y: f32, w: f32, h: f32, color: [f32; 4], outline_color: [f32; 4], outline_thickness: f32) {
        self.rect(x, y, w, h, color, outline_color, outline_thickness);
    }

    pub fn circle(&mut self, cx: f32, cy: f32, radius: f32, color: [f32; 4], outline_color: [f32; 4], outline_thickness: f32) {
        const SEGMENTS: usize = 32;
        
        self.vertices.reserve(SEGMENTS * 3);
        
        let center = self.to_ndc(cx, cy);
        
        use std::sync::LazyLock;
        static CIRCLE_LUT: LazyLock<[(f32, f32); 33]> = LazyLock::new(|| {
            let mut lut = [(0.0, 0.0); 33];
            for i in 0..=32 {
                let angle = (i as f32 / 32.0) * 2.0 * std::f32::consts::PI;
                lut[i] = (angle.cos(), angle.sin());
            }
            lut
        });
        
        for i in 0..SEGMENTS {
            let (cos1, sin1) = CIRCLE_LUT[i];
            let (cos2, sin2) = CIRCLE_LUT[i + 1];
            
            let p1 = self.to_ndc(cx + radius * cos1, cy + radius * sin1);
            let p2 = self.to_ndc(cx + radius * cos2, cy + radius * sin2);
            
            unsafe {
                let len = self.vertices.len();
                let ptr = self.vertices.as_mut_ptr().add(len);
                ptr.write(Vertex { position: center, color });
                ptr.add(1).write(Vertex { position: p1, color });
                ptr.add(2).write(Vertex { position: p2, color });
                self.vertices.set_len(len + 3);
            }
        }

        if outline_thickness > 0.0 {
            self.circle_outline_fast(cx, cy, radius, outline_color, outline_thickness);
        }
    }

    #[inline]
    fn circle_outline_fast(&mut self, cx: f32, cy: f32, radius: f32, color: [f32; 4], thickness: f32) {
        const SEGMENTS: usize = 32;
        
        let inner_radius = radius - thickness * 0.5;
        let outer_radius = radius + thickness * 0.5;
        
        self.vertices.reserve(SEGMENTS * 6);
        
        use std::sync::LazyLock;
        static CIRCLE_LUT: LazyLock<[(f32, f32); 33]> = LazyLock::new(|| {
            let mut lut = [(0.0, 0.0); 33];
            for i in 0..=32 {
                let angle = (i as f32 / 32.0) * 2.0 * std::f32::consts::PI;
                lut[i] = (angle.cos(), angle.sin());
            }
            lut
        });
        
        for i in 0..SEGMENTS {
            let (cos1, sin1) = CIRCLE_LUT[i];
            let (cos2, sin2) = CIRCLE_LUT[i + 1];
            
            let inner1 = self.to_ndc(cx + inner_radius * cos1, cy + inner_radius * sin1);
            let inner2 = self.to_ndc(cx + inner_radius * cos2, cy + inner_radius * sin2);
            let outer1 = self.to_ndc(cx + outer_radius * cos1, cy + outer_radius * sin1);
            let outer2 = self.to_ndc(cx + outer_radius * cos2, cy + outer_radius * sin2);
            
            self.push_quad(inner1, outer1, inner2, outer2, color);
        }
    }

    #[inline(always)]
    pub fn draw_circle(&mut self, cx: f32, cy: f32, radius: f32, color: [f32; 4], outline_color: [f32; 4], outline_thickness: f32) {
        self.circle(cx, cy, radius, color, outline_color, outline_thickness);
    }

    pub fn rounded_rect(&mut self, x: f32, y: f32, w: f32, h: f32, radius: f32, color: [f32; 4], outline_color: [f32; 4], outline_thickness: f32) {
        let radius = radius.min(w * 0.5).min(h * 0.5);
        
        self.rect(x + radius, y, w - radius * 2.0, h, color, [0.0; 4], 0.0);
        self.rect(x, y + radius, radius, h - radius * 2.0, color, [0.0; 4], 0.0);
        self.rect(x + w - radius, y + radius, radius, h - radius * 2.0, color, [0.0; 4], 0.0);
        
        self.quarter_circle_fast(x + radius, y + radius, radius, color, 2);
        self.quarter_circle_fast(x + w - radius, y + radius, radius, color, 3);
        self.quarter_circle_fast(x + w - radius, y + h - radius, radius, color, 0);
        self.quarter_circle_fast(x + radius, y + h - radius, radius, color, 1);

        if outline_thickness > 0.0 {
            self.rounded_rect_outline_fast(x, y, w, h, radius, outline_color, outline_thickness);
        }
    }

    #[inline]
    fn quarter_circle_fast(&mut self, cx: f32, cy: f32, radius: f32, color: [f32; 4], quarter: u32) {
        const SEGMENTS: usize = 8;
        let start_angle = quarter as f32 * std::f32::consts::FRAC_PI_2;
        
        self.vertices.reserve(SEGMENTS * 3);
        let center = self.to_ndc(cx, cy);
        
        for i in 0..SEGMENTS {
            let angle1 = start_angle + (i as f32 / SEGMENTS as f32) * std::f32::consts::FRAC_PI_2;
            let angle2 = start_angle + ((i + 1) as f32 / SEGMENTS as f32) * std::f32::consts::FRAC_PI_2;
            
            let p1 = self.to_ndc(cx + radius * angle1.cos(), cy + radius * angle1.sin());
            let p2 = self.to_ndc(cx + radius * angle2.cos(), cy + radius * angle2.sin());
            
            unsafe {
                let len = self.vertices.len();
                let ptr = self.vertices.as_mut_ptr().add(len);
                ptr.write(Vertex { position: center, color });
                ptr.add(1).write(Vertex { position: p1, color });
                ptr.add(2).write(Vertex { position: p2, color });
                self.vertices.set_len(len + 3);
            }
        }
    }

    #[inline]
    fn rounded_rect_outline_fast(&mut self, x: f32, y: f32, w: f32, h: f32, radius: f32, color: [f32; 4], thickness: f32) {
        let half = thickness * 0.5;
        
        self.rect(x + radius, y - half, w - radius * 2.0, thickness, color, [0.0; 4], 0.0);
        self.rect(x + radius, y + h - half, w - radius * 2.0, thickness, color, [0.0; 4], 0.0);
        self.rect(x - half, y + radius, thickness, h - radius * 2.0, color, [0.0; 4], 0.0);
        self.rect(x + w - half, y + radius, thickness, h - radius * 2.0, color, [0.0; 4], 0.0);
        
        self.quarter_circle_outline_fast(x + radius, y + radius, radius, color, thickness, 2);
        self.quarter_circle_outline_fast(x + w - radius, y + radius, radius, color, thickness, 3);
        self.quarter_circle_outline_fast(x + w - radius, y + h - radius, radius, color, thickness, 0);
        self.quarter_circle_outline_fast(x + radius, y + h - radius, radius, color, thickness, 1);
    }

    #[inline]
    fn quarter_circle_outline_fast(&mut self, cx: f32, cy: f32, radius: f32, color: [f32; 4], thickness: f32, quarter: u32) {
        const SEGMENTS: usize = 8;
        let start_angle = quarter as f32 * std::f32::consts::FRAC_PI_2;
        let inner_radius = radius - thickness * 0.5;
        let outer_radius = radius + thickness * 0.5;
        
        self.vertices.reserve(SEGMENTS * 6);
        
        for i in 0..SEGMENTS {
            let angle1 = start_angle + (i as f32 / SEGMENTS as f32) * std::f32::consts::FRAC_PI_2;
            let angle2 = start_angle + ((i + 1) as f32 / SEGMENTS as f32) * std::f32::consts::FRAC_PI_2;
            
            let (cos1, sin1) = (angle1.cos(), angle1.sin());
            let (cos2, sin2) = (angle2.cos(), angle2.sin());
            
            let inner1 = self.to_ndc(cx + inner_radius * cos1, cy + inner_radius * sin1);
            let inner2 = self.to_ndc(cx + inner_radius * cos2, cy + inner_radius * sin2);
            let outer1 = self.to_ndc(cx + outer_radius * cos1, cy + outer_radius * sin1);
            let outer2 = self.to_ndc(cx + outer_radius * cos2, cy + outer_radius * sin2);
            
            self.push_quad(inner1, outer1, inner2, outer2, color);
        }
    }

    #[inline(always)]
    pub fn draw_rounded_rect(&mut self, x: f32, y: f32, w: f32, h: f32, radius: f32, color: [f32; 4], outline_color: [f32; 4], outline_thickness: f32) {
        self.rounded_rect(x, y, w, h, radius, color, outline_color, outline_thickness);
    }

    pub fn render<'pass>(
        &'pass mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        pass: &mut wgpu::RenderPass<'pass>,
    ) {
        if self.vertices.is_empty() {
            return;
        }

        let vertex_data = bytemuck::cast_slice(&self.vertices);
        let required_size = vertex_data.len() as u64;
        
        if required_size > self.vertex_buffer.size() {
            let new_size = (required_size * 3 / 2).max(required_size);
            self.vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("Shape Vertex Buffer"),
                size: new_size,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
            self.vertex_capacity = (new_size / mem::size_of::<Vertex>() as u64) as usize;
        }
        
        queue.write_buffer(&self.vertex_buffer, 0, vertex_data);

        pass.set_pipeline(&self.pipeline);
        pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        pass.draw(0..self.vertices.len() as u32, 0..1);
    }

    pub fn resize(&mut self, width: f32, height: f32) {
        self.screen_width = width;
        self.screen_height = height;
        self.ndc_scale_x = 2.0 / width;
        self.ndc_scale_y = 2.0 / height;
    }
}
