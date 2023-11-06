// lib.rs

use winit::{
    event::*,
    window::  Window,
};

use crate::render::pipelines::polygon::{Vertex, PolygonPipeline};



pub struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    
    // The window must be declared after the surface so
    // it gets dropped after it as the surface contains
    // unsafe references to the window's resources.
    pub window: Window, 
    bg_color: wgpu::Color,
    render_pipeline: wgpu::RenderPipeline,
    render_pipeline_2: wgpu::RenderPipeline,
    toggle_pipeline: bool,
    vertex_buffer: wgpu::Buffer,
    
}
 

impl State {
    // Creating some of the wgpu types requires async code
    pub async fn new(window: Window) -> Self {
        let size = window.inner_size();


        let bg_color = wgpu::Color {
            r: 0.3,
            g: 0.2,
            b: 0.3,
            a: 1.0,
        };

        // The instance is a handle to our GPU
        // BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
        let wgpu_instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        });

        // # Safety
        //
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
                    // WebGL doesn't support all of wgpu's features, so if
                    // we're building for the web we'll have to disable some.
                    limits: if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                },
                // Some(&std::path::Path::new("trace")), // Trace path
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
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        let shader = device.create_shader_module(wgpu::include_wgsl!("../../assets/shaders/shader.wgsl"));

        let triangle_pipeline: PolygonPipeline = PolygonPipeline::new(&[
            Vertex { position: [0.0, 0.5, 0.0], color: [1.0, 0.0, 0.0] },
            Vertex { position: [-0.5, -0.5, 0.0], color: [0.0, 1.0, 0.0] },
            Vertex { position: [0.5, -0.5, 0.0], color: [0.0, 0.0, 1.0] },],
            &device,
            &shader,
            &config
        );

        let challenge_shader = device.create_shader_module(wgpu::include_wgsl!("../../assets/shaders/pipeline_challenge.wgsl"));

        let render_pipeline_2 = PolygonPipeline::new(&[
            Vertex { position: [0.0, 0.5, 0.0], color: [1.0, 0.0, 0.0] },
            Vertex { position: [-0.5, -0.5, 0.0], color: [0.0, 1.0, 0.0] },
            Vertex { position: [0.5, -0.5, 0.0], color: [0.0, 0.0, 1.0] },],
            &device,
            &challenge_shader,
            &config
        );

        let toggle_pipeline = true;

        

        Self {
            surface,
            device,
            queue,
            config,
            size,
            window,
            bg_color,
            render_pipeline:triangle_pipeline.pipeline,
            render_pipeline_2: render_pipeline_2.pipeline,
            toggle_pipeline,
            vertex_buffer: triangle_pipeline.vertex_buffer
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

            WindowEvent::CursorMoved {position, .. } => {
                let (width, height) = (self.size.width as f64, self.size.height as f64);

            // Normaliza las coordenadas del cursor a un valor entre 0 y 1
            let normalized_x = position.x / width;
            let normalized_y = position.y / height;
            let combined_xy = (position.x * position.y) / (width * height);

            // Asigna los valores normalizados a los canales de color
            // Asegúrate de que los valores estén entre 0 y 1 para que sean válidos para wgpu::Color
            self.bg_color = wgpu::Color {
                r: normalized_x.min(1.0).max(0.0),
                g: normalized_y.min(1.0).max(0.0),
                b: combined_xy.min(1.0).max(0.0),
                a: 1.0, // El canal alfa se mantiene constante
            };
            true
            },
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Space),
                        ..
                    },
                ..
            } => {

                self.toggle_pipeline = !self.toggle_pipeline;
                true

            },

            _ => {
                false
            }

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
                        load: wgpu::LoadOp::Clear(self.bg_color),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            if self.toggle_pipeline {
                render_pass.set_pipeline(&self.render_pipeline);
            }
            else {
                render_pass.set_pipeline(&self.render_pipeline_2);
            }
             // 2.
            render_pass.draw(0..3, 0..1); // 3.
        }

        
    
        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    
        Ok(())
    }
    
    

}
