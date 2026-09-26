//! Recall: a local, Markdown-first active-recall CLI.
//!
//! This file is only the entry point and the command table. The rest lives in
//! focused modules, so a change usually means opening one of them:
//!
//! | Module              | Owns                                                      |
//! |---------------------|-----------------------------------------------------------|
//! | `model.rs`          | the shared data types                                     |
//! | `markdown.rs`       | reading and patching quizzes and attempts                 |
//! | `grading.rs`        | grading one response, and the hint shown when it is wrong  |
//! | `scoring.rs`        | League Points, the rank ladder, accuracy                  |
//! | `store.rs`          | progress rows, file discovery, small helpers              |
//! | `report.rs`         | everything printed to the terminal                        |
//! | `commands/subject`  | `subject add`, `add`                                      |
//! | `commands/start`    | `start`                                                   |
//! | `commands/submit`   | `submit`                                                  |
//! | `commands/attempt`  | `reveal`, `mark`, `giveup`                                |
//! | `cs2100/baseconv`   | the `baseconv` subcommand                                 |

use std::env;
use std::process::ExitCode;

mod commands;
mod config;
mod cs2100;
mod grading;
mod markdown;
mod model;
mod report;
mod scoring;
mod store;

use crate::model::*;
use crate::report::*;
use crate::store::*;

use crate::commands::attempt::*;
use crate::commands::start::*;
use crate::commands::subject::*;
use crate::commands::submit::*;

pub(crate) fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("error: {message}");
            ExitCode::FAILURE
        }
    }
}

pub(crate) fn run() -> AppResult<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    if let Ok(root) = find_root() {
        config::load(&root)?;
    }
    match args.as_slice() {
        [] => dashboard(&find_root()?),
        [command] if command == "help" || command == "--help" || command == "-h" => {
            print_help();
            Ok(())
        }
        [command, action, subject] if command == "subject" && action == "add" => {
            subject_add(subject)
        }
        [command, subject, file] if command == "add" => {
            move_resource(&find_root()?, subject, file, false)
        }
        [command, subject, file, flag] if command == "add" && flag == "--copy" => {
            move_resource(&find_root()?, subject, file, true)
        }
        [command, rest @ ..] if command == "start" => {
            let (query, take) = parse_start_args(rest)?;
            start_attempt(&find_root()?, query.as_deref(), take)
        }
        [command, rest @ ..] if command == "submit" => submit_attempt(&find_root()?, rest),
        [command, rest @ ..] if command == "reveal" => reveal_question(&find_root()?, rest),
        [command, rest @ ..] if command == "mark" => mark_question(&find_root()?, rest),
        [command, rest @ ..] if command == "giveup" => giveup_question(&find_root()?, rest),
        [command, rest @ ..] if command == "baseconv" => cs2100::baseconv::run(rest),
        [subject, command] if command == "rank" => rank_chart(&find_root()?, subject),
        _ => {
            print_help();
            Err("unknown or incomplete command".into())
        }
    }
}

pub(crate) fn print_help() {
    let scoring = config::get();
    println!(
        "Recall\n\n\
         usage:\n\
           recall subject add <subject>              create a subject\n\
           recall add <subject> <file>               move a resource into the subject\n\
           recall add <subject> <file> --copy        copy instead of move\n\
           recall start [quiz] [--take N]            new attempt (optionally N questions)\n\
           recall submit <attempt> [qid ...]         grade one question, or all of them\n\
           recall reveal <attempt> <qid>             show the model answer (self-check)\n\
           recall mark <attempt> <qid> correct|incorrect|retry\n\
           recall giveup <attempt> <qid>             show the answer and take the miss\n\
           recall baseconv <value> [-b N] [-i base]  CS2100 number conversions\n\
           recall <subject> rank                     graph rank history for one subject\n\
           recall                                    dashboard\n\n\
         A wrong answer is not final: you get a hint and another go, but the\n\
         wrong attempt is charged -{} LP and counts as a loss in accuracy.\n\
         Correct answers earn LP (easy {}, medium {}, hard {}), divided and\n\
         floored for each retry. Completing a paper adds an accuracy bonus.\n\
         LP per subject sets your rank:\n\
           Iron - Bronze - Silver - Gold - Platinum - Emerald - Diamond\n\
           Master (A-) - Grandmaster (A) - Challenger (A+)\n",
        scoring.wrong_answer, scoring.rewards[0], scoring.rewards[1], scoring.rewards[2],
    );
}

/// Parse the arguments to `start`: an optional quiz name plus `--take N`.
pub(crate) fn parse_start_args(args: &[String]) -> AppResult<(Option<String>, Option<usize>)> {
    let mut query = None;
    let mut take = None;
    let mut index = 0;
    while index < args.len() {
        let arg = &args[index];
        if arg == "--take" || arg == "-n" {
            let value = args
                .get(index + 1)
                .ok_or_else(|| "--take needs a question count".to_string())?;
            take = Some(
                value
                    .parse::<usize>()
                    .map_err(|_| format!("`{value}` is not a question count"))?,
            );
            index += 2;
        } else if let Some(value) = arg.strip_prefix("--take=") {
            take = Some(
                value
                    .parse::<usize>()
                    .map_err(|_| format!("`{value}` is not a question count"))?,
            );
            index += 1;
        } else if query.is_none() {
            query = Some(arg.clone());
            index += 1;
        } else {
            return Err(format!("unexpected argument `{arg}`"));
        }
    }
    if take == Some(0) {
        return Err("--take must be at least 1".into());
    }
    Ok((query, take))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_start_arguments() {
        let args = |values: &[&str]| {
            values
                .iter()
                .map(|value| value.to_string())
                .collect::<Vec<_>>()
        };
        assert_eq!(parse_start_args(&args(&[])).unwrap(), (None, None));
        assert_eq!(
            parse_start_args(&args(&["demo", "--take", "5"])).unwrap(),
            (Some("demo".to_string()), Some(5))
        );
        assert_eq!(
            parse_start_args(&args(&["--take=3"])).unwrap(),
            (None, Some(3))
        );
        assert!(parse_start_args(&args(&["--take", "0"])).is_err());
        assert!(parse_start_args(&args(&["--take", "x"])).is_err());
        assert!(parse_start_args(&args(&["one", "two"])).is_err());
    }
}
