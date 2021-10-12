use core::ops::{Deref, DerefMut};
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
    DarkGrey = 8,
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
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

impl Deref for ScreenChar {
    type Target = ScreenChar;
    fn deref(&self) -> &Self::Target {
        return self
    }
}

impl DerefMut for ScreenChar {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return self;
    }
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

use volatile::{Volatile, access::ReadWrite};

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar, ReadWrite>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    row_position: usize,
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer
}

impl Writer {
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => {
                    self.write_byte(byte);
                },
                _ => self.write_byte(0xfe),
            }
        }
    }

    //make sure default pos is valid
    pub fn write(&mut self, byte: u8) {
        self.buffer.chars[self.row_position][self.column_position].write(ScreenChar {
            ascii_character: byte as u8,
            color_code: self.color_code
        });
    }

    fn next_pos(&mut self) {
        self.column_position += 1;
        if self.column_position >= BUFFER_WIDTH {
            self.column_position = 0;
            self.row_position += 1;
            if self.row_position >= BUFFER_HEIGHT {
                self.new_line()
            }
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => {
                self.write(' ' as u8);
                self.new_line();
            }
            byte => {
                self.write(byte as u8);
                self.next_pos();
            }
        }
    }

    fn new_line(&mut self) {
        self.column_position = 0;
        if self.row_position < BUFFER_HEIGHT - 1 {
            self.row_position += 1;
        //reached the bottom of the screen
        } else {
            for j in 0..BUFFER_WIDTH {
                for i in 0..BUFFER_HEIGHT - 1 {
                    self.buffer.chars[i][j].write(self.buffer.chars[i+1][j].read());
                }
                self.write(' ' as u8);
            }
        }
    }
}

use core::fmt;

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        return Ok(());
    }
}

use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        row_position: 0,
        column_position: 0,
        color_code: ColorCode::new(Color::White, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

#[macro_export]
macro_rules! print {
    ($($args:tt)*) => ($crate::vga_buffer::_print(format_args!($($args)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;
    //closure
    interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    });
}