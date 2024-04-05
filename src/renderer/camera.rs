use nalgebra::{Isometry3, Perspective3, Point3, Vector3};

/// Virtual camera, through which the scene is rendered.
#[derive(Debug)]
pub struct Camera {
    /// Used to move object into camera space.
    pub view_matrix: Isometry3<f32>,
    /// Used to apply perspective projection on objects.
    pub projection: Perspective3<f32>,
}

impl Camera {
    /// Constructs the camera.
    ///
    /// Takes the aspect ratio of the viewport, vertical FOV, and locations of the near and far
    /// planes.
    pub fn new(aspect: f32, fovy: f32, znear: f32, zfar: f32) -> Camera {
        let eye = Point3::new(0.0, 0.0, 0.0);
        let target = Point3::new(0.0, 0.0, -1.0);
        let view_matrix = Isometry3::look_at_rh(&eye, &target, &Vector3::y());

        let projection = Perspective3::new(aspect, fovy, znear, zfar);

        Camera {
            view_matrix,
            projection,
        }
    }
}
