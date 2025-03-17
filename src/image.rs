
use image;

pub struct RGBD {
    pub rgb: Vec<u8>,
    pub depth: Vec<f32>
}


pub fn create_rgbd_image(rgb_path: &str, depth_path:&str) -> RGBD {
    let rgb: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> = image::open(rgb_path).unwrap().to_rgb8();
    let depth: image::ImageBuffer<image::Luma<u16>, Vec<u16>> = image::open(depth_path).unwrap().to_luma16();

    let rgb_flat = rgb.into_raw();

    let depth_flat = depth.into_raw().iter().map(|&d| d as f32 / 1000.0).collect();

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
        const rgb_path:&str = "rgb.png";
        const depth_path:&str = "depth.png";
        let rgbd = create_rgbd_image(rgb_path, depth_path);

        assert_eq!(rgbd.rgb.len(), 640*480*3);
        assert_eq!(rgbd.depth.len(), 640*480);
    }
}