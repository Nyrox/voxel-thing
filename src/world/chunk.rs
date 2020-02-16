use crate::world::{Voxel, VoxelType};

use cgmath::Vector3;

use gl::types::*;
use std::mem;
use std::ptr;

struct BasicVertex {
    position: Vector3<f32>,
}

impl BasicVertex {
    pub fn from_pos(x: f32, y: f32, z: f32) -> BasicVertex {
        BasicVertex {
            position: Vector3::new(x, y, z),
        }
    }
}

pub const CHUNK_DIM: u32 = 8;
pub const CHUNK_HEIGHT: u32 = 64;
pub const CHUNK_N_VOXELS: usize = (CHUNK_DIM * CHUNK_DIM * CHUNK_HEIGHT) as usize;

pub struct Chunk {
    pub voxels: [Voxel; CHUNK_N_VOXELS],
    pub dirty: bool,
}

impl Chunk {
    pub fn void() -> Chunk {
        Chunk {
            voxels: [Voxel::void(); CHUNK_N_VOXELS],
            dirty: false,
        }
    }

    pub fn iter_mut<F>(&mut self, mut f: F)
    where
        F: FnMut((u32, u32, u32), &mut Voxel),
    {
        for z in 0..CHUNK_DIM {
            for y in 0..CHUNK_HEIGHT {
                for x in 0..CHUNK_DIM {
                    f((x, y, z), self.voxel_mut(x, y, z))
                }
            }
        }

        self.dirty = true;
    }

    pub fn iter<F>(&self, f: F)
    where
        F: Fn((u32, u32, u32), &Voxel),
    {
        for z in 0..CHUNK_DIM {
            for y in 0..CHUNK_HEIGHT {
                for x in 0..CHUNK_DIM {
                    f((x, y, z), self.voxel(x, y, z))
                }
            }
        }
    }

    pub fn voxel_mut(&mut self, x: u32, y: u32, z: u32) -> &mut Voxel {
        self.dirty = true;
        &mut self.voxels[(z + y * CHUNK_DIM * CHUNK_DIM + x * CHUNK_DIM) as usize]
    }

    pub fn voxel(&self, x: u32, y: u32, z: u32) -> &Voxel {
        &self.voxels[(z + y * CHUNK_DIM * CHUNK_DIM + x * CHUNK_DIM) as usize]
    }

    pub fn gen_flat(ground: u32) -> Chunk {
        let mut chunk = Chunk::void();

        chunk.iter_mut(|(_, y, _), v| {
            *v = if y < ground {
                Voxel {
                    voxel_type: VoxelType::GROUND,
                }
            } else {
                Voxel::void()
            }
        });

        chunk
    }

    pub fn gen_vertex_array(&mut self) -> (u32, u32) {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        let mut i = 0;
        self.iter_mut(|(x, y, z), tile| {
            if tile.voxel_type == VoxelType::VOID {
                return;
            }
            // top plane
            vertices.push(BasicVertex::from_pos(
                x as f32 + 0.0,
                y as f32 + 0.0,
                z as f32 + 1.0,
            ));
            vertices.push(BasicVertex::from_pos(
                x as f32 + 1.0,
                y as f32 + 0.0,
                z as f32 + 1.0,
            ));
            vertices.push(BasicVertex::from_pos(
                x as f32 + 1.0,
                y as f32 + 1.0,
                z as f32 + 1.0,
            ));
            vertices.push(BasicVertex::from_pos(
                x as f32 + 0.0,
                y as f32 + 1.0,
                z as f32 + 1.0,
            ));
            // bottom plane
            vertices.push(BasicVertex::from_pos(
                x as f32 + 0.0,
                y as f32 + 0.0,
                z as f32 + 0.0,
            ));
            vertices.push(BasicVertex::from_pos(
                x as f32 + 1.0,
                y as f32 + 0.0,
                z as f32 + 0.0,
            ));
            vertices.push(BasicVertex::from_pos(
                x as f32 + 1.0,
                y as f32 + 1.0,
                z as f32 + 0.0,
            ));
            vertices.push(BasicVertex::from_pos(
                x as f32 + 0.0,
                y as f32 + 1.0,
                z as f32 + 0.0,
            ));

            indices.append(&mut vec![
                // top face
                i + 0,
                i + 1,
                i + 2,
                i + 0,
                i + 2,
                i + 3,
                // bottom face
                i + 4,
                i + 5,
                i + 6,
                i + 4,
                i + 6,
                i + 7,
                // left face
                i + 0,
                i + 4,
                i + 3,
                i + 3,
                i + 4,
                i + 7,
                // right face
                i + 1,
                i + 2,
                i + 6,
                i + 1,
                i + 6,
                i + 5,
                // front face
                i + 0,
                i + 1,
                i + 5,
                i + 0,
                i + 5,
                i + 4,
                // back face
                i + 2,
                i + 3,
                i + 7,
                i + 2,
                i + 7,
                i + 6,
            ]);

            i += 8;
        });

        let mut vao = 0;
        let mut vbo = 0;
        let mut ebo = 0;

        unsafe {
            gl::CreateVertexArrays(1, &mut vao);
            gl::CreateBuffers(1, &mut vbo);
            gl::CreateBuffers(1, &mut ebo);

            gl::BindVertexArray(vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::NamedBufferData(
                vbo,
                (mem::size_of::<BasicVertex>() * vertices.len()) as isize,
                vertices.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW,
            );

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::NamedBufferData(
                ebo,
                (mem::size_of::<u32>() * indices.len()) as isize,
                indices.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW,
            );

            // Positions
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                0,
                mem::size_of::<BasicVertex>() as i32,
                ptr::null(),
            );
            gl::EnableVertexAttribArray(0);

            gl::BindVertexArray(0);
        }

        (vao, indices.len() as u32)
    }
}
