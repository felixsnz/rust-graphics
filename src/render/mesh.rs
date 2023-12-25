use super::Vertex;

#[derive(Clone)]

/// Represents a vec-based mesh on the CPU
pub struct Mesh<V: Vertex> {
    verts: Vec<V>,
    indices: Vec<u16>
}

impl<V: Vertex> Mesh<V> {
    /// Create a new `Mesh`.
    pub fn new() -> Self { Self { verts: Vec::new(), indices: Vec::new() } }

    /// Clear vertices, allows reusing allocated memory of the underlying Vec.
    pub fn clear(&mut self) { self.verts.clear(); }

    /// Get a slice referencing the vertices of this mesh.
    pub fn vertices(&self) -> &[V] { &self.verts }

    pub fn push(&mut self, vert: V) { self.verts.push(vert); }

    // new method to add indices
    pub fn push_indices(&mut self, indices: &[u16]) {
        self.indices.extend_from_slice(indices);
    }

    // returns the indices
    pub fn indices(&self) -> &[u16] {
        &self.indices
    }

    pub fn iter_verts(&self) -> std::slice::Iter<V> { self.verts.iter() }

    pub fn iter_indices(&self) -> std::vec::IntoIter<u16> { self.indices.clone().into_iter() }

    /// Push a new polygon onto the end of this mesh.
    pub fn push_tri(&mut self, tri: Tri<V>) {

        let start_index = self.verts.len() as u16;
        self.verts.push(tri.a);
        self.verts.push(tri.b);
        self.verts.push(tri.c);

        self.indices.push(start_index);
        self.indices.push(start_index + 1);
        self.indices.push(start_index + 2);

    }

    /// Push a new quad onto the end of this mesh.
    pub fn push_quad(&mut self, quad: Quad<V>) {
        let start_index = self.verts.len() as u16;
        // A quad is composed of two triangles. The code below converts the former to
        // the latter.

        self.verts.push(quad.a);
        self.verts.push(quad.b);
        self.verts.push(quad.c);
        self.verts.push(quad.d);


        // triange 1
        self.indices.push(start_index);     // a
        self.indices.push(start_index + 1); // b
        self.indices.push(start_index + 2); // c

        // triangle 2
        self.indices.push(start_index + 2); // a
        self.indices.push(start_index + 3); // c
        self.indices.push(start_index );    // d
    
    }

    // Método para añadir un cubo al mesh.
    pub fn push_cube(&mut self, cube: Cube<V>) {
        let start_index = self.verts.len() as u16;
    
        // Añadir los 8 vértices del cubo.
        self.verts.extend_from_slice(&[
            cube.a, cube.b, cube.c, cube.d, // base inferior
            cube.e, cube.f, cube.g, cube.h, // base superior
        ]);
    
        // Añadir índices para las 12 triángulos (6 caras * 2 triángulos por cara).
        // Asegúrate de que el orden de los vértices para cada triángulo sea coherente y orientado hacia afuera.
        let indices = [
        // Base inferior (vista desde arriba para orientación hacia afuera)
        start_index, start_index + 2, start_index + 1,
        start_index, start_index + 3, start_index + 2,

        // Base superior (vista desde abajo para orientación hacia afuera)
        start_index + 4, start_index + 5, start_index + 6,
        start_index + 4, start_index + 6, start_index + 7,

        // Lado frontal
        start_index + 1, start_index + 5, start_index,
        start_index + 5, start_index + 4, start_index,

        // Lado derecho
        start_index + 2, start_index + 6, start_index + 1,
        start_index + 6, start_index + 5, start_index + 1,

        // Lado trasero
        start_index + 3, start_index + 7, start_index + 2,
        start_index + 7, start_index + 6, start_index + 2,

        // Lado izquierdo
        start_index, start_index + 4, start_index + 3,
        start_index + 4, start_index + 7, start_index + 3,
    ];
        self.indices.extend_from_slice(&indices);
    }
    

    
}


// Definición de la estructura Cube.
pub struct Cube<V: Vertex> {
    pub a: V, pub b: V, pub c: V, pub d: V, // Vértices de la base inferior
    pub e: V, pub f: V, pub g: V, pub h: V, // Vértices de la base superior
}

impl<V: Vertex> Cube<V> {
    pub fn new(a: V, b: V, c: V, d: V, e: V, f: V, g: V, h: V) -> Self {
        Self { a, b, c, d, e, f, g, h }
    }
}



/// Represents a triangle stored on the CPU.
pub struct Tri<V: Vertex> {
    pub a: V,
    pub b: V,
    pub c: V,
}

impl<V: Vertex> Tri<V> {
    pub fn new(a: V, b: V, c: V) -> Self { Self { a, b, c } }
}

/// Represents a quad stored on the CPU.
pub struct Quad<V: Vertex> {
    pub a: V,
    pub b: V,
    pub c: V,
    pub d: V,
}

impl<V: Vertex> Quad<V> {
    pub fn new(a: V, b: V, c: V, d: V) -> Self { Self { a, b, c, d } }

}