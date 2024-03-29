use crate::renderer::{Triangle, Vertex};
use std::{error::Error, fs};

use nalgebra::{Point3, Rotation3, Scale3, Translation3};
pub struct Transform {
    pub position: Translation3<f32>,
    pub scale: Scale3<f32>,
    pub rotation: Rotation3<f32>,
}

impl Transform {
    fn new() -> Transform {
        Transform {
            position: Translation3::new(0.0, 0.0, 0.0),
            scale: Scale3::new(1.0, 1.0, 1.0),
            rotation: Rotation3::from_euler_angles(0.0, 0.0, 0.0),
        }
    }
}

pub struct Model {
    vertex_buffer: Vec<Vertex>,
    index_buffer: Vec<Triangle>,
    transform: Transform,
}

impl Model {
    pub fn load_from_file(filename: &str) -> Result<Model, Box<dyn Error>> {
        let mut vertex_buffer: Vec<Vertex> = vec![];
        let mut index_buffer: Vec<Triangle> = vec![];
        let obj_file = fs::read_to_string(filename)?;

        for line in obj_file.lines() {
            let values: Vec<&str> = line.split_whitespace().collect();
            if values.len() == 0 {
                continue;
            }
            if values[0] == "v" {
                // vertex data
                let vertex = Vertex {
                    position: Point3::new(
                        values[1].parse()?,
                        values[2].parse()?,
                        values[3].parse()?,
                    ),
                };
                vertex_buffer.push(vertex);
            } else if values[0] == "f" {
                // index data
                let triangle = Triangle {
                    indexes: (values[1].parse()?, values[2].parse()?, values[3].parse()?),
                };
                index_buffer.push(triangle);
            }
        }

        Ok(Model {
            vertex_buffer,
            index_buffer,
            transform: Transform::new(),
        })
    }

    pub fn index_buffer(&self) -> &Vec<Triangle> {
        &self.index_buffer
    }

    pub fn transform_and_get_vertex_at(&self, index: usize) -> Vertex {
        let transformed_vertex = self.vertex_buffer[index].get_transformed(&self.transform);

        transformed_vertex
    }

    pub fn translate(&mut self, x: f32, y: f32, z: f32) {
        self.transform.position = Translation3::new(x, y, z);
    }

    pub fn scale(&mut self, x: f32, y: f32, z: f32) {
        self.transform.scale = Scale3::new(x, y, z);
    }

    pub fn rotate(&mut self, x: f32, y: f32, z: f32) {
        self.transform.rotation =
            Rotation3::from_euler_angles(x.to_radians(), y.to_radians(), z.to_radians());
    }
}
