
use super::super::{Vertex as VertexTrait, buffer::Buffer, mesh::Mesh};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
}

impl Vertex {

    const ATTRIBS: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x2];


    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,

        }
    }
}

impl VertexTrait for Vertex {
    const QUADS_INDEX: Option<wgpu::IndexFormat> = Some(wgpu::IndexFormat::Uint16);
    const STRIDE: wgpu::BufferAddress = std::mem::size_of::<Self>() as wgpu::BufferAddress;
}

// pub struct FigureVerts(Buffer<Vertex>);
// //pub struct SpriteVerts(Texture);

// pub(in super::super) fn create_verts_buffer(
//     device: &wgpu::Device,
//     mesh: Mesh<Vertex>,
// ) -> FigureVerts {
//     // TODO: type Buffer by wgpu::BufferUsage
//     FigureVerts(Buffer::new(
//         device,
//         wgpu::BufferUsages::VERTEX,
//         mesh.vertices(),
//     ))
// }






pub struct FigureVerts {
    pub vertex_buffer: Buffer<Vertex>,
    pub num_vertices: u32,
    // Si estás usando indexado, también incluye:
    // pub index_buffer: wgpu::Buffer,
    // pub num_indices: u32,
}

pub(in super::super) fn create_verts_buffer(
    device: &wgpu::Device,
    mesh: Mesh<Vertex>,
) -> FigureVerts {
    let vertex_buffer = Buffer::new(
        device,
        wgpu::BufferUsages::VERTEX,
        mesh.vertices(),
    );

    let num_vertices = mesh.vertices().len() as u32;

    // Si estás usando indexado, crea el index_buffer aquí

    FigureVerts {
        vertex_buffer,
        num_vertices,
        // Si estás usando indexado, también asigna index_buffer y num_indices
    }
}









pub struct FigureLayout {
    pub layout: wgpu::BindGroupLayout,
}


impl FigureLayout {
    pub fn new(device: &wgpu::Device) -> Self {
        Self {
            layout: device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        // This should match the filterable field of the
                        // corresponding Texture entry above.
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            }),
        }
    }
}

pub struct FigurePipeline {
    pub pipeline: wgpu::RenderPipeline,
    // pub vertex_buffer: wgpu::Buffer,
    // pub index_buffer: wgpu::Buffer,
    // pub num_indices: u32,
}

impl FigurePipeline {
    pub fn new(
        // vertices: &[Vertex],
        // indices: &[u16],
        device: &wgpu::Device,
        shader: &wgpu::ShaderModule,
        config: &wgpu::SurfaceConfiguration,
        layout: &FigureLayout
    ) -> Self {

        // let vertex_buffer = device.create_buffer_init(
        //     &wgpu::util::BufferInitDescriptor {
        //         label: Some("Vertex Buffer"),
        //         contents: bytemuck::cast_slice(vertices),
        //         usage: wgpu::BufferUsages::VERTEX,
        //     }
        // );

        // let index_buffer = device.create_buffer_init(
        //     &wgpu::util::BufferInitDescriptor {
        //         label: Some("Index Buffer"),
        //         contents: bytemuck::cast_slice(indices),
        //         usage: wgpu::BufferUsages::INDEX,
        //     }
        // );


        let pipeline_layout =
        device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Figure Pipeline Layout"),
            bind_group_layouts: &[&layout.layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Figure Pipeline"),
            layout: Some(&pipeline_layout),
            primitive: wgpu::PrimitiveState { 
                topology: wgpu::PrimitiveTopology::TriangleList, // 1.
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw, // 2.
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main", // 1.
                buffers: &[
                    Vertex::desc()
                ], // 2.
            },
            fragment: Some(wgpu::FragmentState { // 3.
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState { // 4.
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            depth_stencil: None, // 1.
            multisample: wgpu::MultisampleState {
                count: 1, // 2.
                mask: !0, // 3.
                alpha_to_coverage_enabled: false, // 4.
            },
            multiview: None, // 5.
        });

        
        Self {
            pipeline,
            // vertex_buffer,
            // index_buffer,
            // num_indices: indices.len() as u32,
        }
    }
}