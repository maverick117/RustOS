#![macro_use]


use core::fmt;
use spin::Mutex;

extern "C" {
    fn clear_console();
    fn print_char(txt:u16);
}

pub struct VGAWriter {
    pub fc : Colors,
    pub bc : Colors,
}

impl VGAWriter {
    pub fn new(fc: Colors, bc: Colors) -> VGAWriter {
        VGAWriter {
            fc:fc,
            bc:bc,
        }
    }

    pub fn set_foreground_color(&mut self, color: Colors){
        self.fc = color;
    }

    pub fn set_background_color(&mut self, color: Colors){
        self.bc = color;
    }

    pub fn clear_screen(&self){
        unsafe{clear_console()};
    }
}

impl fmt::Write for VGAWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result{
        print_str_color(s,self.bc,self.fc);
        Ok(())
    }

}

pub static vga: Mutex<VGAWriter> = Mutex::new(VGAWriter {
    fc : Colors::LightBlue,
    bc : Colors::Black,
});

#[allow(dead_code)]
#[repr(u8)]
#[derive(Copy,Clone,Debug)]
pub enum Colors{
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

#[inline]
fn print_str_color(s : &str, bkgc: Colors, txtc: Colors){
    for ch in s.chars(){
        unsafe{
            print_char(ch as u8 as u16 | (bkgc as u16) << 12 | (txtc as u16) << 8);
        }
    }
}

pub fn print(args: fmt::Arguments) {
    use core::fmt::Write;
    vga.lock().write_fmt(args).unwrap()
}

macro_rules! print {
    ($($arg:tt)*) => ({
        $crate::vga_console::print(format_args!($($arg)*));
    })
}

macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}