use crate::voxel::{VoxelGrid, Voxel};
use crate::utils::parse_camera_intrensics;
use glam::Vec3;
use image;

/// Struct to store the RGB and Depth vectors
pub struct RGBD {
    pub rgb: Vec<u8>,
    pub depth: Vec<f32>,
    pub width: u32,   // Added width of RGB image
    pub height: u32,  // Added height of RGB image
}

pub struct Point3D {
    pub position: Vec3,
    pub color: Vec3,
}


pub fn create_rgbd_image(rgb_path: &str, depth_path: &str) -> RGBD {
    let rgb_img = image::open(rgb_path).unwrap().to_rgb8(); // Load the RGB image
    let depth_img = image::open(depth_path).unwrap().to_luma16(); // Load the Depth image

    let (width, height) = rgb_img.dimensions(); // Get RGB image dimensions
    let depth_dims = depth_img.dimensions();

    assert_eq!(
        (width, height),
        depth_dims,
        "RGB and Depth images must have the same dimensions!"
    );

    let rgb_flat = rgb_img.into_raw(); // Convert the RGB image to a flat vector
    let depth_flat = depth_img.into_raw().iter().map(|&d| d as f32 / 1000.0).collect(); // Convert Depth image to meters

    RGBD {
        rgb: rgb_flat,
        depth: depth_flat,
        width,
        height,
    }
}


pub fn rgbd_to_pointcloud(rgbd: &RGBD, intrinsics_path: &str) -> Vec<Point3D> {
    let intrinsics = parse_camera_intrensics(intrinsics_path);
    let mut point_cloud = Vec::new();

    for v in 0..rgbd.height {
        for u in 0..rgbd.width {
            let idx = (v * rgbd.width + u) as usize;
            let depth = rgbd.depth[idx];

            if depth > 0.0 { // Ignore invalid depth
                // Compute normalized coordinates
                let x = (u as f32 - intrinsics.z_axis.x) * depth / intrinsics.x_axis.x;
                let y = (v as f32 - intrinsics.z_axis.y) * depth / intrinsics.y_axis.y;
                let z = depth;

                // Extract RGB color
                let r = rgbd.rgb[idx * 3] as f32 / 255.0;
                let g = rgbd.rgb[idx * 3 + 1] as f32 / 255.0;
                let b = rgbd.rgb[idx * 3 + 2] as f32 / 255.0;

                point_cloud.push(Point3D {
                    position: Vec3{x, y,z},
                    color: Vec3::new(r, g, b),
                });
            }
        }
    }

    point_cloud
}



pub fn create_voxel_grid(point_cloud:Vec<Point3D>) -> VoxelGrid{
    let mut voxel_grid = VoxelGrid::new(0.01);
    for point in point_cloud.iter(){
        voxel_grid.add_point(point.position, point.color);
    }
    voxel_grid
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_pipeline() {
        const rgb_path:&str = "images/source_img.png";
        const depth_path:&str = "images/source_depth.png";
        let rgbd = create_rgbd_image(rgb_path, depth_path);

        assert_eq!(rgbd.rgb.len(), 640*480*3);
        assert_eq!(rgbd.depth.len(), 640*480);

    }
}