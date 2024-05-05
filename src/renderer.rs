pub mod camera;
pub mod model;
pub mod viewport;

use crate::renderer::model::Model;
use nalgebra::{Matrix4, Point2, Point3, Vector3};
use viewport::Viewport;

use self::camera::Camera;

#[derive(Clone, Copy, Debug)]
pub struct Color(u8, u8, u8);

/// A single point in 3D-space.
#[derive(Clone, Debug)]
pub struct Vertex {
    pub position: Point3<f32>,
}

impl Vertex {
    /// Applies model view projection and returns screen coordinates.
    fn project(&mut self, mvp_matrix: Matrix4<f32>, view_width: f32, view_height: f32) {
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
    screen_buffer: Vec<Vec<Color>>,
    camera: Camera,
    wireframe: bool,
}

impl Renderer {
    /// Constructs the renderer. [`Viewport`] must be passed to the constructor.
    pub fn new(viewport: Viewport) -> Renderer {
        let viewport_size = viewport.size();

        Renderer {
            screen_buffer: vec![
                vec![Color(0, 0, 0); viewport_size.1 as usize];
                viewport_size.0 as usize
            ],
            viewport,
            camera: Camera::new(
                viewport_size.0 as f32 / viewport_size.1 as f32,
                std::f32::consts::FRAC_PI_2,
                1.0,
                1000.0,
            ),
            wireframe: false,
        }
    }

    /// Constructs a wireframe renderer. [`Viewport`] must be passed to the constructor.
    pub fn new_wireframe(viewport: Viewport) -> Renderer {
        let viewport_size = viewport.size();

        Renderer {
            screen_buffer: vec![
                vec![Color(0, 0, 0); viewport_size.1 as usize];
                viewport_size.0 as usize
            ],
            viewport,
            camera: Camera::new(
                viewport_size.0 as f32 / viewport_size.1 as f32,
                std::f32::consts::FRAC_PI_2,
                1.0,
                1000.0,
            ),
            wireframe: true,
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
        self.screen_buffer =
            vec![vec![Color(0, 0, 0); viewport_size.1 as usize]; viewport_size.0 as usize];
    }

    /// Draws a [`Model`] to the screen buffer. calling [`render`] afterwards will render the model
    /// to the screen.
    ///
    /// [`render`]: #method.render
    pub fn draw_object(&mut self, model: &Model) {
        let model_view_matrix = model.model_matrix() * self.camera.view_matrix;
        let mvp_matrix = self.camera.projection.as_matrix() * model_view_matrix.to_homogeneous();

        let (width, height) = self.viewport.size();

        let light = Vector3::new(0.0, 0.0, -1.0);

        for face in &model.index_buffer {
            let mut v0 = model.vertex_at(face.indexes.0 - 1);
            let mut v1 = model.vertex_at(face.indexes.1 - 1);
            let mut v2 = model.vertex_at(face.indexes.2 - 1);

            let mut normal = (v2.position - v0.position).cross(&(v1.position - v0.position));

            normal = model.rotation * normal.normalize();
            let light_intensity = normal.dot(&light);
            let intensity = (light_intensity * 255.0) as u8;

            v0.project(mvp_matrix, width as f32, height as f32);
            v1.project(mvp_matrix, width as f32, height as f32);
            v2.project(mvp_matrix, width as f32, height as f32);

            if self.wireframe {
                Self::draw_line(self, &v0, &v1);
                Self::draw_line(self, &v1, &v2);
                Self::draw_line(self, &v2, &v0);
            } else if intensity > 0 {
                Self::draw_triangle(self, &v0, &v1, &v2, Color(intensity, intensity, intensity));
            }
        }
    }

    fn draw_triangle(&mut self, v0: &Vertex, v1: &Vertex, v2: &Vertex, color: Color) {
        let (bbmin, bbmax) = Self::bounding_box(v0, v1, v2);
        let p0 = Point2::new(v0.position.x, v0.position.y);
        let p1 = Point2::new(v1.position.x, v1.position.y);
        let p2 = Point2::new(v2.position.x, v2.position.y);

        for x in bbmin.0..bbmax.0 {
            for y in bbmin.1..bbmax.1 {
                let screen_point = Point2::new(x as f32, y as f32);

                let mut inside = true;
                inside &= 0.0 <= Self::edge_function(&p0, &p1, &screen_point);
                inside &= 0.0 <= Self::edge_function(&p1, &p2, &screen_point);
                inside &= 0.0 <= Self::edge_function(&p2, &p0, &screen_point);

                if inside {
                    Self::draw_pixel(self, x, y, color);
                }
            }
        }
    }

    fn edge_function(v0: &Point2<f32>, v1: &Point2<f32>, v2: &Point2<f32>) -> f32 {
        -((v2.x - v0.x) * (v1.y - v0.y) - (v2.y - v0.y) * (v1.x - v0.x))
    }

    fn bounding_box(v0: &Vertex, v1: &Vertex, v2: &Vertex) -> ((i16, i16), (i16, i16)) {
        let mut min = (0, 0);
        let mut max = (0, 0);

        min.0 = std::cmp::min(
            std::cmp::min(v0.position.x as i16, v1.position.x as i16),
            v2.position.x as i16,
        );
        min.1 = std::cmp::min(
            std::cmp::min(v0.position.y as i16, v1.position.y as i16),
            v2.position.y as i16,
        );
        max.0 = std::cmp::max(
            std::cmp::max(v0.position.x.ceil() as i16, v1.position.x.ceil() as i16),
            v2.position.x.ceil() as i16,
        );
        max.1 = std::cmp::max(
            std::cmp::max(v0.position.y.ceil() as i16, v1.position.y.ceil() as i16),
            v2.position.y.ceil() as i16,
        );

        (min, max)
    }

    fn draw_pixel(&mut self, x: i16, y: i16, color: Color) {
        let x_size = self.viewport.size().0 as i16;
        let y_size = self.viewport.size().1 as i16;
        if x >= 0 && x < x_size && y >= 0 && y < y_size {
            self.screen_buffer[x as usize][y as usize] = color;
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
            Self::draw_pixel(self, x1, y1, Color(255, 255, 255));
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
