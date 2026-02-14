use core::fmt::{self, Write};

pub struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        unsafe {
            libc::write(1, s.as_ptr() as *const _, s.len());
        }
        Ok(())
    }
}

pub fn print(args: fmt::Arguments) {
    let mut stdout = Stdout;
    stdout.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        $crate::shared::io::print(format_args!($($arg)*));
    });
}

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => ({
        $crate::shared::io::print(format_args!($($arg)*));
        $crate::shared::io::print(format_args!("\n"));
    });
}
