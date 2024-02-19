use super::{
    block::BlockVertex, buffer::Buffer, mesh::Mesh, Vertex
};
/// Represents a mesh that has been sent to the GPU.
pub struct Model{
    vbuf: Buffer<BlockVertex>,
    ibuf: Buffer<u16>,
    pub num_indices: u32,
}

impl Model{
    pub fn new(device: &wgpu::Device, mesh: &Mesh) -> Option<Self> {
        if mesh.vertices().is_empty() || mesh.indices().is_empty() {
            return None;
        }

        let vbuf = Buffer::new(device, wgpu::BufferUsages::VERTEX, mesh.vertices());
        let ibuf = Buffer::new(device, wgpu::BufferUsages::INDEX, mesh.indices());

        Some(Self {
            vbuf,
            ibuf,
            num_indices: mesh.indices().len() as u32,
        })
    }
    pub(super) fn vbuf(&self) -> &wgpu::Buffer { &self.vbuf.buff }
    pub(super) fn ibuf(&self) -> &wgpu::Buffer { &self.ibuf.buff }
    pub fn len(&self) -> u32 { self.vbuf.len() as u32}
}

