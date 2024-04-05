use nalgebra::{Isometry3, Perspective3, Point3, Vector3};

#[derive(Debug)]
pub struct Camera {
    pub view_matrix: Isometry3<f32>,
    pub projection_matrix: Perspective3<f32>,
}

impl Camera {
    pub fn new(aspect: f32, fovy: f32, znear: f32, zfar: f32) -> Camera {
        let eye = Point3::new(0.0, 0.0, 1.0);
        let target = Point3::new(0.0, 0.0, 0.0);
        let view_matrix = Isometry3::look_at_rh(&eye, &target, &Vector3::y());

        let projection_matrix = Perspective3::new(aspect, fovy, znear, zfar);

        Camera {
            view_matrix,
            projection_matrix,
        }
    }
}
