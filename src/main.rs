use std::path::PathBuf;
use std::process::ExitCode;
use std::{env::current_dir, fs};

use clap::Parser;
use cli_clipboard;
use colored::Colorize;
use enigo::{Enigo, Key, Keyboard, Settings};
use havsalt_clap_styles::STYLES;
use path_clean::PathClean;

mod colorize_ext;
use colorize_ext::ColorizeExt;

mod fetch;

#[derive(Parser, Debug)]
#[command(
    name = "here",
    version,
    about = "Copies the current working directory to clipboard",
    long_about = None,
    styles = STYLES
)]
struct Args {
    #[arg(default_value = ".", value_name = "PATH SEGMENT / PROGRAM SEARCH")]
    segment_or_name: String,
    #[arg(short = 'f', long = "folder", help = "Get folder component of result")]
    folder_component: bool,
    #[arg(short, long = "from-where", help = "Use `where` command to search")]
    where_search: bool,
    #[arg(
        short = 'd',
        long,
        help = "Set current working directory to result (schedules writing)"
    )]
    change_directory: bool,
    #[arg(short = 'e', long, help = "Escape backslashes (\\ -> \\\\)")]
    escape_backslash: bool,
    #[arg(short = 'q', long, help = "Wrap result in double quotes")]
    wrap_quote: bool,
    #[arg(short = 'r', long, help = "Resolve symlink path")]
    resolve_symlink: bool,
    #[arg(short = 'n', long, help = "Prevent copy to clipboard")]
    no_copy: bool,
    #[arg(short = 'c', long, help = "Suppress color")]
    no_color: bool,
    #[arg(
        long = "posix",
        conflicts_with = "no_posix_style",
        help = "Force posix style path"
    )]
    posix_style: bool,
    #[arg(
        long = "no-posix",
        conflicts_with = "posix_style",
        help = "Prevent posix style path"
    )]
    no_posix_style: bool,
    #[arg(
        long = "select-first",
        requires = "where_search",
        help = "Select first option if multiresult (when: -w/--from-where)"
    )]
    select_first_option: bool,
}

fn main() -> ExitCode {
    let args = Args::parse();

    // Select where to extract the path from
    let mut path = if args.where_search {
        if args.segment_or_name == "." {
            if args.no_color {
                println!("[Error] Argument [PROGRAM SEARCH] cannot be \".\"")
            } else {
                let label = "[Error]".crimson();
                let msg1 = "Argument".gray();
                let arg = "[PROGRAM SEARCH]".white();
                let msg2 = "cannot be".gray();
                let program = "\".\"".white();
                println!("{label} {msg1} {arg} {msg2} {program}");
            }
            return ExitCode::FAILURE;
        }
        match fetch::string_path_from_search(&args.segment_or_name, &args.select_first_option) {
            Ok(string_path) => {
                if string_path.is_empty() {
                    let label = "[Error]".crimson();
                    let msg = "Could not find".gray();
                    let program = format!("\"{}\"", args.segment_or_name).white();
                    println!("{label} {msg} {program}");
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

    if args.resolve_symlink {
        if path.exists() {
            if path.is_symlink() {
                path = fs::read_link(path).expect("path is symlink that exists");
            } else {
                let label = "[Warning]".orange();
                let msg1 = "Path".gray();
                let value = path.display().to_string().bright_white();
                let msg2 = "is".gray();
                let msg3 = "not a symlink".cyan();
                println!("{label} {msg1} {value} {msg2} {msg3}");
            }
        } else {
            let label = "[Warning]".orange();
            let msg1 = "Symlink".gray();
            let value = path.display().to_string().bright_white();
            let msg2 = "does".gray();
            let msg3 = "not exist".cyan();
            println!("{label} {msg1} {value} {msg2} {msg3}");
        }
    }

    if args.folder_component & path.is_file() {
        path = path
            .parent()
            .expect("both current path and parent path is valid")
            .to_path_buf()
    }

    // Apply styling options
    let mut visual = path.display().to_string();

    if args.posix_style {
        visual = visual.replace("\\", "/")
    } else if args.no_posix_style {
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
