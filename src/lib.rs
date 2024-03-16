pub struct Renderer {
    pixel_buffer: Vec<Vec<bool>>,
    char_buffer: Vec<Vec<char>>,
    term_size: (u16, u16),
}

impl Renderer {
    pub fn init() -> Self {
        let term_size = termion::terminal_size().unwrap();
        let width: usize = term_size.0.into();
        let height: usize = term_size.1.into();

        Self {
            pixel_buffer: vec![vec![false; height * 4]; width * 2],
            char_buffer: vec![vec!['⣿'; height]; width],
            term_size,
        }
    }

    pub fn terminal_size(&self) -> (u16, u16) {
        self.term_size
    }

    /// Takes a 2 by 4 slice of the pixel buffer and converts it to a unicode
    /// braille character.
    /// https://en.wikipedia.org/wiki/Braille_Patterns#Identifying.2C_naming_and_ordering
    fn into_braille(tile: &[[bool; 4]; 2]) -> char {
        let ordered_dots: [bool; 8] = [
            tile[0][0], tile[0][1], tile[0][2], tile[1][0], tile[1][1], tile[1][2], tile[0][3],
            tile[1][3],
        ];

        let mut pattern: u32 = 0;
        for (i, dot) in ordered_dots.into_iter().enumerate() {
            pattern += (dot as u32) << i;
        }

        pattern += 0x2800; // Shift to get the correct unicode value
        char::from_u32(pattern).expect("Should generate a valid char")
    }

    fn into_char_buffer(pixel_buffer: Vec<Vec<bool>>) {
        todo!()
    }

    pub fn render(&self) {
        let mut output = String::new();
        output.extend(self.char_buffer.iter().flatten());
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
        assert_eq!('\u{28FF}', Renderer::into_braille(&all_dots));
    }

    #[test]
    fn test_into_braille_some() {
        // o o
        // _ _
        // _ o
        // o _
        let some_dots = [[true, false, false, true], [true, false, true, false]];
        assert_eq!('\u{2869}', Renderer::into_braille(&some_dots));
    }

    #[test]
    fn test_into_braille_none() {
        // _ _
        // _ _
        // _ _
        // _ _
        let no_dots = [[false, false, false, false], [false, false, false, false]];
        assert_eq!('\u{2800}', Renderer::into_braille(&no_dots));
    }
}
