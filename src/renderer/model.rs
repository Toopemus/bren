use crate::renderer::{Face, Vertex};
use std::{error::Error, fs};

use nalgebra::{Isometry3, Point3, Translation3, UnitQuaternion};

/// Struct that manages individual 3D objects.
///
/// Models are represented as index and vertex buffers. Stores also the transformation data of the
/// object.
#[derive(Debug)]
pub struct Model {
    vertex_buffer: Vec<Vertex>,
    index_buffer: Vec<Face>,
    position: Translation3<f32>,
    rotation: UnitQuaternion<f32>,
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

    /// Getter for the model's index buffer.
    pub fn index_buffer(&self) -> &Vec<Face> {
        &self.index_buffer
    }

    /// Returns the vertex at index.
    pub fn vertex_at(&self, index: usize) -> Vertex {
        let transformed_vertex = self.vertex_buffer[index];

        transformed_vertex
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
