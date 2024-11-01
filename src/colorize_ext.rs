use colored::{ColoredString, Colorize};

pub trait ExtendedColorize: Colorize {
    fn crimson(self) -> ColoredString;
    fn salmon(self) -> ColoredString;
    fn gray(self) -> ColoredString;
}

// Implement the new trait for any type that implements Colorize
impl<T: Colorize> ExtendedColorize for T {
    fn crimson(self) -> ColoredString {
        self.truecolor(220, 20, 60)
    }
    fn salmon(self) -> ColoredString {
        self.truecolor(250, 128, 128) // Using Magenta as a close approximation
    }
    fn gray(self) -> ColoredString {
        self.truecolor(128, 128, 128)
    }
}
