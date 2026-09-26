//! Everything printed to the terminal.
//!
//! `report` renders one submit (per-question verdicts, the run aggregate, the
//! two accuracies and the LP to next rank); `dashboard` is the summary for
//! `recall` with no arguments. Colours and the rank-change banner live here.

use std::env;
use std::fs;
use std::io::{self, IsTerminal};
use std::path::Path;

use crate::grading::*;
use crate::model::*;
use crate::scoring::*;
use crate::store::*;

/// Print the per-question verdicts, the two accuracies, and the LP to next rank.
#[allow(clippy::too_many_arguments)]
pub(crate) fn report(
    attempt_id: &str,
    attempt_ref: &str,
    quiz: &[Question],
    targets: &[String],
    attempted: &[AttemptQuestion],
    resolved: &[ProgressRow],
    wrong_attempts: &[ProgressRow],
    waiting: &[String],
    empty: &[String],
    history: &[ProgressRow],
    final_history: &[ProgressRow],
    complete: bool,
    completion_bonus: u32,
    summary: &Summary,
) {
    let lp = total_lp(final_history);
    let raw_rank = rank_for(lp);
    let rank = rank_for_performance(lp, summary.subject_accuracy);
    let rank_before = rank_for_performance(total_lp(history), accuracy_of(history.iter()));
    let mut run_retry = 0_usize;

    println!();
    for id in targets {
        let Some(question) = attempted.iter().find(|question| &question.id == id) else {
            continue;
        };
        let explanation = quiz
            .iter()
            .find(|source| source.id == *id)
            .map(|source| source.explanation.trim())
            .unwrap_or_default();

        if empty.contains(id) {
            println!("  [ ] {id}  no answer written yet");
            continue;
        }

        if let Some(row) = resolved.iter().find(|row| &row.question == id) {
            if row.result == "correct" {
                let reward = points_for(row.difficulty, "correct", row.tries);
                let note = if row.tries > 1 {
                    format!(
                        "   (try {}, after -{} LP)",
                        row.tries,
                        wrong_penalty() * i64::from(row.tries - 1)
                    )
                } else {
                    String::new()
                };
                println!("  [+] {id}  +{reward} LP{note}");
            } else {
                println!("  [-] {id}  missed");
                for line in explanation.lines().filter(|line| !line.trim().is_empty()) {
                    println!("      {line}");
                }
            }
            continue;
        }

        match question.result.as_str() {
            "retry" => {
                run_retry += 1;
                let hint = quiz
                    .iter()
                    .find(|source| source.id == *id)
                    .map(|source| hint_text(source, &question.response, question.tries))
                    .unwrap_or_default();
                let charged = wrong_attempts.iter().any(|row| row.question == *id);
                let penalty = if charged {
                    format!("   -{} LP", wrong_penalty())
                } else {
                    String::new()
                };
                println!("  [~] {id}  {hint}{penalty}");
                if question.tries >= 3 {
                    println!("      answer + take the miss:  recall giveup {attempt_ref} {id}");
                } else {
                    println!("      edit it, then:  recall submit {attempt_ref} {id}");
                }
            }
            "pending" => println!("  [ ] {id}  awaiting a mark"),
            other => println!("  [.] {id}  already scored ({other})"),
        }
    }

    let done = final_history
        .iter()
        .filter(|row| row.attempt == attempt_id && row.result != "bonus")
        .count();
    let total_questions = attempted.len();
    let next = if raw_rank.step > rank.step {
        rank_for(lp_to_reach_step(rank.step + 1))
    } else {
        next_rank(lp)
    };

    // One aggregate line for everything graded in this run.
    let run_correct = resolved
        .iter()
        .filter(|row| row.result == "correct")
        .count();
    let run_missed = resolved
        .iter()
        .filter(|row| row.result == "incorrect")
        .count();
    let run_penalised = wrong_attempts.len();
    let run_lp: i64 = resolved
        .iter()
        .filter(|row| row.result == "correct")
        .map(|row| i64::from(points_for(row.difficulty, "correct", row.tries)))
        .sum::<i64>()
        - wrong_penalty() * run_penalised as i64
        + i64::from(completion_bonus);

    println!();
    if run_correct + run_missed + run_penalised > 0 {
        println!(
            "  this run  {run_lp:+} LP   {run_correct} correct   {run_penalised} to retry   {run_missed} missed"
        );
    }
    println!(
        "  {:<16} {:>3}%  ({}/{})   all quizzes",
        summary.subject,
        summary.subject_accuracy.percent(),
        summary.subject_accuracy.correct,
        summary.subject_accuracy.total
    );
    println!(
        "  {:<16} {:>3}%  ({}/{})   this quiz",
        summary.quiz,
        summary.quiz_accuracy.percent(),
        summary.quiz_accuracy.correct,
        summary.quiz_accuracy.total
    );
    if raw_rank.step > rank.step {
        println!(
            "  {} accuracy gate: {}% needed for {}   (grade {})",
            paint(&format!("{:<16}", rank.label()), rank.color()),
            accuracy_required_for_step(rank.step + 1),
            next.label(),
            rank.grade
        );
    } else {
        println!(
            "  {} {} LP to {}   (grade {})",
            paint(&format!("{:<16}", rank.label()), rank.color()),
            rank.cost - rank.lp,
            next.label(),
            rank.grade
        );
    }

    if rank.step != rank_before.step {
        rank_change_banner(&rank_before, &rank);
    }

    if complete {
        // Same first-attempt rule as the headline accuracy, so the two agree.
        let attempt_accuracy =
            accuracy_of(final_history.iter().filter(|row| row.attempt == attempt_id));
        println!();
        println!(
            "  attempt `{attempt_id}` complete - {}%  ({}/{})",
            attempt_accuracy.percent(),
            attempt_accuracy.correct,
            attempt_accuracy.total
        );
        if completion_bonus > 0 {
            println!("  PAPER COMPLETE  +{completion_bonus} LP accuracy bonus");
        }
    } else if run_retry > 0 {
        println!();
        println!("  {run_retry} still open - retrying always beats giving up.");
    } else if !waiting.is_empty() {
        println!();
        println!(
            "  {} awaiting a mark: {}",
            waiting.len(),
            waiting.join(", ")
        );
        println!("  answer:  recall reveal {attempt_ref} {}", waiting[0]);
    } else if done < total_questions {
        println!();
        println!("  {done}/{total_questions} banked - recall submit {attempt_ref} for the rest");
    }
}

