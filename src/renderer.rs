pub mod camera;
pub mod model;
pub mod viewport;

use crate::renderer::model::Model;
use nalgebra::{Isometry3, Matrix4, Point3};
use viewport::Viewport;

use self::camera::Camera;

/// A single point in 3D-space.
#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    position: Point3<f32>,
}

impl Vertex {
    pub fn project(&mut self, mvp_matrix: Matrix4<f32>, view_width: f32, view_height: f32) {
        self.position = mvp_matrix.transform_point(&self.position);
        self.position.x = self.position.x * view_width + view_width / 2.0;
        self.position.y = self.position.y * view_height + view_height / 2.0;
    }
}

/// A face of a 3D-object.
///
/// Contains the indexes of the vertices that form the face.
#[derive(Debug)]
pub struct Face {
    indexes: (usize, usize, usize),
}

/// A struct used to render 3D-objects.
///
/// The struct should be constructed with [`new`]. It manages a screen buffer to which the objects
/// are rasterised onto. Applications are expected to call [`clear`] first, then an arbitrary number
/// of draw calls, after which [`render`] should be called to print the screen buffer to the
/// terminal. See the method documentation for more info.
///
/// [`new`]: #method.new
/// [`clear`]: #method.clear
/// [`render`]: #method.render
#[derive(Debug)]
pub struct Renderer {
    /// The viewport that the renderer draws onto.
    pub viewport: Viewport,
    screen_buffer: Vec<Vec<bool>>,
    camera: Camera,
}

impl Renderer {
    /// Constructs the renderer. [`Viewport`] must be passed to the constructor.
    pub fn new(viewport: Viewport) -> Renderer {
        let viewport_size = viewport.size();

        Renderer {
            screen_buffer: vec![vec![false; viewport_size.1 as usize]; viewport_size.0 as usize],
            viewport,
            camera: Camera::new(),
        }
    }

    /// Renders the screen buffer on the screen. Should be called after draw-calls. This will not
    /// erase the screen buffer, so you should call [`clear`] after.
    ///
    /// [`clear`]: #method.clear
    pub fn render(&mut self) {
        self.viewport.draw_chars(&self.screen_buffer);
    }

    /// Clears the screen buffer.
    pub fn clear(&mut self) {
        let viewport_size = self.viewport.size();
        self.screen_buffer = vec![vec![false; viewport_size.1 as usize]; viewport_size.0 as usize];
    }

    /// Draws a [`Model`] to the screen buffer. calling [`render`] afterwards will render the model
    /// to the screen.
    ///
    /// [`render`]: #method.render
    pub fn draw_object(&mut self, model: &Model) {
        let mvp_matrix = Self::construct_mvp(model.model_matrix(), &self.camera);
        let (width, height) = self.viewport.size();

        for face in model.index_buffer() {
            let mut v0 = model.vertex_at(face.indexes.0 - 1);
            let mut v1 = model.vertex_at(face.indexes.1 - 1);
            let mut v2 = model.vertex_at(face.indexes.2 - 1);

            v0.project(mvp_matrix, width as f32, height as f32);
            v1.project(mvp_matrix, width as f32, height as f32);
            v2.project(mvp_matrix, width as f32, height as f32);

            Self::draw_line(self, &v0, &v1);
            Self::draw_line(self, &v1, &v2);
            Self::draw_line(self, &v2, &v0);
        }
    }

    fn construct_mvp(model_matrix: Isometry3<f32>, camera: &Camera) -> Matrix4<f32> {
        let model_view_matrix = model_matrix * camera.view_matrix;
        let mvp_matrix = camera.projection_matrix.as_matrix() * model_view_matrix.to_homogeneous();

        mvp_matrix
    }

    fn draw_pixel(&mut self, x: i16, y: i16) {
        let x_size = self.viewport.size().0 as i16;
        let y_size = self.viewport.size().1 as i16;
        if x >= 0 && x < x_size && y >= 0 && y < y_size {
            self.screen_buffer[x as usize][y as usize] = true;
        }
    }

    fn draw_line(&mut self, v1: &Vertex, v2: &Vertex) {
        let mut x1 = v1.position.x as i16;
        let mut y1 = v1.position.y as i16;
        let x2 = v2.position.x as i16;
        let y2 = v2.position.y as i16;
        let dx = (x2 - x1).abs();
        let dy = -(y2 - y1).abs();
        let sx = if x1 < x2 { 1 } else { -1 };
        let sy = if y1 < y2 { 1 } else { -1 };
        let mut err = dy + dx;
        let mut err2;

        loop {
            Self::draw_pixel(self, x1, y1);
            if x1 == x2 && y1 == y2 {
                break;
            }
            err2 = 2 * err;
            if err2 >= dy {
                if x1 == x2 {
                    break;
                }
                err += dy;
                x1 += sx;
            }
            if err2 <= dx {
                if y1 == y2 {
                    break;
                }
                err += dx;
                y1 += sy;
            }
        }
    }
}
