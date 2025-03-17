use std::{fs::File, io::BufReader};
use glam::{Mat3, Vec3};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct Camera {
    fx: f32,
    fy: f32,
    cx: f32,
    cy: f32,
}

pub fn parse_camera_intrensics(file_path: &str) -> Mat3 {
    let file = File::open(file_path).expect("file not found");
    let reader = BufReader::new(file);
    let camera:Camera = serde_json::from_reader(reader).unwrap();
    let intrinisics = Mat3 { x_axis: Vec3{x: camera.fx,y: 0.0,      z: 0.0}, 
                                 y_axis: Vec3{x: 0.0,      y: camera.fy,z: 0.0}, 
                                 z_axis: Vec3{x: camera.cx,y: camera.cy,z: 0.0} 
                                };
    intrinisics

}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_camera_intrensics() {
        const path:&str = "camera.json";
        let original_intrensics = Mat3 {
            x_axis: Vec3{x:525.0, y:0.0, z:0.0},
            y_axis: Vec3{x:0.0,y:525.0, z:0.0},
            z_axis: Vec3{x:319.5,y:239.5, z:0.0}
        };
        let camera_intrensics:Mat3 = parse_camera_intrensics(path);

        assert_eq!(camera_intrensics, original_intrensics);

        
    }
}