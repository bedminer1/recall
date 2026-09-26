//! `recall start [quiz] [--take N]`: build an attempt from a quiz.

use std::fs;
use std::path::Path;

use crate::markdown::*;
use crate::model::*;
use crate::store::*;

pub(crate) fn start_attempt(
    root: &Path,
    query: Option<&str>,
    take: Option<usize>,
) -> AppResult<()> {
    let quizzes = collect_markdown(root, "quizzes")?;
    let quiz = resolve_file(root, &quizzes, query, "quiz")?;
    let subject_dir = quiz
        .parent()
        .and_then(Path::parent)
        .ok_or_else(|| "quiz is not inside a subject".to_string())?;
    let subject = subject_dir
        .file_name()
        .and_then(|v| v.to_str())
        .ok_or_else(|| "invalid subject path".to_string())?;
    let quiz_text = fs::read_to_string(&quiz).map_err(io_error)?;
    let mut questions: Vec<(usize, Question)> =
        parse_quiz(&quiz_text)?.into_iter().enumerate().collect();
    if questions.is_empty() {
        return Err("quiz contains no questions".into());
    }
    let available = questions.len();
    let mut drawn = available;
    if let Some(count) = take
        && count < available
    {
        // Pick a random subset, then restore quiz order so the attempt reads
        // naturally. Question ids keep their original numbers.
        shuffle(&mut questions, random_seed());
        questions.truncate(count);
        questions.sort_by_key(|(index, _)| *index);
        drawn = count;
    }
    let attempts_dir = subject_dir.join("attempts");
    fs::create_dir_all(&attempts_dir).map_err(io_error)?;
    let attempt_id = next_attempt_id(&attempts_dir)?;
    let attempt_path = attempts_dir.join(format!("{attempt_id}.md"));
    let quiz_relative = display_path(&quiz, root);
    let quiz_title = quiz_text
        .lines()
        .find(|line| line.starts_with("# "))
        .unwrap_or("# Quiz");
    let mut output = format!(
        "{quiz_title}\n\n<!-- attempt-id: {attempt_id} -->\n<!-- subject: {subject} -->\n<!-- source-quiz: {quiz_relative} -->\n<!-- submitted: no -->\n\n\
         Put your final response in the response block; the notes block is optional scratch work and is never graded.\n\
         `Working notes` up here is for shortcuts and traps worth remembering.\n\n\
         # Working notes\n\n<!-- notes -->\n\n"
    );
    for (_, question) in &questions {
        output.push_str(&format!(
            "{}\n\n{}\n\n<!-- response -->\n\n<!-- notes -->\n<!-- result: pending --> <!-- tries: 0 -->\n<!-- feedback -->\n\n\n\n",
            question.heading,
            question.prompt.trim()
        ));
    }
    fs::write(&attempt_path, output).map_err(io_error)?;
    println!("Attempt `{attempt_id}` is ready - {drawn} question(s).");
    if drawn < available {
        println!(
            "Drew {drawn} at random from {available} in `{}`.",
            file_stem(&quiz)?
        );
    }
    let first = &questions[0].1.id;
    println!("Open: {}", display_path(&attempt_path, root));
    println!("Then answer one at a time:  recall submit {subject}/{attempt_id} {first}");
    Ok(())
}
