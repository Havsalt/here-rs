use colored::{ColoredString, Colorize};

pub trait ColorizeExt: Colorize {
    fn crimson(self) -> ColoredString;
    fn salmon(self) -> ColoredString;
    fn gray(self) -> ColoredString;
    fn orange(self) -> ColoredString;
}

// Implement the new trait for any type that implements `Colorize`
impl<T: Colorize> ColorizeExt for T {
    fn crimson(self) -> ColoredString {
        self.truecolor(220, 20, 60)
    }
    fn salmon(self) -> ColoredString {
        self.truecolor(250, 128, 128)
    }
    fn gray(self) -> ColoredString {
        self.truecolor(128, 128, 128)
    }
    fn orange(self) -> ColoredString {
        self.truecolor(255, 165, 0)
    }
}