pub(crate) fn dashboard(root: &Path) -> AppResult<()> {
    let subjects = root.join("subjects");
    if !subjects.is_dir() {
        return Err(
            "no Recall project found; run `recall subject add <subject>` in a new directory".into(),
        );
    }
    println!("Recall progress\n");
    let mut entries: Vec<_> = fs::read_dir(&subjects)
        .map_err(io_error)?
        .filter_map(Result::ok)
        .collect();
    entries.sort_by_key(|entry| entry.file_name());
    for entry in entries {
        if !entry.path().is_dir() {
            continue;
        }
        let subject = entry.file_name().to_string_lossy().into_owned();
        let rows = read_progress_rows(&entry.path().join("progress.md"))?;
        let accuracy = accuracy_of(rows.iter());
        let lp = total_lp(&rows);
        let rank = rank_for_performance(lp, accuracy);
        let raw_rank = rank_for(lp);
        let next = if raw_rank.step > rank.step {
            rank_for(lp_to_reach_step(rank.step + 1))
        } else {
            next_rank(lp)
        };
        println!(
            "  {:<16} {:>3}%  ({}/{})",
            subject,
            accuracy.percent(),
            accuracy.correct,
            accuracy.total
        );
        if raw_rank.step > rank.step {
            println!(
                "  {} {}% accuracy needed for {}",
                paint(&format!("{:<16}", rank.label()), rank.color()),
                accuracy_required_for_step(rank.step + 1),
                next.label()
            );
        } else {
            println!(
                "  {} {} LP to {}",
                paint(&format!("{:<16}", rank.label()), rank.color()),
                rank.cost - rank.lp,
                next.label()
            );
        }
    }
    Ok(())
}

