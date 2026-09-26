//! Single-question operations: `reveal`, `mark` and `giveup`.
//!
//! Also holds `AttemptView`, the loader that returns an attempt together with
//! its source quiz.

use std::fs;
use std::path::{Path, PathBuf};

use crate::commands::submit::submit_attempt;
use crate::grading::*;
use crate::markdown::*;
use crate::model::*;
use crate::store::*;

/// Everything needed to act on a single question of an attempt.
pub(crate) struct AttemptView {
    pub(crate) path: PathBuf,
    pub(crate) subject: String,
    pub(crate) attempt_id: String,
    pub(crate) quiz: Vec<Question>,
    pub(crate) attempted: Vec<AttemptQuestion>,
}

impl AttemptView {
    /// `subject/attempt-id` - unambiguous across subjects in printed commands.
    pub(crate) fn reference(&self) -> String {
        format!("{}/{}", self.subject, self.attempt_id)
    }
}

pub(crate) fn load_attempt(root: &Path, query: &str) -> AppResult<AttemptView> {
    let attempts = collect_markdown(root, "attempts")?;
    let path = resolve_file(root, &attempts, Some(query), "attempt")?;
    let text = fs::read_to_string(&path).map_err(io_error)?;
    let subject = marker_value(&text, "subject")
        .ok_or_else(|| "attempt is missing its subject marker".to_string())?;
    let attempt_id = marker_value(&text, "attempt-id")
        .ok_or_else(|| "attempt is missing its attempt-id marker".to_string())?;
    let quiz_relative = marker_value(&text, "source-quiz")
        .ok_or_else(|| "attempt is missing its source-quiz marker".to_string())?;
    let quiz_text = fs::read_to_string(root.join(&quiz_relative))
        .map_err(|_| format!("source quiz not found: {quiz_relative}"))?;
    Ok(AttemptView {
        path,
        subject,
        attempt_id,
        quiz: parse_quiz(&quiz_text)?,
        attempted: parse_attempt(&text)?,
    })
}

pub(crate) fn index_question<'a>(
    view: &'a AttemptView,
    id: &str,
) -> AppResult<(&'a Question, &'a AttemptQuestion)> {
    let question = view
        .quiz
        .iter()
        .find(|question| question.id == id)
        .ok_or_else(|| format!("no question `{id}` in the source quiz"))?;
    let attempted = view
        .attempted
        .iter()
        .find(|candidate| candidate.id == id)
        .ok_or_else(|| format!("attempt `{}` has no question `{id}`", view.attempt_id))?;
    Ok((question, attempted))
}

/// Show the model answer so a free-response question can be self-checked.
pub(crate) fn reveal_question(root: &Path, args: &[String]) -> AppResult<()> {
    if args.len() != 2 {
        return Err("usage: recall reveal <attempt> <question-id>".into());
    }
    let view = load_attempt(root, &args[0])?;
    let (question, attempted) = index_question(&view, &args[1])?;
    let id = &question.id;

    // Don't leak the answer to a question that has not been attempted yet.
    if attempted.response.trim().is_empty() && !is_resolved(attempted) {
        return Err(format!(
            "`{id}` has no answer yet - write your response in the attempt file first"
        ));
    }

    let rule = question.answer.lines().next().unwrap_or("manual").trim();
    println!();
    println!("  -- {id} [{}] --", attempted.difficulty.as_str());
    println!();
    for line in question.prompt.lines() {
        println!("  {line}");
    }
    println!();
    println!("  your answer:");
    for line in attempted.response.lines() {
        println!("    {line}");
    }
    println!();
    println!("  model answer:");
    if rule == "manual" {
        for line in question.explanation.lines() {
            println!("    {line}");
        }
    } else {
        for line in question
            .answer
            .lines()
            .skip(1)
            .filter(|line| !line.trim().is_empty())
        {
            println!("    {line}");
        }
        println!();
        println!("  why:");
        for line in question.explanation.lines() {
            println!("    {line}");
        }
    }
    println!();

    match (rule, is_resolved(attempted)) {
        ("manual", false) => {
            println!("  self-check, then:");
            println!("    recall mark {} {id} correct", view.reference());
            println!("    recall mark {} {id} incorrect", view.reference());
        }
        ("manual", true) => println!(
            "  already marked `{}`. To change it, run `recall mark`.",
            attempted.result
        ),
        _ => println!(
            "  graded automatically:  recall submit {} {id}",
            view.reference()
        ),
    }
    Ok(())
}

