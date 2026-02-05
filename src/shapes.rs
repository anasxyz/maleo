// src/shapes.rs

use wgpu;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 2],  // x, y
    color: [f32; 4],     // r, g, b, a
}

pub struct ShapeRenderer {
    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    vertices: Vec<Vertex>,
    screen_width: f32,
    screen_height: f32,
}

impl ShapeRenderer {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat, width: f32, height: f32) -> Self {
        // Load shader from file
        let vertex_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shape Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/shape.wgsl").into()),
        });

        // Create render pipeline with MSAA
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Shape Pipeline"),
            layout: None,
            vertex: wgpu::VertexState {
                module: &vertex_shader,
                entry_point: "vs_main",
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &[
                        // Position
                        wgpu::VertexAttribute {
                            offset: 0,
                            shader_location: 0,
                            format: wgpu::VertexFormat::Float32x2,
                        },
                        // Color
                        wgpu::VertexAttribute {
                            offset: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                            shader_location: 1,
                            format: wgpu::VertexFormat::Float32x4,
                        },
                    ],
                }],
            },
            fragment: Some(wgpu::FragmentState {
                module: &vertex_shader,
                entry_point: "fs_main",
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
            multisample: wgpu::MultisampleState {
                count: 4, // 4x MSAA
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        // Create initial vertex buffer
        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Shape Vertex Buffer"),
            size: 1024 * std::mem::size_of::<Vertex>() as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Self {
            pipeline,
            vertex_buffer,
            vertices: Vec::new(),
            screen_width: width,
            screen_height: height,
        }
    }

    /// Clear all queued shapes
    pub fn clear(&mut self) {
        self.vertices.clear();
    }

    /// Convert screen coordinates to NDC (Normalized Device Coordinates)
    fn to_ndc(&self, x: f32, y: f32) -> [f32; 2] {
        [
            (x / self.screen_width) * 2.0 - 1.0,
            1.0 - (y / self.screen_height) * 2.0,
        ]
    }

    /// Draw a filled rectangle
    pub fn rect(&mut self, x: f32, y: f32, w: f32, h: f32, color: [f32; 4]) {
        let p1 = self.to_ndc(x, y);
        let p2 = self.to_ndc(x + w, y);
        let p3 = self.to_ndc(x, y + h);
        let p4 = self.to_ndc(x + w, y + h);
        
        // Two triangles forming a rectangle
        self.vertices.extend_from_slice(&[
            // Triangle 1
            Vertex { position: p1, color },
            Vertex { position: p2, color },
            Vertex { position: p3, color },
            // Triangle 2
            Vertex { position: p2, color },
            Vertex { position: p4, color },
            Vertex { position: p3, color },
        ]);
    }

    /// Alias for rect()
    pub fn draw_rect(&mut self, x: f32, y: f32, w: f32, h: f32, color: [f32; 4]) {
        self.rect(x, y, w, h, color);
    }

    /// Draw a filled circle
    pub fn circle(&mut self, cx: f32, cy: f32, radius: f32, color: [f32; 4]) {
        let segments = 32;
        let pi = std::f32::consts::PI;
        
        let center = self.to_ndc(cx, cy);
        
        for i in 0..segments {
            let angle1 = (i as f32 / segments as f32) * 2.0 * pi;
            let angle2 = ((i + 1) as f32 / segments as f32) * 2.0 * pi;
            
            let p1 = self.to_ndc(cx + radius * angle1.cos(), cy + radius * angle1.sin());
            let p2 = self.to_ndc(cx + radius * angle2.cos(), cy + radius * angle2.sin());
            
            self.vertices.extend_from_slice(&[
                Vertex { position: center, color },
                Vertex { position: p1, color },
                Vertex { position: p2, color },
            ]);
        }
    }

    /// Alias for circle()
    pub fn draw_circle(&mut self, cx: f32, cy: f32, radius: f32, color: [f32; 4]) {
        self.circle(cx, cy, radius, color);
    }

    /// Draw a rounded rectangle
    pub fn rounded_rect(&mut self, x: f32, y: f32, w: f32, h: f32, radius: f32, color: [f32; 4]) {
        let radius = radius.min(w / 2.0).min(h / 2.0);
        
        // Center rectangle
        self.rect(x + radius, y, w - radius * 2.0, h, color);
        // Left rectangle
        self.rect(x, y + radius, radius, h - radius * 2.0, color);
        // Right rectangle
        self.rect(x + w - radius, y + radius, radius, h - radius * 2.0, color);
        
        // Four corner circles
        self.quarter_circle(x + radius, y + radius, radius, color, 2); // Top-left
        self.quarter_circle(x + w - radius, y + radius, radius, color, 3); // Top-right
        self.quarter_circle(x + w - radius, y + h - radius, radius, color, 0); // Bottom-right
        self.quarter_circle(x + radius, y + h - radius, radius, color, 1); // Bottom-left
    }

    /// Alias for rounded_rect()
    pub fn draw_rounded_rect(&mut self, x: f32, y: f32, w: f32, h: f32, radius: f32, color: [f32; 4]) {
        self.rounded_rect(x, y, w, h, radius, color);
    }

    /// Draw a quarter circle (for rounded corners)
    fn quarter_circle(&mut self, cx: f32, cy: f32, radius: f32, color: [f32; 4], quarter: u32) {
        let segments = 8;
        let pi = std::f32::consts::PI;
        let start_angle = quarter as f32 * pi / 2.0;
        
        let center = self.to_ndc(cx, cy);
        
        for i in 0..segments {
            let angle1 = start_angle + (i as f32 / segments as f32) * pi / 2.0;
            let angle2 = start_angle + ((i + 1) as f32 / segments as f32) * pi / 2.0;
            
            let p1 = self.to_ndc(cx + radius * angle1.cos(), cy + radius * angle1.sin());
            let p2 = self.to_ndc(cx + radius * angle2.cos(), cy + radius * angle2.sin());
            
            self.vertices.extend_from_slice(&[
                Vertex { position: center, color },
                Vertex { position: p1, color },
                Vertex { position: p2, color },
            ]);
        }
    }

    /// Render all queued shapes
    pub fn render<'pass>(
        &'pass mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        pass: &mut wgpu::RenderPass<'pass>,
    ) {
        if self.vertices.is_empty() {
            return;
        }

        // Upload vertices to GPU
        let vertex_data = bytemuck::cast_slice(&self.vertices);
        
        // Recreate buffer if needed
        let required_size = vertex_data.len() as u64;
        if required_size > self.vertex_buffer.size() {
            self.vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("Shape Vertex Buffer"),
                size: required_size * 2,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
        }
        
        queue.write_buffer(&self.vertex_buffer, 0, vertex_data);

        // Draw
        pass.set_pipeline(&self.pipeline);
        pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        pass.draw(0..self.vertices.len() as u32, 0..1);
    }

    /// Update screen dimensions
    pub fn resize(&mut self, width: f32, height: f32) {
        self.screen_width = width;
        self.screen_height = height;
    }
}
