use crate::renderer::{Face, Vertex};
use std::{error::Error, fs};

use nalgebra::{Point3, Rotation3, Scale3, Translation3};

/// Stores transformation data that is applied to vertices: position, scale, rotation.
#[derive(Debug)]
pub struct Transform {
    /// Position in relation to the origin.
    pub position: Translation3<f32>,
    /// Scaling factors for x, y, and z dimensions.
    pub scale: Scale3<f32>,
    /// Rotation around the x, y, and z axis in radians.
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

/// Struct that manages individual 3D objects.
///
/// Models are represented as index and vertex buffers. Stores also the transformation data of the
/// object.
#[derive(Debug)]
pub struct Model {
    vertex_buffer: Vec<Vertex>,
    index_buffer: Vec<Face>,
    transform: Transform,
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
            transform: Transform::new(),
        })
    }

    /// Getter for the model's index buffer.
    pub fn index_buffer(&self) -> &Vec<Face> {
        &self.index_buffer
    }

    /// Applies model transform and returns the vertex at index.
    pub fn transform_and_get_vertex_at(&self, index: usize) -> Vertex {
        let transformed_vertex = self.vertex_buffer[index].get_transformed(&self.transform);

        transformed_vertex
    }

    /// Translates the object in 3D space.
    pub fn translate(&mut self, x: f32, y: f32, z: f32) {
        self.transform.position = Translation3::new(x, y, z);
    }

    /// Scales the object along the x, y, and z axis.
    pub fn scale(&mut self, x: f32, y: f32, z: f32) {
        self.transform.scale = Scale3::new(x, y, z);
    }

    /// Rotates the object by x, y, and z degrees along the respective axis.
    pub fn rotate(&mut self, x: f32, y: f32, z: f32) {
        self.transform.rotation =
            Rotation3::from_euler_angles(x.to_radians(), y.to_radians(), z.to_radians());
    }
}
