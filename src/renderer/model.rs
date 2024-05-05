use crate::renderer::{Face, Vertex};
use std::{error::Error, fs};

use nalgebra::{Isometry3, Point3, Translation3, UnitQuaternion};

/// Struct that manages individual 3D objects.
///
/// Models are represented as index and vertex buffers. Stores also the transformation data of the
/// object.
#[derive(Debug)]
pub struct Model {
    pub vertex_buffer: Vec<Vertex>,
    pub index_buffer: Vec<Face>,
    pub position: Translation3<f32>,
    pub rotation: UnitQuaternion<f32>,
}

impl Model {
    /// Loads and initializes a model from an .obj file wrapped in a Result.
    ///
    /// **This is mostly unfinished and incorrect, loads some files OK.**
    pub fn load_from_file(filename: &str) -> Result<Model, Box<dyn Error>> {
        let mut vertex_buffer: Vec<Vertex> = vec![];
        let mut index_buffer: Vec<Face> = vec![];
        let obj_file = fs::read_to_string(filename)?;

        for line in obj_file.lines() {
            let values: Vec<&str> = line.split_whitespace().collect();
            if values.is_empty() {
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
                let face = Face {
                    indexes: (values[1].parse()?, values[2].parse()?, values[3].parse()?),
                };
                index_buffer.push(face);
            }
        }

        Ok(Model {
            vertex_buffer,
            index_buffer,
            position: Translation3::new(0.0, 0.0, 0.0),
            rotation: UnitQuaternion::from_euler_angles(0.0, 0.0, 0.0),
        })
    }

    pub fn new_plane(div: i16, width: f32) -> Model {
        let mut vertex_buffer: Vec<Vertex> = vec![];
        let mut index_buffer: Vec<Face> = vec![];

        let div_length = width / div as f32;

        for x in 0..div + 1 {
            for y in 0..div + 1 {
                let offset = width / 2.0;
                vertex_buffer.push(Vertex {
                    position: Point3::new(
                        (x as f32 * div_length) - offset,
                        (y as f32 * div_length) - offset,
                        0.0,
                    ),
                });
            }
        }

        for y in 0..div {
            for x in 0..div {
                let curr_index = ((x + 1) + y * (div + 1)) as usize;
                index_buffer.push(Face {
                    indexes: (curr_index, curr_index + (div + 1) as usize, curr_index + 1),
                });
                index_buffer.push(Face {
                    indexes: (
                        curr_index + 1,
                        curr_index + (div + 1) as usize,
                        curr_index + 1 + (div + 1) as usize,
                    ),
                });
            }
        }

        Model {
            vertex_buffer,
            index_buffer,
            position: Translation3::new(0.0, 0.0, 0.0),
            rotation: UnitQuaternion::from_euler_angles(0.0, 0.0, 0.0),
        }
    }

    /// Returns the vertex at index.
    pub fn vertex_at(&self, index: usize) -> Vertex {
        self.vertex_buffer[index].clone()
    }

    /// Calculates model matrix from translation and rotation.
    pub fn model_matrix(&self) -> Isometry3<f32> {
        let model_matrix: Isometry3<f32> = self.position * self.rotation;

        model_matrix
    }

    /// Translates the object in 3D space.
    pub fn translate(&mut self, x: f32, y: f32, z: f32) {
        self.position = Translation3::new(x, y, z);
    }

    /// Rotates the object by x, y, and z degrees along the respective axis.
    pub fn rotate(&mut self, x: f32, y: f32, z: f32) {
        self.rotation =
            UnitQuaternion::from_euler_angles(x.to_radians(), y.to_radians(), z.to_radians());
    }
}
