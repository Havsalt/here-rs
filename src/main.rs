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

/// Effortlessly grab and copy file locations
///
/// The path that was copied to clipboard will be printed with color.
/// Coloring can be turned off with `-c/--no-color`,
/// and copying to clipboard is ignored with `-n/--no-copy`
///
/// Usefull combinations of flags:
///
/// * here => Copy current working directory to clipboard
///           and print colored result
///
/// * here -wf => Copy folder location of binary/script, that is found in `PATH`
///
/// * here -wfdnc => Change current working directory to where the binary/script is located
///
/// * here -qe => Copy as string path, similar to a literal string
///
#[derive(Parser, Debug)]
#[command(name = "here", version, styles = STYLES)]
struct Args {
    /// Additional path segment or program name used for searching
    ///
    /// Normal mode: This argument will be treated as path segment,
    /// that will be appended to the path of current working directory
    ///
    /// Search mode: Searches for the binary/program,
    /// and uses that path instead of current working directory.
    /// Requires: `-w/--from-where`
    #[arg(value_name = "PATH SEGMENT / PROGRAM SEARCH")]
    path_segment_or_program_search: Option<String>,

    /// Get folder component of result
    ///
    /// If the target path is already a folder,
    /// this flag will be ignored
    #[arg(short = 'f', long = "folder")]
    folder_component: bool,

    /// Use `where` command to search
    ///
    /// On Windows, the `where` command will be called in a subprocess.
    /// The result is the path to that binary/script
    ///
    /// Todo: On Linux, use coresponding command to `where`
    ///
    /// If multiple results are found,
    /// a prompt will be used to select which path to use.
    /// This can be skipped by supplying `--select-first`,
    /// to select the first option
    ///
    /// *Error* if search fails to find the binary/script
    #[arg(
        short,
        long = "from-where",
        requires = "path_segment_or_program_search"
    )]
    where_search: bool,

    /// Set current working directory to result
    ///
    /// This is done by scheduling keyboard events
    /// that will write to the terminal after program execution
    #[arg(short = 'd', long)]
    change_directory: bool,

    /// Escape backslashes
    ///
    /// "\" -> "\\"
    ///
    /// Turn every backslash into a pair of blackslashes
    #[arg(short = 'e', long)]
    escape_backslash: bool,

    /// Wrap result in double quotes
    #[arg(short = 'q', long)]
    wrap_quote: bool,

    /// Resolve symlink path
    ///
    /// The path the symlink was pointing to will instead be used
    ///
    /// *Warning* if target path is not a symlink
    #[arg(short = 'r', long)]
    resolve_symlink: bool,

    /// Prevent copy to clipboard
    ///
    /// The result path is still printed
    #[arg(short = 'n', long)]
    no_copy: bool,

    /// Suppress color
    ///
    /// Prevents the use of ANSI escape codes,
    /// that would normally be used to show colors
    #[arg(short = 'c', long)]
    no_color: bool,

    /// Force posix style path
    ///
    /// Replaces all backslashes with forwardslashes
    #[arg(long = "posix", conflicts_with = "no_posix_style")]
    posix_style: bool,

    /// Prevent posix style path
    ///
    /// Replaces all forwardslashes with backslashes
    #[arg(long = "no-posix", conflicts_with = "posix_style")]
    no_posix_style: bool,

    /// Select first option if multiresult
    ///
    /// Requires: `-w/--from-where`
    #[arg(long = "select-first", requires = "where_search")]
    select_first_option: bool,

    /// Generate completions for given shell
    ///
    /// The generated script will be printed,
    /// and can be piped to a completion file for the given shell
    ///
    /// Cannot be paired with either positional arguments or flags
    #[arg(
        long = "completions",
        value_name = "SHELL",
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
            "path_segment_or_program_search"
        ]
    )]
    generate_completions: Option<Shell>,

    /// Generate markdown help page
    ///
    /// A help page with information about the program
    /// will be generated in the Markdown format,
    /// and can be piped to a file for later use
    ///
    /// Cannot be paired with either positional arguments or flags
    #[arg(
        long = "markdown",
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
            "path_segment_or_program_search"
        ]
    )]
    generate_markdown: bool,
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