/// Record a self-assessment for a free-response question.
pub(crate) fn mark_question(root: &Path, args: &[String]) -> AppResult<()> {
    if args.len() != 3 {
        return Err("usage: recall mark <attempt> <question-id> correct|incorrect|retry".into());
    }
    let verdict = match args[2].trim().to_ascii_lowercase().as_str() {
        "correct" | "c" | "yes" | "y" | "right" | "1" => "correct",
        "incorrect" | "wrong" | "no" | "n" | "0" => "incorrect",
        // Not a miss: park it and let them have another go.
        "retry" | "again" | "r" => "retry",
        other => {
            return Err(format!(
                "`{other}` is not a verdict; use correct, incorrect or retry"
            ));
        }
    };

    let view = load_attempt(root, &args[0])?;
    let (question, attempted) = index_question(&view, &args[1])?;
    let rule = question.answer.lines().next().unwrap_or("manual").trim();
    if rule != "manual" {
        return Err(format!(
            "`{}` is graded automatically; run `recall submit {} {}`",
            question.id,
            view.reference(),
            question.id
        ));
    }
    if attempted.response.trim().is_empty() {
        return Err(format!(
            "`{}` has no response yet - answer it in the attempt file first",
            question.id
        ));
    }
    if is_resolved(attempted) {
        return Err(format!(
            "`{}` is already settled as `{}` - start a fresh attempt to try it again",
            question.id, attempted.result
        ));
    }

    let tries = attempted.tries + 1;
    let mut text = fs::read_to_string(&view.path).map_err(io_error)?;
    upsert_question_marker(&mut text, &question.id, "tries", &tries.to_string())?;
    upsert_question_marker(&mut text, &question.id, "result", verdict)?;

    // Anything that is not correct costs the flat penalty, once per attempt.
    if verdict != "correct" {
        let progress = root
            .join("subjects")
            .join(&view.subject)
            .join("progress.md");
        append_progress_rows(
            &progress,
            &[ProgressRow {
                attempt: view.attempt_id.clone(),
                question: question.id.clone(),
                difficulty: attempted.difficulty,
                result: "wrong".to_string(),
                tries,
            }],
        )?;
    }

    if verdict == "retry" {
        append_question_block(
            &mut text,
            &question.id,
            "feedback",
            "Marked for another go - revise your response and mark it again.",
        )?;
        fs::write(&view.path, text).map_err(io_error)?;
        println!("  [~] {} parked for another attempt.", question.id);
        return Ok(());
    }
    fs::write(&view.path, text).map_err(io_error)?;

    // Hand off to the normal submit path so scoring lives in one place.
    submit_attempt(root, &[args[0].clone(), args[1].clone()])
}

/// Show the answer and take the miss, ending a question's retries.
pub(crate) fn giveup_question(root: &Path, args: &[String]) -> AppResult<()> {
    if args.len() != 2 {
        return Err("usage: recall giveup <attempt> <question-id>".into());
    }
    let view = load_attempt(root, &args[0])?;
    let (question, attempted) = index_question(&view, &args[1])?;
    if is_resolved(attempted) {
        return Err(format!(
            "`{}` is already settled as `{}`",
            question.id, attempted.result
        ));
    }
    if attempted.response.trim().is_empty() {
        return Err(format!(
            "`{}` has no response yet - nothing to give up on",
            question.id
        ));
    }

    let rule = question.answer.lines().next().unwrap_or("manual").trim();
    let hash = response_hash(&attempted.response);
    let ungraded = attempted.graded != hash;

    // Never punish an answer that turns out to be right.
    if ungraded && rule != "manual" && grade_automatic(&question.answer, &attempted.response)? {
        println!("  that answer is actually correct - recording it instead of a miss.");
        return submit_attempt(root, &[args[0].clone(), args[1].clone()]);
    }

    // Make sure this wrong attempt is charged exactly once.
    if ungraded {
        if rule == "manual" {
            let progress = root
                .join("subjects")
                .join(&view.subject)
                .join("progress.md");
            append_progress_rows(
                &progress,
                &[ProgressRow {
                    attempt: view.attempt_id.clone(),
                    question: question.id.clone(),
                    difficulty: attempted.difficulty,
                    result: "wrong".to_string(),
                    tries: attempted.tries + 1,
                }],
            )?;
        } else {
            // Grading through submit records the wrong attempt and its penalty.
            submit_attempt(root, &[args[0].clone(), args[1].clone()])?;
        }
    }

    let mut text = fs::read_to_string(&view.path).map_err(io_error)?;
    upsert_question_marker(&mut text, &question.id, "result", "incorrect")?;
    append_question_block(
        &mut text,
        &question.id,
        "feedback",
        question.explanation.trim(),
    )?;
    fs::write(&view.path, text).map_err(io_error)?;

    println!("  answer revealed - reading it is worth more than the LP.");
    // Hand off to the normal submit path so the miss is recorded in one place.
    submit_attempt(root, &[args[0].clone(), args[1].clone()])
}
