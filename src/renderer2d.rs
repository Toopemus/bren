use std::io::{stdout, Stdout, Write};

use crossterm::{cursor, style::Print, terminal, QueueableCommand};

const CHAR_SPACE: u16 = 0;
const LINE_SPACE: u16 = 0;

struct TextLabel {
    message: String,
    position: (u16, u16),
}

pub struct Renderer {
    screen: Stdout,
    pixel_grid: Vec<Vec<bool>>,
    text_buffer: Vec<TextLabel>,
    term_size: (u16, u16),
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
            text_buffer: Vec::new(),
            term_size: (width, height),
        }
    }

    pub fn terminal_size(&self) -> (u16, u16) {
        self.term_size
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

    pub fn write_label(&mut self, row: u16, col: u16, message: &str) {
        let label = TextLabel {
            message: String::from(message),
            position: (row, col),
        };
        self.text_buffer.push(label);
    }

    pub fn draw_pixel(&mut self, x: i16, y: i16) {
        let x_size = self.term_size.0 as i16;
        let y_size = self.term_size.1 as i16;
        if x >= 0 && x < x_size && y >= 0 && y < y_size {
            self.pixel_grid[x as usize][y as usize] = true;
        }
    }

    /// Draws a line between (x1, y1) and (x2, y2) using Bresenham's algorithm.
    pub fn draw_line(&mut self, mut x1: i16, mut y1: i16, x2: i16, y2: i16) {
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

    /// Draws a circle centered at (cx, cy) with a radius r.
    /// https://www.computerenhance.com/p/efficient-dda-circle-outlines
    pub fn draw_circle(&mut self, cx: i16, cy: i16, r: i16) {
        let r2 = r + r;
        let mut x = r;
        let mut y = 0;
        let mut dy = -2;
        let mut dx = r2 + r2 - 4;
        let mut d = r2 - 1;

        while y <= x {
            Self::draw_pixel(self, cx - x, cy - y);
            Self::draw_pixel(self, cx + x, cy - y);
            Self::draw_pixel(self, cx - x, cy + y);
            Self::draw_pixel(self, cx + x, cy + y);
            Self::draw_pixel(self, cx - y, cy - x);
            Self::draw_pixel(self, cx + y, cy - x);
            Self::draw_pixel(self, cx - y, cy + x);
            Self::draw_pixel(self, cx + y, cy + x);

            d += dy;
            dy -= 4;
            y += 1;

            if d < 0 {
                d += dx;
                dx -= 4;
                x -= 1;
            }
        }
    }

    /// Draws a filled circle centered at (cx, cy) with a radius r.
    /// https://www.computerenhance.com/p/efficient-dda-circle-outlines
    pub fn draw_filled_circle(&mut self, cx: i16, cy: i16, r: i16) {
        let r2 = r + r;
        let mut x = r;
        let mut y = 0;
        let mut dy = -2;
        let mut dx = r2 + r2 - 4;
        let mut d = r2 - 1;

        while y <= x {
            // naive solution, just draws lines from center to edge
            Self::draw_line(self, cx - x, cy - y, cx, cy);
            Self::draw_line(self, cx + x, cy - y, cx, cy);
            Self::draw_line(self, cx - x, cy + y, cx, cy);
            Self::draw_line(self, cx + x, cy + y, cx, cy);
            Self::draw_line(self, cx - y, cy - x, cx, cy);
            Self::draw_line(self, cx + y, cy - x, cx, cy);
            Self::draw_line(self, cx - y, cy + x, cx, cy);
            Self::draw_line(self, cx + y, cy + x, cx, cy);

            d += dy;
            dy -= 4;
            y += 1;

            if d < 0 {
                d += dx;
                dx -= 4;
                x -= 1;
            }
        }
    }

    fn draw_text_buffer(screen: &mut Stdout, text_buffer: &Vec<TextLabel>) {
        for label in text_buffer {
            let pos = label.position;
            screen
                .queue(cursor::MoveTo(pos.0, pos.1))
                .unwrap()
                .queue(Print(&label.message))
                .unwrap();
        }
    }

    pub fn render(&mut self) {
        Self::draw_chars(&mut self.screen, &self.pixel_grid);
        Self::draw_text_buffer(&mut self.screen, &self.text_buffer);
        self.screen.flush().unwrap();
    }

    pub fn clear(&mut self) {
        let term_size = self.term_size;
        self.pixel_grid = vec![vec![false; term_size.1 as usize]; term_size.0 as usize];
        self.text_buffer = Vec::new();
    }
}

#[cfg(test)]
mod tests {
    use crate::renderer2d::Renderer;

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
