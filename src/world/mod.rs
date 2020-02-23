pub mod chunk;
pub mod voxel;

pub mod gen;

pub use chunk::Chunk;
pub use voxel::{Voxel, VoxelType};

use cgmath::Vector2;
use cgmath::Vector3;

use cgmath::num_traits::Signed;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ChunkIndex(Vector2<i32>);

impl Into<ChunkIndex> for (i32, i32) {
    fn into(self) -> ChunkIndex {
        let (x, z) = self;
        ChunkIndex(Vector2::new(x, z))
    }
}

impl ChunkIndex {
    pub fn chunk_origin(self) -> Vector3<i32> {
        Vector3::new(
            self.0.x * chunk::CHUNK_DIM as i32,
            0,
            self.0.y * chunk::CHUNK_DIM as i32,
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VoxelIndex(Vector3<i32>);

impl VoxelIndex {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        VoxelIndex(Vector3::new(x, y, z))
    }

    pub fn chunk_index(&self) -> ChunkIndex {
        ChunkIndex(Vector2::new(
            ((self.0.x) / chunk::CHUNK_DIM as i32) - self.0.x.is_negative() as i32,
            ((self.0.z) / chunk::CHUNK_DIM as i32) - self.0.z.is_negative() as i32,
        ))
    }

    pub fn local_part(&self) -> Vector3<i32> {
        self.0 - self.chunk_index().chunk_origin()
    }

    pub fn from_world(world: cgmath::Point3<f32>) -> VoxelIndex {
        VoxelIndex(Vector3::new(
            world.x as i32 - world.x.is_negative() as i32,
            world.y as i32 - world.y.is_negative() as i32,
            world.z as i32 - world.z.is_negative() as i32,
        ))
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct ChunkRenderdata {
    pub vao: u32,
    pub indices_len: u32,
}

impl ChunkRenderdata {
    pub fn from_vao_handle((vao, indices_len): (u32, u32)) -> ChunkRenderdata {
        ChunkRenderdata { vao, indices_len }
    }
}

pub struct WorldRenderer {
    voxel_shader: graphics::Shader,
    pub camera: crate::Camera,
}

pub fn read_file_contents(filename: &str) -> String {
    let mut f = File::open(filename).unwrap();
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).unwrap();
    buffer
}

impl WorldRenderer {
    pub fn new(camera: crate::Camera) -> WorldRenderer {
        let voxelshade = graphics::Shader::new();
        voxelshade
            .attach(
                &read_file_contents("assets/shaders/voxel.vs"),
                gl::VERTEX_SHADER,
            )
            .unwrap();
        voxelshade
            .attach(
                &read_file_contents("assets/shaders/voxel.fs"),
                gl::FRAGMENT_SHADER,
            )
            .unwrap();
        voxelshade
            .attach(
                &read_file_contents("assets/shaders/voxel.gs"),
                gl::GEOMETRY_SHADER,
            )
            .unwrap();
        voxelshade.compile().unwrap();
        voxelshade.bind();

        WorldRenderer {
            camera,
            voxel_shader: voxelshade,
        }
    }

    pub fn draw_chunk(&self, i: ChunkIndex, renderdata: ChunkRenderdata) {
        self.voxel_shader.bind();
        self.voxel_shader
            .setUniform("view", self.camera.get_view_matrix());
        self.voxel_shader.setUniform(
            "cameraPos",
            self.camera.transform.position.to_homogeneous().truncate(),
        );
        self.voxel_shader
            .setUniform("projection", self.camera.get_projection_matrix());
        self.voxel_shader.setUniform("gTime", 0i32);
        self.voxel_shader.setUniform("chunkIndex", i.0);
        self.voxel_shader.setUniform(
            "chunkDims",
            Vector2::new(chunk::CHUNK_DIM as i32, chunk::CHUNK_DIM as i32),
        );

        unsafe {
            gl::BindVertexArray(renderdata.vao);
            gl::DrawElements(
                gl::TRIANGLES,
                renderdata.indices_len as i32,
                gl::UNSIGNED_INT,
                0 as *const gl::types::GLvoid,
            );
            gl::BindVertexArray(0);
        }
    }
}

pub struct World {
    pub chunks: Vec<(ChunkIndex, Chunk, ChunkRenderdata)>,
    generator: gen::WorldGenerator,
}

impl World {
    pub fn empty() -> World {
        World {
            chunks: Vec::new(),
            generator: gen::WorldGenerator::new(),
        }
    }

    pub fn insert_chunk<C>(&mut self, i: C, chunk: Chunk)
    where
        C: Into<ChunkIndex>,
    {
        self.chunks
            .push((i.into(), chunk, ChunkRenderdata::default()));
    }

    pub fn gen_chunk<C>(&mut self, i: C)
    where
        C: Into<ChunkIndex> + Clone,
    {
        let chunk = self.generator.gen_chunk(i.clone());
        self.insert_chunk(i.into(), chunk);
    }

    pub fn voxel_from_world(&self, world: cgmath::Point3<f32>) -> VoxelIndex {
        VoxelIndex(Vector3::new(
            world.x as i32 - world.x.is_negative() as i32,
            world.y as i32 - world.y.is_negative() as i32,
            world.z as i32 - world.z.is_negative() as i32,
        ))
    }

    pub fn chunk(&self, chunkIndex: ChunkIndex) -> &Chunk {
        for (index, chunk, _) in self.chunks.iter() {
            if *index == chunkIndex {
                return chunk;
            }
        }

        panic!("God help us")
    }

    pub fn voxel(&self, index: VoxelIndex) -> Voxel {
        let i = index.local_part();
        *self
            .chunk(index.chunk_index())
            .voxel(i.x as i32, i.y as i32, i.z as i32)
    }

    pub fn render(&mut self, renderer: &WorldRenderer) {
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LESS);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }

        for (i, chunk, renderdata) in &mut self.chunks {
            if chunk.dirty {
                *renderdata = ChunkRenderdata::from_vao_handle(chunk.gen_vertex_array());
                chunk.dirty = false;
            }

            renderer.draw_chunk(*i, *renderdata);
        }
    }
}











/// Tests

mod tests {
    use super::*;

