//! GPU-ready mesh loader for **glTF 2.0** (internal helper)
//!
//! Converts the first primitive of a glTF document into our engine `Model`.

use anyhow::{Context, Result};
use glium::{backend::Facade, IndexBuffer, VertexBuffer};
use glium::index::PrimitiveType;
use gltf::mesh::util::ReadIndices;
use std::{fmt::Debug, path::Path};
use crate::model::{Vertex, Mesh, Material, Model};
use glium::texture::{RawImage2d, Texture2d, SrgbTexture2d};
use glium::uniforms::{SamplerWrapFunction, MinifySamplerFilter, MagnifySamplerFilter};
use gltf::image::Format as GltfFormat;
use glam::Vec2;

/// Load a glTF 2.0 file from disk and upload the first primitive to the GPU.
pub fn load_gltf<P, F>(path: P, facade: &F) -> Result<Model>
where
    P: AsRef<Path> + Debug,
    F: Facade + ?Sized,
{
    // -- parse the asset & bring buffer blobs into memory --
    let (doc, buffers, images) = gltf::import(path.as_ref()).context("failed to import glTF file")?;

    // -- grab the very first mesh / primitive --
    let mesh      = doc.meshes().next().context("glTF has no meshes")?;
    let primitive = mesh.primitives().next().context("mesh has no primitives")?;

    // ---------- MATERIAL ----------
    let mut mat = Material::default();

    if let Some(mat_idx) = primitive.material().index() {
        let material = doc.materials().nth(mat_idx).unwrap();
        let pbr      = material.pbr_metallic_roughness();

        // Factors --------------------------------------------------
        mat.base_color_factor = pbr.base_color_factor();
        mat.metal_factor      = pbr.metallic_factor();
        mat.roughness_factor  = pbr.roughness_factor();
        mat.emissive_factor   = material.emissive_factor();

        // Helper to update sampler settings from glTF sampler
        fn update_sampler(mat: &mut Material, t: &gltf::texture::Texture<'_>) {
            let sampler_info = t.sampler();
            mat.sampler.wrap_function.0 = match sampler_info.wrap_s() {
                gltf::texture::WrappingMode::ClampToEdge => SamplerWrapFunction::Clamp,
                gltf::texture::WrappingMode::MirroredRepeat => SamplerWrapFunction::Mirror,
                gltf::texture::WrappingMode::Repeat => SamplerWrapFunction::Repeat,
            };
            mat.sampler.wrap_function.1 = match sampler_info.wrap_t() {
                gltf::texture::WrappingMode::ClampToEdge => SamplerWrapFunction::Clamp,
                gltf::texture::WrappingMode::MirroredRepeat => SamplerWrapFunction::Mirror,
                gltf::texture::WrappingMode::Repeat => SamplerWrapFunction::Repeat,
            };
            if let Some(f) = sampler_info.mag_filter() {
                mat.sampler.magnify_filter = match f {
                    gltf::texture::MagFilter::Nearest => MagnifySamplerFilter::Nearest,
                    gltf::texture::MagFilter::Linear => MagnifySamplerFilter::Linear,
                };
            }
            if let Some(f) = sampler_info.min_filter() {
                mat.sampler.minify_filter = match f {
                    gltf::texture::MinFilter::Nearest => MinifySamplerFilter::Nearest,
                    gltf::texture::MinFilter::Linear => MinifySamplerFilter::Linear,
                    gltf::texture::MinFilter::NearestMipmapNearest => MinifySamplerFilter::NearestMipmapNearest,
                    gltf::texture::MinFilter::NearestMipmapLinear => MinifySamplerFilter::NearestMipmapLinear,
                    gltf::texture::MinFilter::LinearMipmapNearest => MinifySamplerFilter::LinearMipmapNearest,
                    gltf::texture::MinFilter::LinearMipmapLinear => MinifySamplerFilter::LinearMipmapLinear,
                };
            }
        }

        // Base-color texture (sRGB)
        if let Some(info) = pbr.base_color_texture() {
            update_sampler(&mut mat, &info.texture());
            let view = info.texture().source().index();
            mat.base_color = Some(glium_srgb_texture(facade, &images[view])?);
        }

        // Metallic-Roughness (linear)
        if let Some(info) = pbr.metallic_roughness_texture() {
            update_sampler(&mut mat, &info.texture());
            let view = info.texture().source().index();
            mat.metallic_roughness = Some(glium_linear_texture(facade, &images[view])?);
        }

        // Normal map (linear)
        if let Some(info) = material.normal_texture() {
            update_sampler(&mut mat, &info.texture());
            let view = info.texture().source().index();
            mat.normal = Some(glium_linear_texture(facade, &images[view])?);
        }

        // Occlusion (linear)
        if let Some(info) = material.occlusion_texture() {
            update_sampler(&mut mat, &info.texture());
            let view = info.texture().source().index();
            mat.occlusion = Some(glium_linear_texture(facade, &images[view])?);
        }

        // Emissive (sRGB)
        if let Some(info) = material.emissive_texture() {
            update_sampler(&mut mat, &info.texture());
            let view = info.texture().source().index();
            mat.emissive = Some(glium_srgb_texture(facade, &images[view])?);
        }

        // KHR_texture_transform
        if let Some(tex) = pbr.base_color_texture() {
            if let Some(xform) = tex.texture_transform() {
                mat.uv_offset = Vec2::new(xform.offset()[0], xform.offset()[1]);
                mat.uv_scale  = Vec2::new(xform.scale()[0],  xform.scale()[1]);
            }
        }
    }

    // ---- Vertex/index data ----
    let reader = primitive.reader(|buf| Some(&buffers[buf.index()].0));

    let positions: Vec<[f32; 3]> = reader.read_positions().context("missing POSITION")?.collect();
    let normals:   Vec<[f32; 3]> = reader.read_normals().context("missing NORMAL")?.collect();
    let tex_coords: Vec<[f32; 2]> = reader.read_tex_coords(0).map(|tc| tc.into_f32().collect()).unwrap_or_else(|| vec![[0.0, 0.0]; positions.len()]);
    let indices:   Vec<u32> = reader.read_indices().context("missing indices")?.into_u32().collect();

    // Interleave
    let vertices: Vec<Vertex> = (0..positions.len()).map(|i| Vertex { position: positions[i], normal: normals[i], tex_coords: tex_coords[i] }).collect();

    let vbuf = VertexBuffer::immutable(facade, &vertices)?;
    let ibuf = IndexBuffer ::immutable(facade, PrimitiveType::TrianglesList, &indices)?;

    Ok(Model { mesh: Mesh { vbuf, ibuf }, material: mat })
}

