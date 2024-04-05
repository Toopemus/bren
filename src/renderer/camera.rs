use nalgebra::{Isometry3, Perspective3, Point3, Vector3};

#[derive(Debug)]
pub struct Camera {
    pub view_matrix: Isometry3<f32>,
    pub projection_matrix: Perspective3<f32>,
}

impl Camera {
    pub fn new() -> Camera {
        let eye = Point3::new(0.0, 0.0, 1.0);
        let target = Point3::new(0.0, 0.0, 0.0);
        let view_matrix = Isometry3::look_at_rh(&eye, &target, &Vector3::y());

        let projection_matrix = Perspective3::new(16.0 / 9.0, 3.14 / 2.0, 1.0, 1000.0);

        Camera {
            view_matrix,
            projection_matrix,
        }
    }
}
