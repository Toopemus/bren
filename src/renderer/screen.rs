use crossterm::{cursor, style::Print, terminal, QueueableCommand};
use std::io::{stdout, Stdout, Write};

pub struct Screen {
    screen_out: Stdout,
    size: (u16, u16),
}

impl Screen {
    pub fn new() -> Screen {
        let term_size = terminal::window_size().unwrap();
        let width = term_size.columns * 2;
        let height = term_size.rows * 4;

        Screen {
            screen_out: stdout(),
            size: (width, height),
        }
    }

    pub fn get_size(&self) -> (u16, u16) {
        self.size
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
    pub fn draw_chars(&mut self, v: &Vec<Vec<bool>>) {
        self.screen_out.queue(cursor::MoveTo(0, 0)).unwrap();
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
                self.screen_out
                    .queue(Print(Self::into_braille(tile)))
                    .unwrap();
            }
        }
        self.screen_out.flush().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::Screen;
    #[test]
    fn test_into_braille_all() {
        // o o
        // o o
        // o o
        // o o
        let all_dots = [[true, true, true, true], [true, true, true, true]];
        assert_eq!('\u{28FF}', Screen::into_braille(all_dots));
    }

    #[test]
    fn test_into_braille_some() {
        // o o
        // _ _
        // _ o
        // o _
        let some_dots = [[true, false, false, true], [true, false, true, false]];
        assert_eq!('\u{2869}', Screen::into_braille(some_dots));
    }

    #[test]
    fn test_into_braille_none() {
        // _ _
        // _ _
        // _ _
        // _ _
        let no_dots = [[false, false, false, false], [false, false, false, false]];
        assert_eq!('\u{2800}', Screen::into_braille(no_dots));
    }
}