/// Linear-space texture (RGBA8) from glTF image data.
fn glium_linear_texture<F>(facade: &F, img: &gltf::image::Data) -> Result<Texture2d>
where
    F: Facade + ?Sized,
{
    let rgba = to_rgba(img);
    let raw = RawImage2d::from_raw_rgba(rgba, (img.width, img.height));
    Ok(Texture2d::new(facade, raw)?)
}

/// sRGB texture from glTF image data.
fn glium_srgb_texture<F>(facade: &F, img: &gltf::image::Data) -> Result<SrgbTexture2d>
where
    F: Facade + ?Sized,
{
    let rgba = to_rgba(img);
    let raw = RawImage2d::from_raw_rgba(rgba, (img.width, img.height));
    Ok(SrgbTexture2d::new(facade, raw)?)
}

/// Convert various glTF image formats to RGBA8 as expected by glium.
fn to_rgba(img: &gltf::image::Data) -> Vec<u8> {
    match img.format {
        GltfFormat::R8G8B8A8 => img.pixels.clone(),
        GltfFormat::R8G8B8 => {
            // Expand RGB to RGBA with alpha=255
            img.pixels
                .chunks(3)
                .flat_map(|rgb| [rgb[0], rgb[1], rgb[2], 255u8])
                .collect()
        }
        GltfFormat::R8G8 => {
            // Treat RG as luminance+alpha? For simplicity, replicate first channel into RGB, second as alpha.
            img.pixels
                .chunks(2)
                .flat_map(|rg| [rg[0], rg[0], rg[0], rg[1]])
                .collect()
        }
        GltfFormat::R8 => {
            // Grayscale: replicate into RGB, alpha=255
            img.pixels
                .iter()
                .flat_map(|l| [*l, *l, *l, 255u8])
                .collect()
        }
        _ => img.pixels.clone(),
    }
} 