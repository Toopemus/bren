pub mod model;
pub mod viewport;

use crate::renderer::model::{Model, Transform};
use nalgebra::Point3;
use viewport::Viewport;

pub struct Vertex {
    position: Point3<f32>,
}

impl Vertex {
    pub fn get_transformed(&self, transform: &Transform) -> Vertex {
        let mut transformed_vertex = Vertex {
            position: Point3::new(self.position.x, self.position.y, self.position.z),
        };
        transformed_vertex.rotate(&transform);
        transformed_vertex.scale(&transform);
        transformed_vertex.translate(&transform);

        transformed_vertex
    }

    fn translate(&mut self, transform: &Transform) {
        self.position.x += transform.position.x;
        self.position.y += transform.position.y;
        self.position.z += transform.position.z;
    }

    fn scale(&mut self, transform: &Transform) {
        self.position.x *= transform.scale.x;
        self.position.y *= transform.scale.y;
        self.position.z *= transform.scale.z;
    }

    fn rotate(&mut self, transform: &Transform) {
        // TODO: is there an unnecessary copy here?
        self.position = transform.rotation * self.position;
    }
}

pub struct Face {
    indexes: (usize, usize, usize),
}

pub struct Renderer {
    viewport: Viewport,
    pixel_grid: Vec<Vec<bool>>,
}

impl Renderer {
    pub fn new() -> Renderer {
        let viewport = Viewport::new();
        let viewport_size = viewport.size();

        Renderer {
            pixel_grid: vec![vec![false; viewport_size.1 as usize]; viewport_size.0 as usize],
            viewport,
        }
    }

    pub fn with_viewport(viewport: Viewport) -> Renderer {
        let viewport_size = viewport.size();

        Renderer {
            pixel_grid: vec![vec![false; viewport_size.1 as usize]; viewport_size.0 as usize],
            viewport,
        }
    }

    pub fn viewport_size(&self) -> (u16, u16) {
        self.viewport.size()
    }

    pub fn draw_object(&mut self, object: &Model) {
        for face in object.index_buffer() {
            let v0 = object.transform_and_get_vertex_at(face.indexes.0 - 1);
            let v1 = object.transform_and_get_vertex_at(face.indexes.1 - 1);
            let v2 = object.transform_and_get_vertex_at(face.indexes.2 - 1);
            Self::draw_line(self, &v0, &v1);
            Self::draw_line(self, &v1, &v2);
            Self::draw_line(self, &v2, &v0);
        }
    }

    fn draw_pixel(&mut self, x: i16, y: i16) {
        let x_size = self.viewport.size().0 as i16;
        let y_size = self.viewport.size().1 as i16;
        if x >= 0 && x < x_size && y >= 0 && y < y_size {
            self.pixel_grid[x as usize][y as usize] = true;
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

    pub fn render(&mut self) {
        self.viewport.draw_chars(&self.pixel_grid);
    }

    pub fn clear(&mut self) {
        let viewport_size = self.viewport.size();
        self.pixel_grid = vec![vec![false; viewport_size.1 as usize]; viewport_size.0 as usize];
    }
}
