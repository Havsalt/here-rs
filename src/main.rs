use std::env::current_dir;
use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser;
use cli_clipboard;
use colored::Colorize;
use path_clean::PathClean;
use enigo::{Enigo, Key, Keyboard, Settings};

mod colorize_ext;
use colorize_ext::ColorizeExt;

mod util;
use util::string_path_from_search;

mod styles;
use styles::STYLES;

#[derive(Parser, Debug)]
#[command(
    version,
    about = "Copies the current working directory to clipboard",
    long_about = None,
    styles = STYLES
)]
struct Args {
    #[arg(default_value = ".", value_name = "PATH SEGMENT / PROGRAM SEARCH")]
    segment_or_name: String,
    #[arg(short, long = "folder", help = "Get folder component of result")]
    folder_component: bool,
    #[arg(short, long = "from-where", help = "Use `where` command to search")]
    where_search: bool,
    #[arg(
        short = 'd',
        long,
        help = "Set current working directory to result (schedules writing)"
    )]
    change_directory: bool,
    #[arg(short, long, help = "Escape backslashes (\\ -> \\\\)")]
    escape_backslash: bool,
    #[arg(short = 'q', long, help = "Wrap result in double quotes")]
    wrap_quote: bool,
    #[arg(short = 'n', long, help = "Prevent copy to clipboard")]
    no_copy: bool,
    #[arg(short = 'c', long, help = "Suppress color")]
    no_color: bool,
    #[arg(
        long = "posix",
        conflicts_with = "no_posix",
        help = "Force posix style path"
    )]
    posix: bool,
    #[arg(
        long = "no-posix",
        conflicts_with = "posix",
        help = "Prevent posix style path"
    )]
    no_posix: bool,
    #[arg(
        long = "select-first",
        help = "Select first option if multiresult (when: -w/--from-where)"
    )]
    select_first_option: bool,
}

fn main() -> ExitCode {
    let args = Args::parse();

    // Select where to extract the path from
    let mut path = if args.where_search {
        if args.segment_or_name == "." {
            let error = "[Error]".crimson();
            let msg1 = "Argument".gray();
            let arg = "[PROGRAM SEARCH]".white();
            let msg2 = "cannot be".gray();
            let program = "\".\"".white();
            println!("{error} {msg1} {arg} {msg2} {program}");
            return ExitCode::FAILURE;
        }
        match string_path_from_search(&args.segment_or_name, &args.select_first_option) {
            Ok(string_path) => {
                if string_path.is_empty() {
                    let error = "[Error]".crimson();
                    let msg = "Could not find".gray();
                    let program = format!("\"{}\"", args.segment_or_name).white(); // White
                    println!("{error} {msg} {program}");
                    return ExitCode::FAILURE;
                } else {
                    PathBuf::from(string_path)
                }
            }
            Err(exit_code) => return exit_code,
        }
    } else {
        current_dir()
            .expect("cwd was found and have permission")
            .join(&args.segment_or_name)
    };

    // Apply path manipulations
    path = path.clean();

    if args.folder_component & path.is_file() {
        path = path
            .parent()
            .expect("both current path and parent path is valid")
            .to_path_buf()
    }

    // Apply styling options
    let mut visual = path.display().to_string();

    if args.posix {
        visual = visual.replace("\\", "/")
    } else if args.no_posix {
        visual = visual.replace("/", "\\")
    }

    if args.wrap_quote {
        visual = format!("\"{}\"", visual)
    }

    if args.escape_backslash {
        visual = visual.replace("\\", "\\\\")
    }

    // Final actions
    if !args.no_copy {
        cli_clipboard::set_contents(visual.to_owned()).expect("clipboard opened successfully");
    }
    if args.no_color {
        println!("{}", visual);
    } else {
        println!("{}", visual.salmon());
    }
    if args.change_directory {
        let mut keyboard = Enigo::new(&Settings::default()).unwrap();
        let quoted_path = format!("\"{}\"", path.display());
        let command = format!("cd {quoted_path}");
        let _ = keyboard.text(&command);
        let _ = keyboard.key(Key::Return, enigo::Direction::Press);
    }

    ExitCode::SUCCESS
}