    #[test]
    pub fn voxel_to_chunk_index() {
        let samples = [
            (
                VoxelIndex(Vector3::new(0, 0, 0)),
                ChunkIndex(Vector2::new(0, 0)),
            ),
            (
                VoxelIndex(Vector3::new(-1, 0, -1)),
                ChunkIndex(Vector2::new(-1, -1)),
            ),
            (
                VoxelIndex(Vector3::new(chunk::CHUNK_DIM as i32, 0, -1)),
                ChunkIndex(Vector2::new(1, -1)),
            ),
            (
                VoxelIndex(Vector3::new(chunk::CHUNK_DIM as i32 * 4, 0, 0)),
                ChunkIndex(Vector2::new(4, 0)),
            ),
            (
                VoxelIndex(Vector3::new(chunk::CHUNK_DIM as i32 * -4, 0, 0)),
                ChunkIndex(Vector2::new(-5, 0)),
            ),
            (
                VoxelIndex(Vector3::new(0, 0, chunk::CHUNK_DIM as i32 * -3)),
                ChunkIndex(Vector2::new(0, -4)),
            ),
        ];

        for (sample, predicate) in samples.iter() {
            assert_eq!(sample.chunk_index(), *predicate);
        }
    }

    #[test]
    pub fn voxel_index_local_part() {
        let samples = [
            (VoxelIndex(Vector3::new(0, 0, 0)), Vector3::new(0, 0, 0)),
            (VoxelIndex(Vector3::new(5, 3, 7)), Vector3::new(5, 3, 7)),
            (
                VoxelIndex(Vector3::new(-3, 2, -4)),
                Vector3::new(chunk::CHUNK_DIM as i32 - 3, 2, chunk::CHUNK_DIM as i32 - 4),
            ),
            (VoxelIndex(Vector3::new(0, 0, 0)), Vector3::new(0, 0, 0)),
        ];

        for (sample, predicate) in samples.iter() {
            assert_eq!(sample.local_part(), *predicate);
        }
    }

    use cgmath::Point3;

    #[test]
    pub fn world_to_voxel() {
        let samples = [
            (Point3::new(0.0, 0.0, 0.0), VoxelIndex::new(0, 0, 0)),
            (Point3::new(1.2, 5.3, 6.4), VoxelIndex::new(1, 5, 6)),
            (Point3::new(-1.2, -5.7, -2.9), VoxelIndex::new(-2, -6, -3)),
            (
                Point3::new(
                    chunk::CHUNK_DIM as f32 * 3.0 + 2.0,
                    chunk::CHUNK_DIM as f32 * 3.0 + 2.0,
                    chunk::CHUNK_DIM as f32 * 3.0 + 2.0,
                ),
                VoxelIndex::new(
                    chunk::CHUNK_DIM * 3 + 2,
                    chunk::CHUNK_DIM * 3 + 2,
                    chunk::CHUNK_DIM * 3 + 2,
                ),
            ),
        ];

        for (sample, predicate) in samples.iter() {
            assert_eq!(VoxelIndex::from_world(*sample), *predicate);
        }
    }
}
