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

        // // Bind group layouts describe data available to the GPU in different shader stages.
        // let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        //     label: None,
        //     entries: &[
        //         // Binding 0 is a uniform buffer used to hold a transformation matrix for the
        //         // vertex shader.
        //         wgpu::BindGroupLayoutEntry {
        //             binding: 0,
        //             visibility: wgpu::ShaderStages::VERTEX,
        //             ty: wgpu::BindingType::Buffer {
        //                 ty: wgpu::BufferBindingType::Uniform,
        //                 has_dynamic_offset: false,
        //                 min_binding_size: None,
        //             },
        //             count: None,
        //         },
        //         // Binding 1 holds a texture that is sampled by texture coordinates to produce the
        //         // appearance of a particular set of geometry in the fragment shader.
        //         wgpu::BindGroupLayoutEntry {
        //             binding: 1,
        //             visibility: wgpu::ShaderStages::FRAGMENT,
        //             ty: wgpu::BindingType::Texture {
        //                 multisampled: false,
        //                 sample_type: wgpu::TextureSampleType::Float { filterable: true },
        //                 view_dimension: wgpu::TextureViewDimension::D2,
        //             },
        //             count: None,
        //         },
        //         // Binding 2 holds a sampling algorithm used to define the behavior when sampling
        //         // the texture in the fragment shader.
        //         wgpu::BindGroupLayoutEntry {
        //             binding: 2,
        //             visibility: wgpu::ShaderStages::FRAGMENT,
        //             ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
        //             count: None,
        //         },
        //     ],
        // });

        // let render_format = wgpu::TextureFormat::Bgra8Unorm;
        // let config = wgpu::SurfaceConfiguration {
        //     usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        //     format: render_format,
        //     width: SIZE_X as u32,
        //     height: SIZE_Y as u32,
        //     present_mode: wgpu::PresentMode::Mailbox,
        // };
        // surface.configure(&device, &config);

        // // A multisampled framebuffer is used for anti-aliasing.
        // let multisampled_framebuffer =
        //     create_multisampled_framebuffer(&device, &config, MSAA_SAMPLES);

        // // The graphics pipeline specifies what behavior to use when rendering to the screen.
        // let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        //     label: None,
        //     bind_group_layouts: &[&bind_group_layout],
        //     push_constant_ranges: &[],
        // });
        // let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        //     label: None,
        //     layout: Some(&pipeline_layout),
        //     vertex: wgpu::VertexState {
        //         module: &vs_module,
        //         entry_point: "main",
        //         buffers: &[wgpu::VertexBufferLayout {
        //             array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
        //             step_mode: wgpu::VertexStepMode::Vertex,
        //             attributes: &wgpu::vertex_attr_array![0 => Float32x4, 1 => Float32x2],
        //         }],
        //     },
        //     fragment: None,
        //     primitive: wgpu::PrimitiveState {
        //         topology: wgpu::PrimitiveTopology::TriangleList,
        //         front_face: wgpu::FrontFace::Ccw,
        //         cull_mode: Some(wgpu::Face::Back),
        //         ..Default::default()
        //     },
        //     depth_stencil: None,
        //     multisample: wgpu::MultisampleState {
        //         count: MSAA_SAMPLES,
        //         mask: !0,
        //         alpha_to_coverage_enabled: false,
        //     },
        //     multiview: None,
        // });

        // let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
        //     address_mode_u: wgpu::AddressMode::ClampToEdge,
        //     address_mode_v: wgpu::AddressMode::ClampToEdge,
        //     address_mode_w: wgpu::AddressMode::ClampToEdge,
        //     mag_filter: wgpu::FilterMode::Nearest,
        //     min_filter: wgpu::FilterMode::Linear,
        //     mipmap_filter: wgpu::FilterMode::Nearest,
        //     ..Default::default()
        // });

        // // We render the background and pointer to the screen as two rectangles with various
        // // rotations. These index and vertex buffers describe a single rectangle split into two
        // // triangular fragments that is reused for both images.
        // let rectangle_vertex_buffer =
        //     device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        //         label: None,
        //         contents: &[
        //             Vertex::new(1., 1., 1., 0.),
        //             Vertex::new(-1., 1., 0., 0.),
        //             Vertex::new(-1., -1., 0., 1.),
        //             Vertex::new(1., -1., 1., 1.),
        //         ]
        //         .as_bytes(),
        //         usage: wgpu::BufferUsages::VERTEX,
        //     });
        // let rectangle_index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        //     label: None,
        //     contents: &[0u32, 1, 2, 2, 3, 0].as_bytes(),
        //     usage: wgpu::BufferUsages::INDEX,
        // });

        // // Font rendering is conveniently handled by `wgpu_glyph` :)
        // let fonts: Vec<wgpu_glyph::ab_glyph::FontArc> =
        //     vec![wgpu_glyph::ab_glyph::FontArc::try_from_slice(FONT).unwrap()];
        // let text_renderer = GlyphBrushBuilder::using_fonts(fonts).build(&device, render_format);

        Self {
            device,
            queue,
            surface,
        }
    }

    pub fn draw_frame(&mut self) {}
}
