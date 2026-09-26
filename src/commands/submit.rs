//! `recall submit <attempt> [qid ...]`.
//!
//! Decides which questions to grade (an explicit list, or the outstanding run
//! when none are named), grades them, records the ledger rows, and hands the
//! result to `report`.

use std::fs;
use std::path::Path;

use crate::grading::*;
use crate::markdown::*;
use crate::model::*;
use crate::report::*;
use crate::scoring::*;
use crate::store::*;

/// Grade one or more questions and report the moment each one resolves.
///
/// Resolved questions are written to progress.md straight away, so points and
/// streaks arrive per question instead of only when the whole attempt is done.
pub(crate) fn submit_attempt(root: &Path, args: &[String]) -> AppResult<()> {
    let Some(query) = args.first() else {
        return Err("usage: recall submit <attempt> [question-id ...]".into());
    };
    let selected: Vec<String> = args[1..]
        .iter()
        .map(|value| value.trim().to_string())
        .collect();

    let attempts = collect_markdown(root, "attempts")?;
    let attempt_path = resolve_file(root, &attempts, Some(query.as_str()), "attempt")?;
    let mut attempt_text = fs::read_to_string(&attempt_path).map_err(io_error)?;

    let subject = marker_value(&attempt_text, "subject")
        .ok_or_else(|| "attempt is missing its subject marker".to_string())?;
    let attempt_id = marker_value(&attempt_text, "attempt-id")
        .ok_or_else(|| "attempt is missing its attempt-id marker".to_string())?;
    let quiz_relative = marker_value(&attempt_text, "source-quiz")
        .ok_or_else(|| "attempt is missing its source-quiz marker".to_string())?;
    let quiz_text = fs::read_to_string(root.join(&quiz_relative))
        .map_err(|_| format!("source quiz not found: {quiz_relative}"))?;
    let quiz = parse_quiz(&quiz_text)?;

    let mut attempted = parse_attempt(&attempt_text)?;
    if attempted.is_empty() {
        return Err("attempt contains no questions".into());
    }

    let progress_path = root.join("subjects").join(&subject).join("progress.md");
    let history = read_progress_rows(&progress_path)?;

    // With no question ids, grade unanswered automatic work and bank any
    // verdict an LLM has already written into the attempt. The ledger remains
    // exclusively owned by this command.
    let targets: Vec<String> = if selected.is_empty() {
        submittable_questions(&attempted, &history, &attempt_id)
    } else {
        for id in &selected {
            if !attempted.iter().any(|question| &question.id == id) {
                return Err(format!("attempt `{attempt_id}` has no question `{id}`"));
            }
        }
        selected.clone()
    };

    if targets.is_empty() {
        let settled = attempted.iter().filter(|q| is_resolved(q)).count();
        let total = attempted.len();
        let open: Vec<&AttemptQuestion> = attempted.iter().filter(|q| !is_resolved(q)).collect();

        // A paper can become fully settled before a bare submit reaches this
        // command (for example through the last single-question mark). Do not
        // let the early "nothing to grade" return swallow its completion bonus.
        if open.is_empty() {
            let all_recorded = attempted.iter().all(|question| {
                history.iter().any(|row| {
                    row.attempt == attempt_id
                        && row.question == question.id
                        && (row.result == "correct" || row.result == "incorrect")
                })
            });
            if all_recorded {
                let existing_bonus = history.iter().find(|row| {
                    row.attempt == attempt_id
                        && row.question == "__paper_bonus__"
                        && row.result == "bonus"
                });
                let accuracy = accuracy_of(history.iter().filter(|row| row.attempt == attempt_id));
                let bonus = existing_bonus
                    .map(|row| row.tries)
                    .unwrap_or_else(|| paper_completion_bonus(accuracy));
                if existing_bonus.is_none() {
                    append_progress_rows(
                        &progress_path,
                        &[ProgressRow {
                            attempt: attempt_id.clone(),
                            question: "__paper_bonus__".to_string(),
                            difficulty: Difficulty::Easy,
                            result: "bonus".to_string(),
                            tries: bonus,
                        }],
                    )?;
                    replace_global_marker(&mut attempt_text, "submitted", "yes")?;
                    fs::write(&attempt_path, &attempt_text).map_err(io_error)?;
                }
                let final_history = read_progress_rows(&progress_path)?;
                let lp = total_lp(&final_history);
                let subject_accuracy = accuracy_of(final_history.iter());
                let rank = rank_for_performance(lp, subject_accuracy);
                println!();
                println!("  +==================================+");
                println!("  |        P A P E R   D O N E       |");
                println!("  |          +{bonus:<4} LP BONUS          |");
                println!("  +==================================+");
                println!(
                    "  {}% accuracy ({}/{})  ·  {} LP total  ·  {}",
                    accuracy.percent(),
                    accuracy.correct,
                    accuracy.total,
                    lp,
                    rank.label()
                );
                if existing_bonus.is_some() {
                    println!("  Bonus already claimed; no LP was added again.");
                }
                return Ok(());
            }
        }

        println!();
        println!("  {settled}/{total} settled - nothing new to grade.");
        // Working written but no answer is the easy mistake now that the two
        // areas look alike, so name it rather than just saying "blank".
        if let Some(question) = open
            .iter()
            .find(|q| q.response.trim().is_empty() && !q.notes.trim().is_empty())
        {
            println!(
                "  `{}` has working under **Working** but nothing under **Answer**, and only",
                question.id
            );
            println!("  the Answer block is graded. Move the answer down, then run:");
            println!("    recall submit {subject}/{attempt_id}");
        } else if let Some(question) = open.iter().find(|q| q.response.trim().is_empty()) {
            println!(
                "  Next up: `{}` - put your answer under **Answer**, then run:",
                question.id
            );
            println!("    recall submit {subject}/{attempt_id}");
        } else if let Some(question) = open.first() {
            println!(
                "  `{}` is open with an unchanged answer. Edit it, or name another:",
                question.id
            );
            println!("    recall submit {subject}/{attempt_id} {}", question.id);
        } else {
            println!("  Every question in `{attempt_id}` is done.");
        }
        return Ok(());
    }

    // Only a final verdict blocks a question from being scored again; the
    // per-attempt penalty rows must not.
    let already_final = |id: &str| {
        history.iter().any(|row| {
            row.attempt == attempt_id
                && row.question == id
                && (row.result == "correct" || row.result == "incorrect")
        })
    };

    let mut waiting = Vec::new();
    let mut empty = Vec::new();
    let mut wrong_attempts = Vec::new();
    for id in &targets {
        let question = quiz
            .iter()
            .find(|question| &question.id == id)
            .ok_or_else(|| format!("question `{id}` is missing from the source quiz"))?;
        let rule = question.answer.lines().next().unwrap_or("manual").trim();
        let attempted_question = attempted
            .iter_mut()
            .find(|candidate| &candidate.id == id)
            .expect("targets come from the attempt");

        // Already settled questions are never re-graded, so a give-up or an
        // LLM verdict cannot be overwritten by a later submit.
        if is_resolved(attempted_question) {
            continue;
        }

        // Already scored in an earlier run. The attempt file can drift out of
        // step with progress.md (a hand edit, a revert), and re-grading would
        // charge the penalty a second time, so trust the ledger and repair the
        // marker if it disagrees.
        if let Some(banked) = history.iter().find(|row| {
            row.attempt == attempt_id
                && row.question == *id
                && (row.result == "correct" || row.result == "incorrect")
        }) {
            let result = banked.result.clone();
            attempted_question.result = result.clone();
            upsert_question_marker(&mut attempt_text, id, "result", &result)?;
            continue;
        }

        // A blank response is not a wrong answer; it is simply not an attempt yet.
        if attempted_question.response.trim().is_empty() {
            empty.push(id.clone());
            continue;
        }

        if rule == "manual" {
            waiting.push(id.clone());
            continue;
        }

        let hash = response_hash(&attempted_question.response);
        // Re-grading the identical answer (a full sweep, say) must not charge
        // the wrong-answer penalty a second time.
        if attempted_question.result == "retry"
            && attempted_question.graded != 0
            && attempted_question.graded == hash
        {
            continue;
        }

        attempted_question.tries += 1;
        let tries = attempted_question.tries;
        upsert_question_marker(&mut attempt_text, id, "tries", &tries.to_string())?;
        upsert_question_marker(&mut attempt_text, id, "graded", &hash.to_string())?;
        attempted_question.graded = hash;

        if grade_automatic(&question.answer, &attempted_question.response)? {
            attempted_question.result = "correct".to_string();
            replace_question_marker(&mut attempt_text, id, "result", "correct")?;
        } else {
            // Wrong is not final: the question goes back to `retry` with a nudge
            // instead of the answer, but this attempt is charged and counted.
            attempted_question.result = "retry".to_string();
            replace_question_marker(&mut attempt_text, id, "result", "retry")?;
            let hint = hint_text(question, &attempted_question.response, tries);
            append_question_block(&mut attempt_text, id, "feedback", &hint)?;
            wrong_attempts.push(ProgressRow {
                attempt: attempt_id.clone(),
                question: question.id.clone(),
                difficulty: attempted_question.difficulty,
                result: "wrong".to_string(),
                tries,
            });
        }
    }

    // Record everything that became known during this run, once each.
    let mut resolved = Vec::new();
    for id in &targets {
        if let Some(question) = attempted.iter().find(|question| &question.id == id)
            && is_resolved(question)
            && !already_final(id)
        {
            resolved.push(ProgressRow {
                attempt: attempt_id.clone(),
                question: question.id.clone(),
                difficulty: question.difficulty,
                result: question.result.clone(),
                tries: question.tries,
            });
        }
    }
    // Penalty rows go in first so the first row for a question records how the
    // first attempt actually went.
    append_progress_rows(&progress_path, &wrong_attempts)?;
    append_progress_rows(&progress_path, &resolved)?;
    let mut final_history = read_progress_rows(&progress_path)?;

    // An attempt is only finished when every question is both answered and
    // banked; a question marked in the file but never submitted is not scored.
    let all_answered = attempted.iter().all(is_resolved);
    let all_recorded = attempted.iter().all(|question| {
        final_history
            .iter()
            .any(|row| row.attempt == attempt_id && row.question == question.id)
    });
    let complete = all_answered && all_recorded;
    let already_bonused = final_history.iter().any(|row| {
        row.attempt == attempt_id && row.question == "__paper_bonus__" && row.result == "bonus"
    });
    let completion_bonus = if complete && !resolved.is_empty() && !already_bonused {
        let accuracy = accuracy_of(final_history.iter().filter(|row| row.attempt == attempt_id));
        let bonus = paper_completion_bonus(accuracy);
        append_progress_rows(
            &progress_path,
            &[ProgressRow {
                attempt: attempt_id.clone(),
                question: "__paper_bonus__".to_string(),
                difficulty: Difficulty::Easy,
                result: "bonus".to_string(),
                tries: bonus,
            }],
        )?;
        final_history = read_progress_rows(&progress_path)?;
        bonus
    } else {
        0
    };
    if complete {
        replace_global_marker(&mut attempt_text, "submitted", "yes")?;
    }
    fs::write(&attempt_path, &attempt_text).map_err(io_error)?;

    // Suggestions must be unambiguous: every subject numbers its attempts from
    // a001, so always show the subject-qualified name in commands we print.
    let attempt_ref = format!("{subject}/{attempt_id}");
    let quiz_stem = quiz_stem_of(&quiz_relative);
    let quiz_map = quizzes_by_attempt(root, &subject)?;
    let summary = Summary {
        subject: subject.clone(),
        subject_accuracy: accuracy_of(final_history.iter()),
        quiz_accuracy: accuracy_of(
            final_history
                .iter()
                .filter(|row| quiz_map.get(&row.attempt).map(String::as_str) == Some(&quiz_stem)),
        ),
        quiz: quiz_stem,
    };
    report(
        &attempt_id,
        &attempt_ref,
        &quiz,
        &targets,
        &attempted,
        &resolved,
        &wrong_attempts,
        &waiting,
        &empty,
        &history,
        &final_history,
        complete,
        completion_bonus,
        &summary,
    );
    Ok(())
}

