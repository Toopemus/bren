pub struct Renderer {
    pixel_buffer: Vec<Vec<bool>>,
    char_buffer: Vec<char>,
    term_size: (u16, u16),
}

impl Renderer {
    pub fn init() -> Self {
        let term_size = termion::terminal_size().unwrap();
        let width: usize = term_size.0.into();
        let height: usize = term_size.1.into();

        Self {
            pixel_buffer: vec![vec![false; height * 6]; width * 3],
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

    pub fn draw_pixel(&mut self, (x, y): (usize, usize)) {
        let x_size = self.term_size.0 as usize * 2;
        let y_size = self.term_size.1 as usize * 4;
        if x < x_size && y < y_size {
            self.pixel_buffer[x][y] = true;
        }
    }

    pub fn render(&mut self) {
        self.char_buffer = Self::into_char_buffer(&self.pixel_buffer);
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
