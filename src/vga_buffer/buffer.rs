use core::{
    default::Default,
    fmt,
    ptr::{read_volatile, write_volatile},
};

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> Self {
        Self((background as u8) << 4 | (foreground as u8))
    }
}

impl Default for ColorCode {
    fn default() -> Self {
        Self::new(Color::White, Color::Black)
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(C)]
struct Char {
    ascii_character: u8,
    color_code: ColorCode,
}

#[repr(transparent)]
struct Buf {
    chars: [[Char; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct BufWriter {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buf,
}

impl Default for BufWriter {
    fn default() -> Self {
        BufWriter {
            column_position: 0,
            color_code: ColorCode::default(),
            buffer: unsafe { &mut *(0xb8000 as *mut Buf) },
        }
    }
}

impl BufWriter {
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // if we cant print just replace with ■
                _ => self.write_byte(0xfe),
            }
        }
    }
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;

                unsafe {
                    write_volatile(
                        &mut self.buffer.chars[row][col] as *mut Char,
                        Char {
                            ascii_character: byte,
                            color_code,
                        },
                    );
                }

                self.column_position += 1;
            }
        }
    }

    pub fn new_line(&mut self) -> () {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                unsafe {
                    let character = read_volatile(&mut self.buffer.chars[row][col] as *mut Char);
                    write_volatile(
                        &mut self.buffer.chars[row - 1][col] as *mut Char,
                        character,
                    );
                }
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = Char {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            unsafe {
                write_volatile(&mut self.buffer.chars[row][col] as *mut Char, blank);
            }
        }
    }
}

impl fmt::Write for BufWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}
