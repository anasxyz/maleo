// src/gpu.rs - All wgpu/GPU related stuff

use std::sync::Arc;
use wgpu;
use winit::window::Window;

/// GPU context - handles all wgpu resources
pub struct GpuContext {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub surface: wgpu::Surface<'static>,
    pub config: wgpu::SurfaceConfiguration,
    pub format: wgpu::TextureFormat,
    pub msaa_texture: wgpu::Texture,
}

impl GpuContext {
    /// Create a new GPU context for a window
    pub async fn new(window: Arc<Window>) -> Self {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Main Device"),
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        let format = surface_caps.formats[0];

        let size = window.inner_size();
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

        let msaa_texture = Self::create_msaa_texture(&device, &config, format);

        Self {
            device,
            queue,
            surface,
            config,
            format,
            msaa_texture,
        }
    }

    /// Resize the surface and MSAA texture
    pub fn resize(&mut self, width: u32, height: u32) {
        if width == 0 || height == 0 {
            return;
        }

        self.config.width = width;
        self.config.height = height;
        self.surface.configure(&self.device, &self.config);

        self.msaa_texture = Self::create_msaa_texture(&self.device, &self.config, self.format);
    }

    /// Create MSAA texture for anti-aliasing
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

    /// Begin a render pass
    pub fn begin_frame(&mut self) -> Result<RenderFrame, wgpu::SurfaceError> {
        let frame = self.surface.get_current_texture()?;
        let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let msaa_view = self.msaa_texture.create_view(&wgpu::TextureViewDescriptor::default());
        let encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        Ok(RenderFrame {
            frame,
            view,
            encoder,
            msaa_view,
        })
    }
}

/// A single render frame - owns everything needed for one frame
pub struct RenderFrame {
    frame: wgpu::SurfaceTexture,
    view: wgpu::TextureView,
    encoder: wgpu::CommandEncoder,
    msaa_view: wgpu::TextureView,
}

impl RenderFrame {
    /// Begin a render pass - consumes self and returns encoder + finisher
    pub fn begin(mut self) -> (wgpu::CommandEncoder, FrameFinisher, wgpu::TextureView, wgpu::TextureView) {
        (
            self.encoder,
            FrameFinisher { frame: self.frame },
            self.view,
            self.msaa_view,
        )
    }
}

/// Used to finish and present a frame after rendering
pub struct FrameFinisher {
    frame: wgpu::SurfaceTexture,
}

impl FrameFinisher {
    /// Finish rendering and present the frame
    pub fn present(self, encoder: wgpu::CommandEncoder, queue: &wgpu::Queue) {
        queue.submit(Some(encoder.finish()));
        self.frame.present();
    }
}
