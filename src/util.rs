use core::str;
use std::process::{Command, ExitCode};

use inquire::Select;

pub fn string_path_from_search(
    program: &str,
    select_first_option: &bool,
) -> Result<String, ExitCode> {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .arg("/C")
            .arg(format!("where {}", program))
            .output()
            .expect("'where' command found path to program/script on Windows")
    } else {
        todo!("implement for Linux")
    };
    let text = str::from_utf8(&output.stdout)
        .expect("path string is valid UTF-8")
        .trim()
        .replace("\r", "")
        .leak();
    if text.contains("\n") {
        let options: Vec<&str> = text.split("\n").collect();
        if select_first_option.to_owned() {
            return Ok(options[0].to_owned());
        }
        let select = Select::new("Select a path:", options);
        return match select.prompt_skippable() {
            Ok(answer) => match answer {
                Some(str_answer) => Ok(str_answer.to_owned()),
                None => return Err(ExitCode::FAILURE),
            },
            Err(_) => return Err(ExitCode::FAILURE),
        };
    }
    Ok(text.to_owned())
}
