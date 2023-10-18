use super::WINDOW_DIMENSIONS;

pub struct VoiceEyeRenderer {
    device: wgpu::Device,
    surface: wgpu::Surface,
    queue: wgpu::Queue,
}

impl VoiceEyeRenderer {
    pub fn new<W: raw_window_handle::HasRawWindowHandle>(handle: W) -> Self {
        let instance = wgpu::Instance::new(wgpu::Backends::PRIMARY);

        let surface = unsafe { instance.create_surface(&handle) };

        let (device, queue) = futures::executor::block_on(async {
            let adapter = instance
                .request_adapter(&wgpu::RequestAdapterOptions {
                    power_preference: wgpu::PowerPreference::HighPerformance,
                    force_fallback_adapter: false,
                    compatible_surface: Some(&surface),
                })
                .await
                .unwrap();

            adapter
                .request_device(
                    &wgpu::DeviceDescriptor {
                        label: None,
                        features: wgpu::Features::empty(),
                        limits: wgpu::Limits::default(),
                    },
                    None,
                )
                .await
                .unwrap()
        });

        let render_format = wgpu::TextureFormat::Bgra8Unorm;
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: render_format,
            width: (WINDOW_DIMENSIONS.0 * 2) as u32,
            height: (WINDOW_DIMENSIONS.1 * 2) as u32,
            present_mode: wgpu::PresentMode::Mailbox,
        };
        surface.configure(&device, &config);

        Self {
            device,
            queue,
            surface,
        }
    }

    pub fn draw_frame(&mut self) {
        if let Ok(frame) = self.surface.get_current_texture() {
            let view: wgpu::TextureView = frame
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default());

            let mut encoder = self
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

            let render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("clear screen"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 1.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0,
                        }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });

            drop(render_pass);

            self.queue.submit(std::iter::once(encoder.finish()));
            frame.present();
        }
    }
}
