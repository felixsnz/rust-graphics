// lib.rs

use winit::{
    event::*,
    window::  Window,
};

use crate::render::{
    pipelines::figure::{Vertex, FigurePipeline, FigureLayout},
    texture
};




/// State gestiona los recursos de renderizado de la aplicación,
/// actualmente para un triángulo. Con la expansión del proyecto,
/// se podría renombrar a Renderer y crear un GlobalState para un
/// alcance más amplio.
pub struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub window: Window,
    toggle_pipeline: bool, 
    render_pipeline: FigurePipeline,
    render_pipeline2:FigurePipeline,
    diffuse_bind_group: wgpu::BindGroup,
    diffuse_texture: texture::Texture, //for later usage
    
}
 

impl State {
    pub async fn new(window: Window) -> Self {
        let size = window.inner_size();

        // The instance is a handle to our GPU
        // BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
        let wgpu_instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        });

        // # Safety
        // The surface needs to live as long as the window that created it.
        // State owns the window so this should be safe.
        let surface = unsafe { wgpu_instance.create_surface(&window) }.unwrap();

        let adapter = wgpu_instance
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
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .unwrap();
        
        let surface_caps = surface.get_capabilities(&adapter);
        // Shader code in this tutorial assumes an Srgb surface texture. Using a different
        // one will result all the colors comming out darker. If you want to support non
        // Srgb surfaces, you'll need to account for that when drawing to the frame.
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,//investigar cual es la diferencia entre esto y usar surface_caps.usages
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);


        let diffuse_bytes = include_bytes!("../../assets/images/happy-tree.png");

        let diffuse_texture = texture::Texture::from_bytes(&device, &queue, diffuse_bytes, "happy-tree.png").unwrap();


        let figure_bind_group_layout = FigureLayout::new(&device);


            let diffuse_bind_group = device.create_bind_group(
                &wgpu::BindGroupDescriptor {
                    layout: &figure_bind_group_layout.layout,
                    entries: &[
                        wgpu::BindGroupEntry {
                            binding: 0,
                            resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
                        },
                        wgpu::BindGroupEntry {
                            binding: 1,
                            resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
                        }
                    ],
                    label: Some("diffuse_bind_group"),
                }
            );

        let shader = device.create_shader_module(wgpu::include_wgsl!("../../assets/shaders/shader.wgsl"));

        const PENTAGON_VERTICES: &[Vertex] = &[
            // Changed
            Vertex { position: [-0.0868241, 0.49240386, 0.0], tex_coords: [0.4131759, 0.00759614], }, // A
            Vertex { position: [-0.49513406, 0.06958647, 0.0], tex_coords: [0.0048659444, 0.43041354], }, // B
            Vertex { position: [-0.21918549, -0.44939706, 0.0], tex_coords: [0.28081453, 0.949397], }, // C
            Vertex { position: [0.35966998, -0.3473291, 0.0], tex_coords: [0.85967, 0.84732914], }, // D
            Vertex { position: [0.44147372, 0.2347359, 0.0], tex_coords: [0.9414737, 0.2652641], }, // E
        ];

        const PENTAGON_INDICES: &[u16] = &[
            0, 1, 4,
            1, 2, 4,
            2, 3, 4,
        ];

        let pentagon_pipeline: FigurePipeline = FigurePipeline::new(
            PENTAGON_VERTICES,
            PENTAGON_INDICES,
            &device,
            &shader,
            &config,
            &figure_bind_group_layout
        );

        const OCTAGON_VERTICES: &[Vertex] = &[
            // Changed
            Vertex { position: [-0.0868241, 0.49240386, 0.0], tex_coords: [0.4131759, 0.00759614], }, // A
            Vertex { position: [-0.49513406, 0.06958647, 0.0], tex_coords: [0.0048659444, 0.43041354], }, // B
            Vertex { position: [-0.21918549, -0.44939706, 0.0], tex_coords: [0.28081453, 0.949397], }, // C
            Vertex { position: [0.35966998, -0.3473291, 0.0], tex_coords: [0.85967, 0.84732914], }, // D
            Vertex { position: [0.44147372, 0.2347359, 0.0], tex_coords: [0.9414737, 0.2652641], }, // E
        ];

        const OCTAGON_INDICES: &[u16] = &[
            0, 1, 7,
            1, 2, 3,
            1, 3, 7,
            3, 4, 5,
            3, 5, 7,
            5, 6, 7,
        ];

        let complex_pipeline: FigurePipeline = FigurePipeline::new(
            OCTAGON_VERTICES,
            OCTAGON_INDICES,
            &device,
            &shader,
            &config,
            &figure_bind_group_layout,

        );


        Self {
            surface,
            device,
            queue,
            config,
            size,
            window,
            toggle_pipeline:true,
            render_pipeline:pentagon_pipeline,
            render_pipeline2:complex_pipeline,
            diffuse_bind_group,
            diffuse_texture,
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {

        match event {
            WindowEvent::KeyboardInput {
                input:
                KeyboardInput {
                    state,
                    virtual_keycode: Some(VirtualKeyCode::Space),
                    ..
                },
                ..
            } => {
                self.toggle_pipeline = *state == ElementState::Pressed;
                true
            }
            _ => false
        }
    }

    pub fn update(&mut self) {
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });
        {
            
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color { 
                            r: 0.5,
                            g: 0.5,
                            b: 1.0,
                            a: 1.0
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            let selected_pipeline;

            if self.toggle_pipeline {
                selected_pipeline = &self.render_pipeline;
            }
            else {
                selected_pipeline = &self.render_pipeline2;
            }

            




            render_pass.set_pipeline(&selected_pipeline.pipeline);
            render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
            render_pass.set_vertex_buffer(0, selected_pipeline.vertex_buffer.slice(..));
            render_pass.set_index_buffer(selected_pipeline.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..selected_pipeline.num_indices, 0, 0..1); // pendiente crear una forma para determinar automaticamente los vertices (sin agregar los vertices al state)
        }
        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        Ok(())
    }
}
