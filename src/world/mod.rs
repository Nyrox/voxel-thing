pub mod chunk;
pub mod voxel;

pub use chunk::Chunk;
pub use voxel::{Voxel, VoxelType};

use cgmath::Vector2;

use std::fs::File;
use std::io::prelude::*;


#[derive(Debug, Clone, Copy)]
pub struct ChunkIndex(Vector2<i32>);

impl Into<ChunkIndex> for (i32, i32) {
    fn into(self) -> ChunkIndex {
        let (x, z) = self;
        ChunkIndex(Vector2::new(x, z))
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
		self.voxel_shader.setUniform("chunkDims", Vector2::new(chunk::CHUNK_DIM as i32, chunk::CHUNK_DIM as i32));

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
}

impl World {
    pub fn empty() -> World {
        World { chunks: Vec::new() }
    }

    pub fn insert_chunk<C>(&mut self, i: C, chunk: Chunk)
    where
        C: Into<ChunkIndex>,
    {
        self.chunks
            .push((i.into(), chunk, ChunkRenderdata::default()));
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
