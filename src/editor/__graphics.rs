pub(super) struct Renderer {
    device: wgpu::Device,
    surface: wgpu::Surface,
    queue: wgpu::Queue,
    // multisampled_framebuffer: wgpu::TextureView,

    // text_renderer: GlyphBrush<()>,
    // /// Required by `wgpu_glyph`
    // local_pool: futures::executor::LocalPool,
    // /// Required by `wgpu_glyph`
    // staging_belt: wgpu::util::StagingBelt,

    // pipeline: wgpu::RenderPipeline,
    // rectangle_index_buffer: wgpu::Buffer,
    // rectangle_vertex_buffer: wgpu::Buffer,
}

impl Renderer {
    /// Creates a new `Renderer` by initializing the GPU to prepare it for rendering.
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

        Self {
            device,
            queue,
            surface,
        }
    }

    pub fn draw_frame(&mut self) -> Result<(), wgpu::SurfaceError> {
        let res = self.surface.get_current_texture();

        if let Err(e) = res {}

        //let output = self.surface.get_current_texture()?;
        // let view = output
        //     .texture
        //     .create_view(&wgpu::TextureViewDescriptor::default());

        // let mut encoder = self
        //     .device
        //     .create_command_encoder(&wgpu::CommandEncoderDescriptor {
        //         label: Some("main"),
        //     });

        // let render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        //     label: Some("clear screen"),
        //     color_attachments: &[wgpu::RenderPassColorAttachment {
        //         view: &view,
        //         resolve_target: None,
        //         ops: wgpu::Operations {
        //             load: wgpu::LoadOp::Clear(wgpu::Color {
        //                 r: 0.1,
        //                 g: 0.2,
        //                 b: 0.3,
        //                 a: 1.0,
        //             }),
        //             store: true,
        //         },
        //     }],
        //     depth_stencil_attachment: None,
        // });

        // drop(render_pass);

        // self.queue.submit(std::iter::once(encoder.finish()));
        // output.present();

        Ok(())
    }
}
