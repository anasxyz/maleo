// src/app.rs

use std::sync::Arc;
use wgpu;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use crate::{ShapeRenderer, TextRenderer};

pub struct App {
    event_loop: Option<EventLoop<()>>,
    window: Arc<Window>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface<'static>,
    config: wgpu::SurfaceConfiguration,
    surface_format: wgpu::TextureFormat,
    msaa_texture: wgpu::Texture,
    msaa_view: wgpu::TextureView,
    scale_factor: f64,
}

pub struct Canvas<'a> {
    pub shapes: &'a mut ShapeRenderer,
    pub text: &'a mut TextRenderer,
    pub width: f32,
    pub height: f32,
    pub scale_factor: f64,
}

impl App {
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        pollster::block_on(Self::new_async(title, width, height))
    }

    async fn new_async(title: &str, width: u32, height: u32) -> Self {
        let event_loop = EventLoop::new().unwrap();
        let window = Arc::new(
            WindowBuilder::new()
                .with_title(title)
                .with_inner_size(winit::dpi::LogicalSize::new(width, height)) // Use logical size
                .build(&event_loop)
                .unwrap(),
        );

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
        let surface = instance.create_surface(Arc::clone(&window)).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats[0];

        // Use physical size for rendering, but track scale factor
        let physical_size = window.inner_size();
        let scale_factor = window.scale_factor();
        
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: physical_size.width,
            height: physical_size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

        // Create MSAA texture
        let msaa_texture = Self::create_msaa_texture(&device, &config, surface_format);
        let msaa_view = msaa_texture.create_view(&wgpu::TextureViewDescriptor::default());

        Self {
            event_loop: Some(event_loop),
            window,
            device,
            queue,
            surface,
            config,
            surface_format,
            msaa_texture,
            msaa_view,
            scale_factor,
        }
    }

    fn create_msaa_texture(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        format: wgpu::TextureFormat,
    ) -> wgpu::Texture {
        device.create_texture(&wgpu::TextureDescriptor {
            label: Some("MSAA Texture"),
            size: wgpu::Extent3d {
                width: config.width,
                height: config.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 4,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        })
    }

    pub fn run<F>(mut self, mut render_fn: F)
    where
        F: FnMut(&mut Canvas) + 'static,
    {
        let mut shape_renderer = ShapeRenderer::new(
            &self.device,
            self.surface_format,
            (self.config.width as f64 / self.scale_factor) as f32,
            (self.config.height as f64 / self.scale_factor) as f32,
        );

        let mut text_renderer = TextRenderer::new(&self.device, &self.queue, self.surface_format);

        let event_loop = self.event_loop.take().unwrap();

        let _ = event_loop.run(move |event, target| {
            target.set_control_flow(ControlFlow::Wait);

            match event {
                Event::WindowEvent {
                    event,
                    window_id,
                } if window_id == self.window.id() => match event {
                    WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                        self.scale_factor = scale_factor;
                        let physical_size = self.window.inner_size();
                        self.config.width = physical_size.width;
                        self.config.height = physical_size.height;
                        self.surface.configure(&self.device, &self.config);

                        // Recreate MSAA texture
                        self.msaa_texture = Self::create_msaa_texture(
                            &self.device,
                            &self.config,
                            self.surface_format,
                        );
                        self.msaa_view = self
                            .msaa_texture
                            .create_view(&wgpu::TextureViewDescriptor::default());

                        shape_renderer.resize(
                            (self.config.width as f64 / self.scale_factor) as f32,
                            (self.config.height as f64 / self.scale_factor) as f32,
                        );
                        self.window.request_redraw();
                    }
                    WindowEvent::Resized(new_size) => {
                        self.config.width = new_size.width;
                        self.config.height = new_size.height;
                        self.surface.configure(&self.device, &self.config);

                        // Recreate MSAA texture
                        self.msaa_texture = Self::create_msaa_texture(
                            &self.device,
                            &self.config,
                            self.surface_format,
                        );
                        self.msaa_view = self
                            .msaa_texture
                            .create_view(&wgpu::TextureViewDescriptor::default());

                        shape_renderer.resize(
                            (self.config.width as f64 / self.scale_factor) as f32,
                            (self.config.height as f64 / self.scale_factor) as f32,
                        );
                        self.window.request_redraw();
                    }
                    WindowEvent::RedrawRequested => {
                        let frame = self.surface.get_current_texture().unwrap();
                        let view = frame.texture.create_view(&Default::default());
                        let mut encoder = self
                            .device
                            .create_command_encoder(&Default::default());

                        {
                            let mut pass =
                                encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                                    label: None,
                                    color_attachments: &[Some(
                                        wgpu::RenderPassColorAttachment {
                                            view: &self.msaa_view,
                                            resolve_target: Some(&view),
                                            ops: wgpu::Operations {
                                                load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                                                store: wgpu::StoreOp::Store,
                                            },
                                        },
                                    )],
                                    depth_stencil_attachment: None,
                                    timestamp_writes: None,
                                    occlusion_query_set: None,
                                });

                            // Clear shapes from previous frame
                            shape_renderer.clear();

                            // Call user's render function
                            let mut canvas = Canvas {
                                shapes: &mut shape_renderer,
                                text: &mut text_renderer,
                                width: (self.config.width as f64 / self.scale_factor) as f32,
                                height: (self.config.height as f64 / self.scale_factor) as f32,
                                scale_factor: self.scale_factor,
                            };
                            render_fn(&mut canvas);

                            // Render shapes
                            shape_renderer.render(&self.device, &self.queue, &mut pass);

                            // Render text
                            // text_renderer.render(&self.device, &self.queue, &mut pass);
                        }

                        self.queue.submit([encoder.finish()]);
                        frame.present();
                    }
                    WindowEvent::CloseRequested => {
                        target.exit();
                    }
                    _ => {}
                },
                _ => {}
            }
        });
    }
}