/// Plot one subject's cumulative rank after every ledger event.
pub(crate) fn rank_chart(root: &Path, raw_subject: &str) -> AppResult<()> {
    let subject = validate_slug(raw_subject, "subject")?;
    let subject_dir = root.join("subjects").join(&subject);
    if !subject_dir.is_dir() {
        return Err(format!("subject `{subject}` does not exist"));
    }
    let rows = read_progress_rows(&subject_dir.join("progress.md"))?;
    if rows.is_empty() {
        return Err(format!("subject `{subject}` has no scored questions yet"));
    }

    let history = tier_history(&rows);
    let terminal_columns = env::var("COLUMNS")
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(100);
    let plot_width = terminal_columns.saturating_sub(19).clamp(20, 100);
    let plotted = sample_history(&history, plot_width);
    let current_lp = total_lp(&rows);
    let current_accuracy = accuracy_of(rows.iter());
    let current = rank_for_performance(current_lp, current_accuracy);
    let current_tier = tier_index(&current);
    let top_tier = (current_tier + 2).min(9);

    println!("Rank history: {subject}\n");
    for tier in (0..=top_tier).rev() {
        let label = tier_label(tier);
        print!("  {label:>12} │");
        for &point in &plotted {
            if point.min(9) == tier {
                print!("●");
            } else {
                print!(" ");
            }
        }
        println!();
    }
    println!("  {:>12} └{}", "", "─".repeat(plotted.len()));
    println!(
        "  {:>12}  start{:>width$}",
        "",
        format!("{} events", rows.len()),
        width = plotted.len().saturating_sub(5)
    );
    println!();
    println!(
        "  Current: {} ({} LP, {}% first-attempt accuracy)",
        current.label(),
        current_lp,
        current_accuracy.percent()
    );
    Ok(())
}

/// Tier after each progress row, including the zero-LP starting point.
pub(crate) fn tier_history(rows: &[ProgressRow]) -> Vec<u32> {
    let mut history = Vec::with_capacity(rows.len() + 1);
    history.push(0);
    for end in 1..=rows.len() {
        history.push(tier_index(&rank_for_performance(
            total_lp(&rows[..end]),
            accuracy_of(rows[..end].iter()),
        )));
    }
    history
}

fn tier_index(rank: &Rank) -> u32 {
    if rank.step < DIVISION_STEPS {
        rank.step / DIVISIONS_PER_TIER
    } else {
        7 + (rank.step - DIVISION_STEPS).min(2)
    }
}

fn tier_label(tier: u32) -> &'static str {
    match tier {
        0..=6 => DIVISION_TIERS[tier as usize],
        7 => "Master",
        8 => "Grandmaster",
        _ => "Challenger",
    }
}

fn sample_history(history: &[u32], width: usize) -> Vec<u32> {
    if history.len() <= width {
        return history.to_vec();
    }
    (0..width)
        .map(|column| {
            let index = column * (history.len() - 1) / (width - 1);
            history[index]
        })
        .collect()
}

pub(crate) fn color_enabled() -> bool {
    env::var_os("NO_COLOR").is_none() && io::stdout().is_terminal()
}

pub(crate) fn paint(text: &str, code: &str) -> String {
    if color_enabled() {
        format!("\x1b[{code}m{text}\x1b[0m")
    } else {
        text.to_string()
    }
}

/// ASCII banner shown when a rank changes, in the new tier's colour.
pub(crate) fn rank_change_banner(before: &Rank, after: &Rank) {
    let promoted = after.step > before.step;
    let (title, colour) = if promoted {
        ("R A N K   U P", after.color())
    } else {
        ("R A N K   D O W N", "38;5;160")
    };
    let movement = format!("{}  ->  {}", before.label(), after.label());
    let line = "=".repeat(34);
    println!();
    println!("  +{line}+");
    println!("  |{:^34}|", "");
    println!("  |{}|", paint(&format!("{title:^34}"), colour));
    println!("  |{}|", paint(&format!("{movement:^34}"), colour));
    println!("  |{:^34}|", format!("counts as grade {}", after.grade));
    println!("  |{:^34}|", "");
    println!("  +{line}+");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn row(question: &str, result: &str) -> ProgressRow {
        ProgressRow {
            attempt: "a001".into(),
            question: question.into(),
            difficulty: Difficulty::Hard,
            result: result.into(),
            tries: 1,
        }
    }

    #[test]
    fn tier_history_tracks_big_promotions_and_demotions() {
        let rows = vec![
            row("q1", "correct"),
            row("q2", "correct"),
            row("q3", "correct"),
            row("q4", "correct"),
            row("q5", "correct"),
            row("q6", "wrong"),
            row("q7", "wrong"),
        ];
        assert_eq!(tier_history(&rows), vec![0, 0, 0, 0, 0, 1, 1, 0]);
    }

    #[test]
    fn sampling_keeps_both_ends() {
        let history: Vec<u32> = (0..100).collect();
        let sampled = sample_history(&history, 20);
        assert_eq!((sampled.first(), sampled.last()), (Some(&0), Some(&99)));
    }
}
