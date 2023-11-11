// lib.rs

use winit::{
    event::*,
    window::  Window,
};

use crate::render::pipelines::figure::{Vertex, FigurePipeline};



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

        let shader = device.create_shader_module(wgpu::include_wgsl!("../../assets/shaders/shader.wgsl"));

        const PENTAGON_VERTICES: &[Vertex] = &[
            Vertex { position: [-0.0868241, 0.49240386, 0.0], color: [0.0, 0.5, 0.5] }, // A
            Vertex { position: [-0.49513406, 0.06958647, 0.0], color: [0.5, 0.0, 0.5] }, // B
            Vertex { position: [-0.21918549, -0.44939706, 0.0], color: [0.5, 0.0, 0.0] }, // C
            Vertex { position: [0.35966998, -0.3473291, 0.0], color: [0.5, 0.5, 0.5] }, // D
            Vertex { position: [0.44147372, 0.2347359, 0.0], color: [0.0, 0.0, 0.5] }, // E
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
            &config
        );

        const OCTAGON_VERTICES: &[Vertex] = &[
            Vertex { position: [0.0, 0.5, 0.0], color: [0.0, 0.5, 0.5] }, // A
            Vertex { position: [-0.3536, 0.3536, 0.0], color: [0.5, 0.0, 0.5] }, // B
            Vertex { position: [-0.5, 0.0, 0.0], color: [0.5, 0.0, 0.0] }, // C
            Vertex { position: [-0.3536, -0.3536, 0.0], color: [0.5, 0.5, 0.5] }, // D
            Vertex { position: [0.0, -0.5, 0.0], color: [0.0, 0.0, 0.5] }, // E
            Vertex { position: [0.3536, -0.3536, 0.0], color: [0.0, 0.5, 0.0] }, // F
            Vertex { position: [0.5, 0.0, 0.0], color: [0.5, 0.0, 0.0] }, // G
            Vertex { position: [0.3536, 0.3536, 0.0], color: [0.0, 0.0, 0.5] }, // H
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
            &config
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
