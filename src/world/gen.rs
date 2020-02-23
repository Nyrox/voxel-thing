use noise::{Perlin, NoiseFn};

use crate::world::chunk;
use crate::world::{ChunkIndex, voxel::Voxel, voxel::VoxelType};

pub struct WorldGenerator {
    noise: noise::Perlin,
}

const NOISE_SCALE: f64 = 100.0;

impl WorldGenerator {
    pub fn new() -> WorldGenerator {
        WorldGenerator {
            noise: Perlin::new(),
        }
    }

    pub fn gen_chunk<C>(&mut self, i: C) -> chunk::Chunk
        where C: Into<ChunkIndex>
    {
        let mut chunk = chunk::Chunk::void();
        let i: ChunkIndex = i.into();

        let sampler = &self.noise;

        chunk.iter_mut(|(x, y, z), v| {
            let x = i.chunk_origin().x + x as i32;
            let z = i.chunk_origin().z + z as i32;

            let sampled_height = sampler.get([x as f64 / NOISE_SCALE, z as f64 / NOISE_SCALE]);
            *v = if (y as f64) < (sampled_height * (chunk::CHUNK_HEIGHT - 80) as f64 + 40.0) {
                Voxel {
                    voxel_type: VoxelType::GROUND,
                }
            }  else {
                Voxel::void()
            }
        });

        chunk
    }
}