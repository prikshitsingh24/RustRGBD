use glam::IVec3;

use crate::{image::{create_rgbd_image, rgbd_to_pointcloud, create_voxel_grid}, voxel::Voxel};


struct Correspondence {
    source_voxel:Voxel,
    target_voxel:Voxel,
}


/// Park[2017] RGBD Odometry which aligns two RGBD images
pub fn compute_rgbd_odometry(source_rgb_path:&str, source_depth_path:&str, target_rgb_path:&str, target_depth_path:&str, camera_intrinsics_path:&str){
    // source
    let source_rgbd = create_rgbd_image(source_rgb_path, source_depth_path);
    let source_point_cloud = rgbd_to_pointcloud(&source_rgbd, camera_intrinsics_path);
    let source_voxel_grid = create_voxel_grid(source_point_cloud);

    // target
    let target_rgbd = create_rgbd_image(target_rgb_path, target_depth_path);
    let target_point_cloud = rgbd_to_pointcloud(&target_rgbd, camera_intrinsics_path);
    let target_voxel_grid = create_voxel_grid(target_point_cloud);

    // Compute transformation
    let mut correspondences: Vec<Correspondence> = Vec::new();
    for voxel in source_voxel_grid.voxels.iter(){
        for dx in -1..=1 {
            for dy in -1..=1 {
                for dz in -1..=1 {
                    let index = IVec3::new(voxel.0.x + dx, voxel.0.y +dy, voxel.0.z + dz);
                    if let Some(target_voxel) = target_voxel_grid.voxels.get(&index) {
                        correspondences.push(Correspondence{source_voxel:voxel.1.clone(), target_voxel:target_voxel.clone()});
                    }
                }
            }
        }
    }
    


}