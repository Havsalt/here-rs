use clap::Parser;
use cli_clipboard;
use colored::Colorize;
use core::str;
use inquire::Select;
use path_clean::PathClean;
use std::env::current_dir;
use std::path::PathBuf;
use std::process::{Command, ExitCode};
use std::str::FromStr;

#[derive(Parser, Debug)]
#[command(
    version,
    about = "Copies the current working directory to clipboard",
    long_about = None
)]
struct Args {
    #[arg(default_value = ".", value_name = "PATH SEGMENT / SEARCH PROGRAM")]
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
}

fn main() -> ExitCode {
    let args = Args::parse();

    // Select where to extract the raw path from
    let path = if args.where_search {
        // TODO: Fix process not starting correctly
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .arg("/C")
                .arg(format!("where {}", args.segment_or_name))
                .output()
                .expect("'where' command found path to program/script on Windows")
        } else {
            todo!("implement for Linux")
        };
        let str_result = str::from_utf8(&output.stdout)
            .expect("path string is valid UTF-8")
            .trim()
            .replace("\r", "")
            .leak();
        // Check for multiresult
        let str_path = if str_result.contains("\n") {
            let options: Vec<&str> = str_result.split("\n").collect();
            let select = Select::new("Select a path:", options);
            match select.prompt_skippable() {
                Ok(answer) => match answer {
                    Some(str_answer) => str_answer,
                    None => return ExitCode::FAILURE,
                },
                Err(_) => return ExitCode::FAILURE,
            }
        } else {
            str_result
        };
        PathBuf::from_str(str_path).expect("valid path string format")
    } else {
        let path = current_dir().expect("cwd was found and have permission");
        path.join(&args.segment_or_name)
    };

    // Apply path manipulations
    let path = path.clean();
    let path = if args.folder_component & path.is_file() {
        path.parent()
            .expect("both current path and parent path is valid")
            .to_path_buf()
    } else {
        path
    };

    // Apply styling options
    let visual = path.display().to_string().to_string();

    let visual = if args.posix {
        visual.replace("\\", "/")
    } else if args.no_posix {
        visual.replace("/", "\\")
    } else {
        visual
    };

    let visual = if args.wrap_quote {
        format!("\"{}\"", visual)
    } else {
        visual
    };

    let visual = if args.escape_backslash {
        visual.replace("\\", "\\\\")
    } else {
        visual
    };

    // Final actions
    if !args.no_copy {
        cli_clipboard::set_contents(visual.to_owned()).expect("clipboard opened successfully");
    }
    if args.no_color {
        println!("{}", visual);
    } else {
        println!("{}", visual.truecolor(250, 128, 114));
    }

    ExitCode::SUCCESS
}
