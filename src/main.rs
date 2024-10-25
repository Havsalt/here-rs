use clap::Parser;
use cli_clipboard;
use colored::Colorize;
use path_clean::PathClean;
use core::str;
use std::env::current_dir;
use std::path::PathBuf;
use std::process::Command;
use std::str::FromStr;

#[derive(Parser, Debug)]
#[command(
    version,
    about = "Copies the current working directory to clipboard",
    long_about = None
)]
struct Args {
    #[arg(default_value = ".")]
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
}

fn main() {
    let args = Args::parse();

    let path = if args.where_search {
        // TODO: Fix process not starting correctly
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .arg("/C")
                .arg(format!("where {}", args.segment_or_name))
                .output()
                .expect("'where' command found path to program/script on Windows")
        } else {
            panic!()
        };
        // dbg!(&output.stdout);
        // println!("{}", str::from_utf8(&output.stdout).unwrap());
        let str_path = str::from_utf8(&output.stdout).unwrap();
        PathBuf::from_str(str_path).unwrap()
    } else {
        let path = current_dir().unwrap();
        let location = path.join(&args.segment_or_name);
        if args.folder_component & location.is_file() {
            location.parent().unwrap().to_path_buf()
        } else {
            location
        }
    };

    let absolute_path = path.clean();
    // Apply styling options
    let visual = absolute_path
        .display()
        .to_string()
        .to_string();

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
        cli_clipboard::set_contents(visual.to_owned()).unwrap();
    }
    if args.no_color {
        println!("{}", visual);
    } else {
        println!("{}", visual.truecolor(250, 128, 114));
    }
}
