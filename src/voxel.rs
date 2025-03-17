use glam::{Vec3, IVec3};
use std::collections::HashMap;

pub struct VoxelGrid {
    pub voxels: HashMap<IVec3, Voxel>,  // Use IVec3 as key for indexing
    pub size: f32,  // Voxel size
}

impl VoxelGrid {
    pub fn add_point(&mut self, position: Vec3, color: Vec3) {
        let index = IVec3::new(
            (position.x / self.size) as i32,
            (position.y / self.size) as i32,
            (position.z / self.size) as i32,
        );

        if let Some(voxel) = self.voxels.get_mut(&index) {
            voxel.update(color); // Update existing voxel
        } else {
            self.voxels.insert(index, Voxel::new(index, color, self.size));
        }
    }
}

pub struct Voxel {
    index: IVec3,   // Store integer voxel index
    position: Vec3, // World-space center position of voxel
    color: Vec3,    // Averaged color
    count: u32,     // Number of points merged into this voxel
    size: f32,      // Size of voxel
}

impl Voxel {
    pub fn new(index: IVec3, color: Vec3, size: f32) -> Voxel {
        let position = Vec3::new(
            (index.x as f32 + 0.5) * size,  // Centering voxel
            (index.y as f32 + 0.5) * size,
            (index.z as f32 + 0.5) * size,
        );

        Voxel {
            index,
            position,
            color,
            count: 1,
            size,
        }
    }

    pub fn update(&mut self, new_color: Vec3) {
        // Average the color over multiple updates
        self.color = (self.color * self.count as f32 + new_color) / (self.count as f32 + 1.0);
        self.count += 1;
    }
}
