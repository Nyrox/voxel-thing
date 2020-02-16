#[derive(Copy, Clone, Debug, PartialEq)]
pub enum VoxelType {
    VOID,
    GROUND,
}

#[derive(Copy, Clone, Debug)]
pub struct Voxel {
    pub voxel_type: VoxelType,
}

impl Voxel {
    pub fn void() -> Voxel {
        Voxel {
            voxel_type: VoxelType::VOID,
        }
    }

    pub fn is_solid(self) -> bool {
        match self.voxel_type {
            VoxelType::VOID => false,
            _ => true,
        }
    }
}
