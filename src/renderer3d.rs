use std::io::{stdout, Stdout, Write};

use crossterm::{cursor, style::Print, terminal, QueueableCommand};

const CHAR_SPACE: u16 = 0;
const LINE_SPACE: u16 = 0;

pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub struct Renderer {
    screen: Stdout,
    pixel_grid: Vec<Vec<bool>>,
    term_size: (u16, u16),
    vertex_buffer: Vec<Vertex>,
    index_buffer: Vec<usize>,
}

impl Renderer {
    pub fn new() -> Renderer {
        let screen = stdout();
        let term_size = terminal::window_size().unwrap();
        let width = term_size.columns * (2 + CHAR_SPACE);
        let height = term_size.rows * (4 + LINE_SPACE);

        Renderer {
            screen,
            pixel_grid: vec![vec![false; height as usize]; width as usize],
            term_size: (width, height),
            vertex_buffer: vec![],
            index_buffer: vec![],
        }
    }

    pub fn terminal_size(&self) -> (u16, u16) {
        self.term_size
    }

    /// Temporary setters
    pub fn set_vertex_buffer(&mut self, v: &mut Vec<Vertex>) {
        self.vertex_buffer.append(v);
    }

    /// Temporary setters
    pub fn set_index_buffer(&mut self, v: &mut Vec<usize>) {
        self.index_buffer.append(v);
    }

    pub fn draw(&mut self) {
        for i in (0..self.index_buffer.len()).step_by(3) {
            Self::draw_line(self, self.index_buffer[i], self.index_buffer[i + 1]);
            Self::draw_line(self, self.index_buffer[i + 1], self.index_buffer[i + 2]);
            Self::draw_line(self, self.index_buffer[i + 2], self.index_buffer[i]);
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
        for row in (0..v[0].len()).rev().step_by(4 + LINE_SPACE as usize) {
            for col in (0..v.len()).step_by(2 + CHAR_SPACE as usize) {
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

    pub fn draw_pixel(&mut self, x: i16, y: i16) {
        let x_size = self.term_size.0 as i16;
        let y_size = self.term_size.1 as i16;
        if x >= 0 && x < x_size && y >= 0 && y < y_size {
            self.pixel_grid[x as usize][y as usize] = true;
        }
    }

    pub fn draw_line(&mut self, i1: usize, i2: usize) {
        let v1 = &self.vertex_buffer[i1];
        let v2 = &self.vertex_buffer[i2];
        let mut x1 = v1.x;
        let mut y1 = v1.y;
        let x2 = v2.x;
        let y2 = v2.y;
        let dx = (x2 - x1).abs();
        let dy = -(y2 - y1).abs();
        let sx = if x1 < x2 { 1.0 } else { -1.0 };
        let sy = if y1 < y2 { 1.0 } else { -1.0 };
        let mut err = dy + dx;
        let mut err2;

        loop {
            // casting f32 as i16, could lead to unexpected values. Then again, such values won't
            // fit the screen
            Self::draw_pixel(self, x1 as i16, y1 as i16);
            if x1 == x2 && y1 == y2 {
                break;
            }
            err2 = 2.0 * err;
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
        Self::draw_chars(&mut self.screen, &self.pixel_grid);
        self.screen.flush().unwrap();
    }

    pub fn clear(&mut self) {
        let term_size = self.term_size;
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
