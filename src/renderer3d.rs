use crossterm::{cursor, style::Print, terminal, QueueableCommand};
use nalgebra::Vector3;
use std::{
    error::Error,
    fs,
    io::{stdout, Stdout, Write},
};

struct Vertex {
    position: Vector3<f32>,
}

impl Vertex {
    fn get_transformed(&self, transform: &Transform) -> Vertex {
        let mut transformed_vertex = Vertex {
            position: Vector3::new(self.position.x, self.position.y, self.position.z),
        };
        transformed_vertex.rotate(transform.rotation);
        transformed_vertex.scale(transform.scale);
        transformed_vertex.translate(transform.position);

        transformed_vertex
    }
    fn translate(&mut self, (x, y, z): (f32, f32, f32)) {
        self.position.x += x;
        self.position.y += y;
        self.position.z += z;
    }

    fn scale(&mut self, (x, y, z): (f32, f32, f32)) {
        self.position.x *= x;
        self.position.y *= y;
        self.position.z *= z;
    }

    fn rotate(&mut self, (x, y, z): (f32, f32, f32)) {
        // around x axis
        self.position.y =
            x.to_radians().cos() * self.position.y - x.to_radians().sin() * self.position.z;
        self.position.z =
            x.to_radians().sin() * self.position.y + x.to_radians().cos() * self.position.z;
        // around y axis
        self.position.x =
            y.to_radians().cos() * self.position.x + y.to_radians().sin() * self.position.z;
        self.position.z = (-1.0 * y.to_radians().sin()) * self.position.x
            + y.to_radians().cos() * self.position.z;
        // around z axis
        self.position.x =
            z.to_radians().cos() * self.position.x - z.to_radians().sin() * self.position.y;
        self.position.y =
            z.to_radians().sin() * self.position.x + z.to_radians().cos() * self.position.y;
    }
}

struct Triangle {
    indexes: (usize, usize, usize),
}

struct Transform {
    position: (f32, f32, f32),
    scale: (f32, f32, f32),
    rotation: (f32, f32, f32),
}

impl Transform {
    fn new() -> Transform {
        Transform {
            position: (0.0, 0.0, 0.0),
            scale: (1.0, 1.0, 1.0),
            rotation: (0.0, 0.0, 0.0),
        }
    }
}

pub struct Object {
    vertex_buffer: Vec<Vertex>,
    index_buffer: Vec<Triangle>,
    transform: Transform,
}

impl Object {
    pub fn load_from_file(filename: &str) -> Result<Object, Box<dyn Error>> {
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
                    position: Vector3::new(
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

        Ok(Object {
            vertex_buffer,
            index_buffer,
            transform: Transform::new(),
        })
    }

    fn transform_and_get_vertex_at(&self, index: usize) -> Vertex {
        let transformed_vertex = self.vertex_buffer[index].get_transformed(&self.transform);

        transformed_vertex
    }

    pub fn translate(&mut self, x: f32, y: f32, z: f32) {
        self.transform.position = (x, y, z);
    }

    pub fn scale(&mut self, x: f32, y: f32, z: f32) {
        self.transform.scale = (x, y, z);
    }

    pub fn rotate(&mut self, x: f32, y: f32, z: f32) {
        self.transform.rotation = (x, y, z);
    }
}

struct Screen {
    screen_out: Stdout,
    size: (u16, u16),
}

impl Screen {
    fn new() -> Screen {
        let term_size = terminal::window_size().unwrap();
        let width = term_size.columns * 2;
        let height = term_size.rows * 4;

        Screen {
            screen_out: stdout(),
            size: (width, height),
        }
    }
}

pub struct Renderer {
    screen: Screen,
    pixel_grid: Vec<Vec<bool>>,
}

impl Renderer {
    pub fn new() -> Renderer {
        let screen = Screen::new();

        Renderer {
            pixel_grid: vec![vec![false; screen.size.1 as usize]; screen.size.0 as usize],
            screen,
        }
    }

    pub fn screen_size(&self) -> (u16, u16) {
        self.screen.size
    }

    pub fn draw_object(&mut self, object: &Object) {
        for triangle in &object.index_buffer {
            let v0 = object.transform_and_get_vertex_at(triangle.indexes.0 - 1);
            let v1 = object.transform_and_get_vertex_at(triangle.indexes.1 - 1);
            let v2 = object.transform_and_get_vertex_at(triangle.indexes.2 - 1);
            Self::draw_line(self, &v0, &v1);
            Self::draw_line(self, &v1, &v2);
            Self::draw_line(self, &v2, &v0);
        }
    }

    fn draw_pixel(&mut self, x: i16, y: i16) {
        let x_size = self.screen.size.0 as i16;
        let y_size = self.screen.size.1 as i16;
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

    /// Takes a 2 by 4 slice of the pixel buffer and converts it to a unicode braille character.
    /// https://en.wikipedia.org/wiki/Braille_Patterns#Identifying.2C_naming_and_ordering
    fn into_braille(tile: [[bool; 4]; 2]) -> char {
        let ordered_dots: [bool; 8] = [
            tile[0][0], tile[0][1], tile[0][2], tile[1][0], tile[1][1], tile[1][2], tile[0][3],
            tile[1][3],
        ];

        let mut pattern: u32 = 0;
        for (i, dot) in ordered_dots.into_iter().enumerate() {
            let dot: u32 = if dot { 1 } else { 0 };
            pattern += dot << i;
        }

        pattern += 0x2800; // Shift to get the correct unicode value
        char::from_u32(pattern).expect("Should generate a valid char")
    }

    /// Takes a 2d vector of pixels (on/off) and transforms it into a char vector of braille
    /// characters.
    fn draw_chars(screen: &mut Stdout, v: &Vec<Vec<bool>>) {
        screen.queue(cursor::MoveTo(0, 0)).unwrap();
        for row in (0..v[0].len()).rev().step_by(4) {
            for col in (0..v.len()).step_by(2) {
                let tile: [[bool; 4]; 2] = [
                    [
                        v[col][row],
                        v[col][row - 1],
                        v[col][row - 2],
                        v[col][row - 3],
                    ],
                    [
                        v[col + 1][row],
                        v[col + 1][row - 1],
                        v[col + 1][row - 2],
                        v[col + 1][row - 3],
                    ],
                ];
                screen.queue(Print(Self::into_braille(tile))).unwrap();
            }
        }
    }

    pub fn render(&mut self) {
        Self::draw_chars(&mut self.screen.screen_out, &self.pixel_grid);
        self.screen.screen_out.flush().unwrap();
    }

    pub fn clear(&mut self) {
        let term_size = self.screen.size;
        self.pixel_grid = vec![vec![false; term_size.1 as usize]; term_size.0 as usize];
    }
}

#[cfg(test)]
mod tests {
    use crate::renderer3d::Renderer;

    #[test]
    fn test_into_braille_all() {
        // o o
        // o o
        // o o
        // o o
        let all_dots = [[true, true, true, true], [true, true, true, true]];
        assert_eq!('\u{28FF}', Renderer::into_braille(all_dots));
    }

    #[test]
    fn test_into_braille_some() {
        // o o
        // _ _
        // _ o
        // o _
        let some_dots = [[true, false, false, true], [true, false, true, false]];
        assert_eq!('\u{2869}', Renderer::into_braille(some_dots));
    }

    #[test]
    fn test_into_braille_none() {
        // _ _
        // _ _
        // _ _
        // _ _
        let no_dots = [[false, false, false, false], [false, false, false, false]];
        assert_eq!('\u{2800}', Renderer::into_braille(no_dots));
    }
}
