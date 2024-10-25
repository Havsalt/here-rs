use clap::Parser;
use cli_clipboard;
use colored::Colorize;
use core::str;
use std::env::current_dir;
use std::path::PathBuf;
use std::process::Command;
use std::str::FromStr;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(default_value = ".")]
    segment_or_name: String,
    #[arg(short, long = "folder")]
    folder_mode: bool,
    #[arg(short, long = "where")]
    where_mode: bool,
}

fn main() {
    let args = Args::parse();

    let path = if args.where_mode {
        // TODO: Fix process not starting correctly
        let output = Command::new("cmd")
            .arg("where")
            // .arg(&args.segment_or_name)
            .output()
            .expect("'where' command found path to program/script on Windows");
        let str_path = str::from_utf8(&output.stdout).unwrap();
        PathBuf::from_str(str_path).unwrap()
    } else {
        let path = current_dir().unwrap();
        let location = path.join(&args.segment_or_name);
        if args.folder_mode & location.is_file() {
            location.parent().unwrap().to_path_buf()
        } else {
            location
        }
    };

    let absolute_path = path.canonicalize().unwrap();
    let visual = absolute_path
        .display()
        .to_string()
        .strip_prefix("\\\\?\\") // On Windows, absolute paths may be prefixed with `\\?\`
        .unwrap()
        .to_string();

    cli_clipboard::set_contents(visual.to_owned()).unwrap();
    println!("{}", visual.truecolor(250, 128, 114));
}
