pub mod renderer;
pub mod pipelines;
pub mod texture;
pub mod mesh;
pub mod model;
pub mod buffer;


pub trait Vertex: Clone + bytemuck::Pod {
    const STRIDE: wgpu::BufferAddress;
    // Whether these types of verts use the quad index buffer for drawing them
    const QUADS_INDEX: Option<wgpu::IndexFormat>;
}