/// Everything a bare submit can act on: fresh responses plus verdicts written
/// by an external marker that have not yet reached the ledger.
pub(crate) fn submittable_questions(
    attempted: &[AttemptQuestion],
    history: &[ProgressRow],
    attempt_id: &str,
) -> Vec<String> {
    attempted
        .iter()
        .filter(|question| {
            if !is_resolved(question) {
                return !question.response.trim().is_empty();
            }
            !history.iter().any(|row| {
                row.attempt == attempt_id
                    && row.question == question.id
                    && (row.result == "correct" || row.result == "incorrect")
            })
        })
        .map(|question| question.id.clone())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn attempted(id: &str, response: &str, result: &str) -> AttemptQuestion {
        AttemptQuestion {
            id: id.into(),
            difficulty: Difficulty::Medium,
            response: response.into(),
            result: result.into(),
            tries: 0,
            graded: 0,
            notes: String::new(),
        }
    }

    fn banked(id: &str) -> ProgressRow {
        ProgressRow {
            attempt: "a001".into(),
            question: id.into(),
            difficulty: Difficulty::Medium,
            result: "correct".into(),
            tries: 1,
        }
    }

    #[test]
    fn bare_submit_picks_up_llm_verdicts_once() {
        let questions = vec![
            attempted("q1", "answer", "correct"),
            attempted("q2", "answer", "pending"),
            attempted("q3", "", "pending"),
            attempted("q4", "answer", "incorrect"),
        ];
        assert_eq!(
            submittable_questions(&questions, &[banked("q1")], "a001"),
            vec!["q2", "q4"]
        );
    }
}
