use std::io;
use std::path::PathBuf;
use std::process::ExitCode;
use std::{env::current_dir, fs};

use clap::CommandFactory;
use clap::Parser;
use clap_complete::generate;
use clap_markdown::print_help_markdown;
use cli_clipboard;
use colored::Colorize;
use enigo::{Enigo, Key, Keyboard, Settings};
use path_clean::PathClean;

mod cli;
use cli::Cli;

mod colorize_ext;
use colorize_ext::ColorizeExt;

mod fetch;

fn main() -> ExitCode {
    let args = Cli::parse();

    // Generate completions or markdown help page
    if let Some(shell) = args.generate_completions {
        generate(
            shell,
            &mut Cli::command(),
            Cli::command().get_name().to_string(),
            &mut io::stdout(),
        );
    }
    if args.generate_markdown {
        print_help_markdown::<Cli>();
    }
    // Abort after printing completions or markdown help page
    if args.generate_completions.is_some() || args.generate_markdown {
        return ExitCode::SUCCESS;
    }

    // Select where to extract the path from
    let mut path = if args.where_search {
        if let Some(search_name) = args.path_segment_or_program_search {
            match fetch::string_path_from_search(&search_name, &args.select_first_option) {
                Ok(string_path) => {
                    if string_path.is_empty() {
                        if args.no_color {
                            println!("[Error] Could not find \"{search_name}\"");
                        } else {
                            let label = "[Error]".crimson();
                            let msg = "Could not find".gray();
                            let program = format!("\"{}\"", search_name).white();
                            println!("{label} {msg} {program}");
                        }
                        return ExitCode::FAILURE;
                    } else {
                        PathBuf::from(string_path)
                    }
                }
                Err(exit_code) => return exit_code,
            }
        } else {
            if args.no_color {
                println!("[Error] Argument [PROGRAM SEARCH] is required")
            } else {
                let label = "[Error]".crimson();
                let msg1 = "Argument".gray();
                let arg = "[PROGRAM SEARCH]".white();
                let msg2 = "is".gray();
                let msg3 = "required".white();
                println!("{label} {msg1} {arg} {msg2} {msg3}");
            }
            return ExitCode::FAILURE;
        }
    } else {
        // If not using `-w/--from-where`, use current working directory
        let segment = args
            .path_segment_or_program_search
            .unwrap_or(".".to_owned());
        current_dir()
            .expect("cwd was found and have permission")
            .join(segment)
    };

    // Apply path manipulations
    path = path.clean();

    if args.resolve_symlink {
        if path.exists() {
            if path.is_symlink() {
                path = fs::read_link(path).expect("path is symlink that exists");
            } else {
                if args.no_color {
                    println!("[Warning] Path {} is not a symlink", path.display());
                } else {
                    let label = "[Warning]".orange();
                    let msg1 = "Path".gray();
                    let value = path.display().to_string().bright_white();
                    let msg2 = "is".gray();
                    let msg3 = "not a symlink".cyan();
                    println!("{label} {msg1} {value} {msg2} {msg3}");
                }
            }
        } else {
            if args.no_color {
                println!("[Warning] Symlink {} does not exist", path.display());
            } else {
                let label = "[Warning]".orange();
                let msg1 = "Symlink".gray();
                let value = path.display().to_string().bright_white();
                let msg2 = "does".gray();
                let msg3 = "not exist".cyan();
                println!("{label} {msg1} {value} {msg2} {msg3}");
            }
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
