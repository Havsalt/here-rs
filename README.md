# here-rs

Effortlessly grab and copy file locations

## Table of Contents

- [Rational](#rational)
- [Help page](#help-page-for-here)
  - [Usage](#usage)
  - [Arguments](#arguments)
  - [Options](#options)
- [License](#license)

## Rational

This tool was originally designed to simply copy path of `current working directory` into the `clipboard`.
As time went, more flags and features were added, like locating binaries and copying their path, or being able to resolve symlinks.
This could have been done by simply aliasing `here` -> `pwd | clip` (on PowerShell),
but I chose to use this as an excuse to try `Rust` for the first proper time.
It was also a much-needed speedup from the [`Python` solution](https://github.com/havsalt/here) I originally came up with.
Now it's feature rich and blazingly fast.

## Help Page for `here`

Effortlessly grab and copy file locations

The path that was copied to clipboard will be printed with color. Coloring can be turned off with `-c/--no-color`, and copying to clipboard is ignored with `-n/--no-copy`

Usefull combinations of flags:

- *none* => Copy current working directory to clipboard and print colored result

- `-wf` => Copy folder location of binary/script, that is found in `PATH`

- `-wfdnc` => Change current working directory to where the binary/script is located

- `-qe` => Copy as string path, similar to a literal string

### **Usage:**

`here [OPTIONS] [PATH SEGMENT / PROGRAM SEARCH]`

### **Arguments:**

- `<PATH SEGMENT / PROGRAM SEARCH>` — Additional path segment or program name used for searching

    Default mode: If not present, uses path of `current working directory`

    Segment mode: Treats argument as a path segment, that will be appended to the path of current working directory

    Search mode: Searches for the binary/program, and uses that path instead of current working directory. Requires: `-w/--from-where`

### **Options:**

- `-f`, `--folder` — Get folder component of result

    If the target path is already a folder, this flag will be ignored

- `-w`, `--from-where` — Use `where` command to search

    On Windows, the `where` command will be called in a subprocess. The result is the path to that binary/script

    Todo: On Linux, use coresponding command to `where`

    If multiple results are found, a prompt will be used to select which path to use. This can be skipped by supplying `--select-first`, to select the first option

    **Error** if search fails to find the binary/script

- `-d`, `--change-directory` — Set current working directory to result

    This is done by scheduling keyboard events that will write to the terminal after program execution

- `-e`, `--escape-backslash` — Escape backslashes

    "\\" -> "\\\\"

    Turn every backslash into a pair of blackslashes

- `-q`, `--wrap-quote` — Wrap result in double quotes

- `-r`, `--resolve-symlink` — Resolve symlink path

    The path the symlink was pointing to will instead be used

    **Warning** if target path is not a symlink

- `-n`, `--no-copy` — Prevent copy to clipboard

    The result path is still printed

- `-c`, `--no-color` — Suppress color

    Prevents the use of ANSI escape codes, that would normally be used to show colors

- `--posix` — Force posix style path

    Replaces all backslashes with forwardslashes

- `--no-posix` — Prevent posix style path

    Replaces all forwardslashes with backslashes

- `--select-first` — Select first option if multiresult

    Requires: `-w/--from-where`

- `--completions <SHELL>` — Generate completions for given shell

    The generated script will be printed, and can be piped to a completion file for the given shell

    Cannot be paired with either positional arguments or flags

  Possible values: `bash`, `elvish`, `fish`, `powershell`, `zsh`

- `--markdown` — Generate markdown help page

    A help page with information about the program will be generated in the Markdown format, and can be piped to a file for later use

    Cannot be paired with either positional arguments or flags

## License

MIT
