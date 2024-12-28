use clap::Parser;
use clap_complete::Shell;
use havsalt_clap_styles::STYLES;

/// Effortlessly grab and copy file locations
///
/// The path that was copied to clipboard will be printed with color.
/// Coloring can be turned off with `-c/--no-color`,
/// and copying to clipboard is ignored with `-n/--no-copy`
///
/// Usefull combinations of flags:
///
/// * _none_ => Copy current working directory to clipboard
///           and print colored result
///
/// * `-wf` => Copy folder location of binary/script, that is found in `PATH`
///
/// * `-wfdnc` => Change current working directory to where the binary/script is located
///
/// * `-qe` => Copy as string path, similar to a literal string
///
#[derive(Parser, Debug)]
#[command(name = "here", version, styles = STYLES)]
pub struct Cli {
    /// Additional path segment or program name used for searching
    ///
    /// Default mode: If not present, uses path of `current working directory`
    ///
    /// Segment mode: Treats argument as a path segment,
    /// that will be appended to the path of current working directory
    ///
    /// Search mode: Searches for the binary/program,
    /// and uses that path instead of current working directory.
    /// Requires: `-w/--from-where`
    #[arg(value_name = "PATH SEGMENT / PROGRAM SEARCH")]
    pub path_segment_or_program_search: Option<String>,

    /// Get folder component of result
    ///
    /// If the target path is already a folder,
    /// this flag will be ignored
    #[arg(short = 'f', long = "folder")]
    pub folder_component: bool,

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
    /// **Error** if search fails to find the binary/script
    #[arg(
        short,
        long = "from-where",
        requires = "path_segment_or_program_search"
    )]
    pub where_search: bool,

    /// Set current working directory to result
    ///
    /// This is done by scheduling keyboard events
    /// that will write to the terminal after program execution
    #[arg(short = 'd', long)]
    pub change_directory: bool,

    /// Escape backslashes
    ///
    /// "\\" -> "\\\\"
    ///
    /// Turn every backslash into a pair of blackslashes
    #[arg(short = 'e', long)]
    pub escape_backslash: bool,

    /// Wrap result in double quotes
    #[arg(short = 'q', long)]
    pub wrap_quote: bool,

    /// Resolve symlink path
    ///
    /// The path the symlink was pointing to will instead be used
    ///
    /// **Warning** if target path is not a symlink
    #[arg(short = 'r', long)]
    pub resolve_symlink: bool,

    /// Prevent copy to clipboard
    ///
    /// The result path is still printed
    #[arg(short = 'n', long)]
    pub no_copy: bool,

    /// Suppress color
    ///
    /// Prevents the use of ANSI escape codes,
    /// that would normally be used to show colors
    #[arg(short = 'c', long)]
    pub no_color: bool,

    /// Force posix style path
    ///
    /// Replaces all backslashes with forwardslashes
    #[arg(long = "posix", conflicts_with = "no_posix_style")]
    pub posix_style: bool,

    /// Prevent posix style path
    ///
    /// Replaces all forwardslashes with backslashes
    #[arg(long = "no-posix", conflicts_with = "posix_style")]
    pub no_posix_style: bool,

    /// Select first option if multiresult
    ///
    /// Requires: `-w/--from-where`
    #[arg(long = "select-first", requires = "where_search")]
    pub select_first_option: bool,

    /// Generate completion script for given shell
    ///
    /// The generated script will be printed,
    /// and can be piped to a completion file for the given shell
    ///
    /// Cannot be paired with positional arguments or flags
    #[arg(
        long = "completion",
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
    pub generate_completions: Option<Shell>,

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
    pub generate_markdown: bool,
}
