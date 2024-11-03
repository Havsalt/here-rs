use anstyle::{AnsiColor, Color, Style};
use clap::builder::Styles;

pub const STYLES: Styles = Styles::styled()
    .usage(
        Style::new()
            .bold()
            .underline()
            .fg_color(Some(Color::Ansi(AnsiColor::Yellow))),
    )
    .header(
        Style::new()
            .bold()
            .underline()
            .fg_color(Some(Color::Ansi(AnsiColor::Yellow))),
    )
    .literal(Style::new().fg_color(Some(Color::Ansi(AnsiColor::BrightCyan))))
    .valid(
        Style::new()
            .bold()
            .fg_color(Some(Color::Ansi(AnsiColor::Green))),
    )
    .invalid(
        Style::new()
            .bold()
            .fg_color(Some(Color::Ansi(AnsiColor::Yellow))),
    )
    .error(
        Style::new()
            .bold()
            .fg_color(Some(Color::Ansi(AnsiColor::Red))),
    )
    .placeholder(Style::new().fg_color(Some(Color::Ansi(AnsiColor::BrightWhite))));
