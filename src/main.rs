use std::io;
use std::path::PathBuf;
use std::process::ExitCode;
use std::{env::current_dir, fs};

use clap::{CommandFactory, Parser};
use clap_complete::{generate, Shell};
use clap_markdown::print_help_markdown;
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
    #[arg(long = "completions",
        value_name = "SHELL",
        help = "Generate completions for given shell",
        conflicts_with_all = [
            "folder_component",
            "where_search",
            "change_directory",
            "escape_backslash",
            "wrap_quote",
            "resolve_symlink",
            "no_copy",
            "no_color",
            "posix_style",
            "no_posix_style",
            "select_first_option",
            "generate_markdown",
            "segment_or_name"
        ]
    )]
    pub generate_completions: Option<Shell>,
    #[arg(
        long = "markdown",
        help = "Generate markdown help page",
        conflicts_with_all = [
            "folder_component",
            "where_search",
            "change_directory",
            "escape_backslash",
            "wrap_quote",
            "resolve_symlink",
            "no_copy",
            "no_color",
            "posix_style",
            "no_posix_style",
            "select_first_option",
            "generate_completions",
            "segment_or_name"
        ]
    )]
    pub generate_markdown: bool,
}

fn main() -> ExitCode {
    let args = Args::parse();

    // Generate completions and markdown help page
    if let Some(shell) = args.generate_completions {
        generate(
            shell,
            &mut Args::command(),
            Args::command().get_name().to_string(),
            &mut io::stdout(),
        );
    }
    if args.generate_markdown {
        print_help_markdown::<Args>();
    }
    // Abort after printing completions or markdown help page
    if args.generate_completions.is_some() || args.generate_markdown {
        return ExitCode::SUCCESS;
    }

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
