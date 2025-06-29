//! GPU-ready mesh loader for **glTF 2.0**
//!
//! Loads the first mesh/primitive found in a .gltf/.glb file.

use anyhow::{Context, Result};
use glium::{backend::Facade, implement_vertex, IndexBuffer, VertexBuffer};
use glium::index::PrimitiveType;
use gltf::mesh::util::ReadIndices;
use std::{fmt::Debug, path::Path};

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal:   [f32; 3],
}
implement_vertex!(Vertex, position, normal);

pub struct Mesh {
    pub vbuf: VertexBuffer<Vertex>,
    pub ibuf: IndexBuffer<u32>,
}

/// Load a glTF 2.0 file from disk and upload the first primitive to the GPU.
pub fn load_gltf<P, F>(path: P, facade: &F) -> Result<Mesh>
where
    P: AsRef<Path> + Debug,   // `gltf::import` wants Debug for diagnostics :contentReference[oaicite:3]{index=3}
    F: Facade + ?Sized,
{
    // -- parse the asset & bring buffer blobs into memory --
    let (doc, buffers, _images) =
        gltf::import(path.as_ref()).context("failed to import glTF file")?;   // :contentReference[oaicite:4]{index=4}

    // -- grab the very first mesh / primitive --
    let mesh      = doc.meshes().next().context("glTF has no meshes")?;
    let primitive = mesh.primitives().next().context("mesh has no primitives")?;

    // -- read vertex and index streams using the util::Reader helper --
    let reader = primitive.reader(|buf| Some(&buffers[buf.index()].0));        // Reader pattern :contentReference[oaicite:5]{index=5}

    let positions : Vec<[f32; 3]> = reader
        .read_positions()
        .context("primitive is missing POSITION attribute")?                  // POSITION is mandatory :contentReference[oaicite:6]{index=6}
        .collect();

    let normals   : Vec<[f32; 3]> = reader
        .read_normals()
        .context("primitive is missing NORMAL attribute")?
        .collect();

    let indices   : Vec<u32> = reader
        .read_indices()
        .context("primitive has no indices")?
        .into_u32()
        .collect();                                                           // ReadIndices enum :contentReference[oaicite:7]{index=7}

    // -- interleave into our engine's Vertex struct --
    let vertices: Vec<Vertex> = positions
        .into_iter()
        .zip(normals.into_iter())
        .map(|(p, n)| Vertex { position: p, normal: n })
        .collect();

    // -- immutable GPU buffers (fast path in glium) --
    let vbuf = VertexBuffer::immutable(facade, &vertices)?;                   // Immutable VBO :contentReference[oaicite:8]{index=8}
    let ibuf = IndexBuffer ::immutable(facade, PrimitiveType::TrianglesList, &indices)?;

    Ok(Mesh { vbuf, ibuf })
}

/// Create a unit cube (edge length = 2) with per-face normals.
pub fn cube<F>(facade: &F) -> Result<Mesh>
where
    F: Facade + ?Sized,
{
    // 24 unique vertices (4 per face) so that each face has a flat normal.
    let vertices: [Vertex; 24] = [
        // Front (+Z)
        Vertex { position: [-1.0, -1.0,  1.0], normal: [ 0.0,  0.0,  1.0] },
        Vertex { position: [ 1.0, -1.0,  1.0], normal: [ 0.0,  0.0,  1.0] },
        Vertex { position: [ 1.0,  1.0,  1.0], normal: [ 0.0,  0.0,  1.0] },
        Vertex { position: [-1.0,  1.0,  1.0], normal: [ 0.0,  0.0,  1.0] },

        // Back (-Z)
        Vertex { position: [ 1.0, -1.0, -1.0], normal: [ 0.0,  0.0, -1.0] },
        Vertex { position: [-1.0, -1.0, -1.0], normal: [ 0.0,  0.0, -1.0] },
        Vertex { position: [-1.0,  1.0, -1.0], normal: [ 0.0,  0.0, -1.0] },
        Vertex { position: [ 1.0,  1.0, -1.0], normal: [ 0.0,  0.0, -1.0] },

        // Left (-X)
        Vertex { position: [-1.0, -1.0, -1.0], normal: [-1.0,  0.0,  0.0] },
        Vertex { position: [-1.0, -1.0,  1.0], normal: [-1.0,  0.0,  0.0] },
        Vertex { position: [-1.0,  1.0,  1.0], normal: [-1.0,  0.0,  0.0] },
        Vertex { position: [-1.0,  1.0, -1.0], normal: [-1.0,  0.0,  0.0] },

        // Right (+X)
        Vertex { position: [ 1.0, -1.0,  1.0], normal: [ 1.0,  0.0,  0.0] },
        Vertex { position: [ 1.0, -1.0, -1.0], normal: [ 1.0,  0.0,  0.0] },
        Vertex { position: [ 1.0,  1.0, -1.0], normal: [ 1.0,  0.0,  0.0] },
        Vertex { position: [ 1.0,  1.0,  1.0], normal: [ 1.0,  0.0,  0.0] },

        // Top (+Y)
        Vertex { position: [-1.0,  1.0,  1.0], normal: [ 0.0,  1.0,  0.0] },
        Vertex { position: [ 1.0,  1.0,  1.0], normal: [ 0.0,  1.0,  0.0] },
        Vertex { position: [ 1.0,  1.0, -1.0], normal: [ 0.0,  1.0,  0.0] },
        Vertex { position: [-1.0,  1.0, -1.0], normal: [ 0.0,  1.0,  0.0] },

        // Bottom (-Y)
        Vertex { position: [-1.0, -1.0, -1.0], normal: [ 0.0, -1.0,  0.0] },
        Vertex { position: [ 1.0, -1.0, -1.0], normal: [ 0.0, -1.0,  0.0] },
        Vertex { position: [ 1.0, -1.0,  1.0], normal: [ 0.0, -1.0,  0.0] },
        Vertex { position: [-1.0, -1.0,  1.0], normal: [ 0.0, -1.0,  0.0] },
    ];

    let mut indices: Vec<u32> = Vec::with_capacity(36);
    for face in 0..6 {
        let o = (face * 4) as u32;
        indices.extend_from_slice(&[o, o + 1, o + 2, o, o + 2, o + 3]);
    }

    let vbuf = VertexBuffer::immutable(facade, &vertices)?;
    let ibuf = IndexBuffer::immutable(facade, PrimitiveType::TrianglesList, &indices)?;

    Ok(Mesh { vbuf, ibuf })
}
