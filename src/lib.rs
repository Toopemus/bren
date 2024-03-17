pub struct Renderer {
    pixel_grid: Vec<Vec<bool>>,
    char_buffer: Vec<char>,
    term_size: (u16, u16),
}

impl Renderer {
    pub fn init() -> Self {
        let term_size = termion::terminal_size().unwrap();
        let width: usize = term_size.0.into();
        let height: usize = term_size.1.into();

        Self {
            pixel_grid: vec![vec![false; height * 6]; width * 3],
            char_buffer: Vec::new(),
            term_size,
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
    fn into_char_buffer(v: &Vec<Vec<bool>>) -> Vec<char> {
        let mut char_buffer: Vec<char> = Vec::new();
        for row in (0..v[0].len()).step_by(6) {
            for col in (0..v.len()).step_by(3) {
                let tile: [[bool; 4]; 2] = [
                    [
                        v[col][row],
                        v[col][row + 1],
                        v[col][row + 2],
                        v[col][row + 3],
                    ],
                    [
                        v[col + 1][row],
                        v[col + 1][row + 1],
                        v[col + 1][row + 2],
                        v[col + 1][row + 3],
                    ],
                ];
                char_buffer.push(Self::into_braille(tile))
            }
        }
        char_buffer
    }

    pub fn draw_pixel(&mut self, x: i16, y: i16) {
        let x_size = self.term_size.0 as i16 * 3;
        let y_size = self.term_size.1 as i16 * 6;
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

    /// Draws a line centered at (cx, cy) with a radius r.
    /// https://www.computerenhance.com/p/efficient-dda-circle-outlines
    pub fn draw_circle(&mut self, cx: i16, cy: i16, r: i16) {
        let r2 = r + r;
        let mut x = r;
        let mut y = 0;
        let mut dy = -2;
        let mut dx = r2 + r2 - 4;
        let mut d = r2 - 1;

        while y <= x {
            Self::draw_pixel(self, cy - y, cx - x);
            Self::draw_pixel(self, cy - y, cx + x);
            Self::draw_pixel(self, cy + y, cx - x);
            Self::draw_pixel(self, cy + y, cx + x);
            Self::draw_pixel(self, cy - x, cx - y);
            Self::draw_pixel(self, cy - x, cx + y);
            Self::draw_pixel(self, cy + x, cx - y);
            Self::draw_pixel(self, cy + x, cx + y);

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

    pub fn render(&mut self) {
        self.char_buffer = Self::into_char_buffer(&self.pixel_grid);
        let output = String::from_iter(&self.char_buffer);
        print!("{}", output);
    }

    pub fn clear(&self) {
        print!("{}", termion::clear::All);
    }
}

#[cfg(test)]
mod tests {
    use crate::Renderer;

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

    #[test]
    fn test_into_char_buffer() {
        let pixel_buffer = vec![
            vec![true, true, true, true],
            vec![true, true, true, true],
            vec![true, false, false, true],
            vec![true, false, true, false],
            vec![false, false, false, false],
            vec![false, false, false, false],
        ];
        assert_eq!(
            vec!['\u{28FF}', '\u{2869}', '\u{2800}'],
            Renderer::into_char_buffer(&pixel_buffer)
        )
    }
}
