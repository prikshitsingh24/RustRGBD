
use image;

/// Struct to store the RGB and Depth vectors
pub struct RGBD {
    pub rgb: Vec<u8>,
    pub depth: Vec<f32>
}


pub fn create_rgbd_image(rgb_path: &str, depth_path:&str) -> RGBD {
    let rgb: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> = image::open(rgb_path).unwrap().to_rgb8(); // Load the RGB
    let depth: image::ImageBuffer<image::Luma<u16>, Vec<u16>> = image::open(depth_path).unwrap().to_luma16(); // Load the Depth

    let rgb_flat = rgb.into_raw(); // Convert the RGB image to a flat vector

    let depth_flat = depth.into_raw().iter().map(|&d| d as f32 / 1000.0).collect(); // Convert the Depth image to a flat vector and divide by 1000 to get the depth in meters

    RGBD {
        rgb: rgb_flat,
        depth: depth_flat
    }

}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_rgbd_image() {
        const rgb_path:&str = "images/source_img.png";
        const depth_path:&str = "images/source_depth.png";
        let rgbd = create_rgbd_image(rgb_path, depth_path);

        assert_eq!(rgbd.rgb.len(), 640*480*3);
        assert_eq!(rgbd.depth.len(), 640*480);
    }
}