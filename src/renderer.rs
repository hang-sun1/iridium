use std::{rc::Rc, time::{Instant, Duration}};

use crate::{camera::{Camera, Movement}, BoidInstance};
use glam::{Vec3, Mat4, Vec4};
use wgpu::{include_wgsl, util::DeviceExt, BufferDescriptor, BufferAddress};
use winit::{window::Window, event::{WindowEvent, Event, KeyboardInput, ElementState, VirtualKeyCode}};

#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: Vec4,
    color: Vec4,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct MatrixData {
    mvp: Mat4,
}

pub struct Renderer {
    device: wgpu::Device,
    surface: wgpu::Surface,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    instance_buffer: wgpu::Buffer,
    imgui_renderer: crate::imgui::Imgui,
    camera: Camera,
    matrix_data: wgpu::Buffer,
    matrix_bind_group: wgpu::BindGroup,
    time: Instant,
}

impl Renderer {
    pub async fn new(window: Rc<Window>) -> Self {
        let size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::Backends::METAL);
        let surface = unsafe { instance.create_surface(&window.as_ref()) };
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }).await.expect("could not find an adequate adapter");

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default(),
                label: None,
            },
            None,
        ).await.expect("could not find a suitable device");

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Immediate,
        };
        surface.configure(&device, &config);

        let vert_shader = device.create_shader_module(&include_wgsl!("shaders/vert.wgsl"));
        let frag_shader = device.create_shader_module(&include_wgsl!("shaders/frag.wgsl"));

        let vertex_layout = wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute { format: wgpu::VertexFormat::Float32x4, offset: 0, shader_location: 0 },
                wgpu::VertexAttribute { format: wgpu::VertexFormat::Float32x4, offset: 16, shader_location: 1 },
            ],
        };

        let instance_layout = wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<BoidInstance>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: 16 as wgpu::BufferAddress,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: 32 as wgpu::BufferAddress,
                    shader_location: 4,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: 48 as wgpu::BufferAddress,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ]
        };
        
        let camera = Camera::new(
            Vec3::new(0.0, 0.0, 0.5),
            Vec3::ZERO,
            90.0,
            0.01,
            100.0,
            config.width as f32 / config.height as f32
        );

        // let halve_size = Mat4::from_scale(Vec3::new(1.5, 1.5, 1.5));
        let halve_size = Mat4::IDENTITY;
        // let camera_uniform = UniformData { mvp: halve_size.to_cols_array_2d() };
        let camera_uniform = MatrixData { mvp: camera.model_view_proj(halve_size) };

        let uniform_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("uniform buff"),
                contents: bytemuck::cast_slice(&[camera_uniform]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );

        let uniform_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: None },
                    count: None,
                },
            ],
            label: Some("stuff"),
        });

        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &uniform_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform_buffer.as_entire_binding(),
                }
            ],
            label: Some("other stuff"),
        });
        
        let render_pipeline_layout = device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                label: Some("graphics pipeline descriptor"),
                bind_group_layouts: &[&uniform_bind_group_layout],
                push_constant_ranges: &[],
            },
        );

        let render_pipeline = Self::create_render_pipeline(
            &device,
            vertex_layout,
            Some(instance_layout),
            &vert_shader,
            &frag_shader,
            render_pipeline_layout,
        );

        let vertices: &[Vertex] = &[
            Vertex { position: Vec4::new(0.0868241, 0.49240386, 0.0, 0.0), color: Vec4::new(0.5, 0.0, 0.5, 0.0) }, // A
            Vertex { position: Vec4::new(-0.49513406, 0.06958647, 0.0, 0.0), color: Vec4::new(0.5, 0.0, 0.5, 0.0) }, // B
            Vertex { position: Vec4::new(0.21918549, -0.44939706, 0.0, 0.0), color: Vec4::new(0.5, 0.0, 0.5, 0.0) }, // C
        ];
        let indices: &[u16] = &[
            0, 1, 2,
        ];

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buff"),
            contents: bytemuck::cast_slice(vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buff"),
            contents: bytemuck::cast_slice(indices),
            usage: wgpu::BufferUsages::INDEX,
        });
        
        let instance_buffer = device.create_buffer(&BufferDescriptor { 
            label: Some("hello"), 
            size: std::mem::size_of::<BoidInstance>() as BufferAddress, 
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
         });

        let imgui_renderer = crate::imgui::Imgui::new(window.clone(), &device, &queue, &config);
        
        Self {
            device,
            surface,
            queue,
            config,
            size,
            render_pipeline,
            vertex_buffer,
            instance_buffer,
            index_buffer,
            imgui_renderer,
            camera,
            matrix_data: uniform_buffer,
            matrix_bind_group: uniform_bind_group,
            time: Instant::now(),
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn input(&mut self, _win_event: &WindowEvent, _event: &Event<()>) -> bool {
        let mut movement = Movement::default();
        
        match _win_event {
            WindowEvent::KeyboardInput {
                input: KeyboardInput {
                    state,
                    virtual_keycode: Some(keycode),
                    ..
                },
                ..
            } => {
                let is_pressed = *state == ElementState::Pressed;
                match keycode {
                    VirtualKeyCode::W => {
                        movement.strafe_forward = true;
                    },
                    VirtualKeyCode::S => {
                        movement.strafe_back = true;
                    },
                    VirtualKeyCode::A => {
                        movement.strafe_left = true;
                    },
                    VirtualKeyCode::D => {
                        movement.strafe_right = true;
                    },
                    _ => (),
                }
            }
            
            _ => ()
            
        };
        self.camera.add_movement(movement);
        self.imgui_renderer.event(_event);
        false
    }

    pub(crate) fn fill_instance_buffer(&mut self, instance_data: &[BoidInstance]) {
        self.queue.write_buffer(&self.instance_buffer, 0, bytemuck::cast_slice(instance_data));
        self.queue.submit(std::iter::empty());
    }

    pub fn render(&mut self, delta_t: Duration) -> Result<(), wgpu::SurfaceError> {
        self.camera.update(delta_t);
        let swapchain_image = self.surface.get_current_texture()?;
        let swapchain_imageview = swapchain_image.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let color_attachment = wgpu::RenderPassColorAttachment {
            view: &swapchain_imageview,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                store: true,
            },
        };

        let mvp = MatrixData {
            mvp: self.camera.model_view_proj(Mat4::IDENTITY),
        };
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
        self.queue.write_buffer(&self.matrix_data, 0, bytemuck::cast_slice(&[mvp]));

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("main pass"),
            color_attachments: &[color_attachment],
            depth_stencil_attachment: None,
        });

        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.matrix_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..3, 0, 0..1);
        // render_pass.draw(0..3, 0..1);

        self.imgui_renderer.render_ui(&self.device, &self.queue, &mut render_pass);
        drop(render_pass);

        self.queue.submit(std::iter::once(encoder.finish()));
        swapchain_image.present();
        Ok(())
    }

    fn create_render_pipeline(device: &wgpu::Device, vert_layout: wgpu::VertexBufferLayout, inst_layout: Option<wgpu::VertexBufferLayout>, vert: &wgpu::ShaderModule, 
        frag: &wgpu::ShaderModule, layout: wgpu::PipelineLayout) -> wgpu::RenderPipeline {
            
        let buff = if let Some(layout) = inst_layout { vec![vert_layout, layout] } else { vec![vert_layout] };
        let render_pipeline = device.create_render_pipeline(
            &wgpu::RenderPipelineDescriptor {
                label: Some("graphics pipeline"),
                layout: Some(&layout),
                vertex: wgpu::VertexState {
                    module: &vert,
                    entry_point: "main",
                    buffers: &buff,
                },
                fragment: Some(wgpu::FragmentState {
                    module: &frag,
                    entry_point: "main",
                    targets: &[wgpu::ColorTargetState {
                        format: wgpu::TextureFormat::Bgra8UnormSrgb,
                        blend: Some(wgpu::BlendState::REPLACE),
                        write_mask: wgpu::ColorWrites::ALL,
                    }],
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: Some(wgpu::Face::Back),
                    polygon_mode: wgpu::PolygonMode::Fill,
                    unclipped_depth: false,
                    conservative: false,
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                multiview: None,
            }
        );
        render_pipeline
    }


    pub(crate) fn camera(&self) -> &Camera {
        &self.camera
    }
}