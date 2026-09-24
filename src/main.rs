use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::{SystemTime, UNIX_EPOCH};

mod cs2100;

type AppResult<T> = Result<T, String>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl Difficulty {
    fn parse(value: &str) -> Option<Self> {
        match value.trim().to_ascii_lowercase().as_str() {
            "easy" => Some(Self::Easy),
            "medium" => Some(Self::Medium),
            "hard" => Some(Self::Hard),
            _ => None,
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            Self::Easy => "easy",
            Self::Medium => "medium",
            Self::Hard => "hard",
        }
    }
}

#[derive(Debug, Clone)]
struct Question {
    id: String,
    heading: String,
    prompt: String,
    answer: String,
    explanation: String,
}

#[derive(Debug, Clone)]
struct AttemptQuestion {
    id: String,
    difficulty: Difficulty,
    response: String,
    result: String,
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("error: {message}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> AppResult<()> {
    let args: Vec<String> = env::args().skip(1).collect();
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
        [command, rest @ ..] if command == "baseconv" => cs2100::baseconv::run(rest),
        _ => {
            print_help();
            Err("unknown or incomplete command".into())
        }
    }
}

fn print_help() {
    println!(
        "Recall\n\n\
         usage:\n\
           recall subject add <subject>              create a subject\n\
           recall add <subject> <file>               move a resource into the subject\n\
           recall add <subject> <file> --copy        copy instead of move\n\
           recall start [quiz] [--take N]            new attempt (optionally N questions)\n\
           recall submit <attempt> [qid ...]         grade one question, or all of them\n\
           recall reveal <attempt> <qid>             show the model answer (self-check)\n\
           recall mark <attempt> <qid> correct|incorrect\n\
           recall baseconv <value> [-b N] [-i base]  CS2100 number conversions\n\
           recall                                    dashboard\n\n\
         Every question is worth points: easy 10, medium 20, hard 30.\n\
         Correct answers build a streak; wrong answers reset it.\n"
    );
}

/// Parse the arguments to `start`: an optional quiz name plus `--take N`.
fn parse_start_args(args: &[String]) -> AppResult<(Option<String>, Option<usize>)> {
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

fn subject_add(raw_subject: &str) -> AppResult<()> {
    let root = env::current_dir().map_err(io_error)?;
    let subject = validate_slug(raw_subject, "subject")?;
    fs::write(root.join(".recall"), "Recall project\n").map_err(io_error)?;
    let base = root.join("subjects").join(&subject);
    for directory in ["resources", "quizzes", "attempts"] {
        fs::create_dir_all(base.join(directory)).map_err(io_error)?;
    }
    let progress = base.join("progress.md");
    if !progress.exists() {
        fs::write(
            &progress,
            "# Progress\n\n| date | attempt | question | difficulty | result |\n|---|---|---|---|---|\n",
        )
        .map_err(io_error)?;
    }
    println!(
        "Added subject `{subject}` at {}",
        display_path(&base, &root)
    );
    Ok(())
}

fn move_resource(root: &Path, raw_subject: &str, raw_file: &str, copy: bool) -> AppResult<()> {
    let subject = validate_slug(raw_subject, "subject")?;
    let subject_dir = root.join("subjects").join(&subject);
    if !subject_dir.is_dir() {
        return Err(format!(
            "subject `{subject}` does not exist; run `recall subject add {subject}`"
        ));
    }
    let source = PathBuf::from(raw_file);
    if !source.is_file() {
        return Err(format!("resource file not found: {}", source.display()));
    }
    let file_name = source
        .file_name()
        .ok_or_else(|| "resource path has no file name".to_string())?;
    let destination = subject_dir.join("resources").join(file_name);
    if destination.exists() {
        return Err(format!(
            "resource already exists: {}",
            destination.display()
        ));
    }
    if copy {
        fs::copy(&source, &destination).map_err(io_error)?;
        println!(
            "Added resource (copied): {}",
            display_path(&destination, root)
        );
        return Ok(());
    }
    match fs::rename(&source, &destination) {
        Ok(()) => {}
        // A cross-device rename cannot succeed on its own; fall back to a
        // copy plus removal so the source is still moved, never left behind.
        Err(error) if error.raw_os_error() == Some(18) => {
            fs::copy(&source, &destination).map_err(io_error)?;
            fs::remove_file(&source).map_err(io_error)?;
        }
        Err(error) => return Err(io_error(error)),
    }
    println!(
        "Added resource (moved): {}",
        display_path(&destination, root)
    );
    Ok(())
}

fn start_attempt(root: &Path, query: Option<&str>, take: Option<usize>) -> AppResult<()> {
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
        "{quiz_title}\n\n<!-- attempt-id: {attempt_id} -->\n<!-- subject: {subject} -->\n<!-- source-quiz: {quiz_relative} -->\n<!-- submitted: no -->\n\n"
    );
    for (_, question) in &questions {
        output.push_str(&format!(
            "{}\n\n{}\n\n<!-- response:start -->\n\n<!-- response:end -->\n<!-- result: pending -->\n<!-- feedback:start -->\n\n<!-- feedback:end -->\n\n",
            question.heading,
            question.prompt.trim()
        ));
    }
    fs::write(&attempt_path, output).map_err(io_error)?;
    println!("Attempt `{attempt_id}` is ready — {drawn} question(s).");
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

/// Grade one or more questions and report the moment each one resolves.
///
/// Resolved questions are written to progress.md straight away, so points and
/// streaks arrive per question instead of only when the whole attempt is done.
fn submit_attempt(root: &Path, args: &[String]) -> AppResult<()> {
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

    // With no question ids, sweep the whole attempt; otherwise only what was asked.
    let targets: Vec<String> = if selected.is_empty() {
        attempted
            .iter()
            .map(|question| question.id.clone())
            .collect()
    } else {
        for id in &selected {
            if !attempted.iter().any(|question| &question.id == id) {
                return Err(format!("attempt `{attempt_id}` has no question `{id}`"));
            }
        }
        selected.clone()
    };

    let progress_path = root.join("subjects").join(&subject).join("progress.md");
    let history = read_progress_rows(&progress_path)?;
    let already_scored = |id: &str| {
        history
            .iter()
            .any(|row| row.attempt == attempt_id && row.question == id)
    };

    let mut waiting = Vec::new();
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

        if rule == "manual" {
            if attempted_question.result != "correct" && attempted_question.result != "incorrect" {
                waiting.push(id.clone());
            }
            continue;
        }

        let correct = grade_automatic(&question.answer, &attempted_question.response)?;
        attempted_question.result = if correct { "correct" } else { "incorrect" }.to_string();
        replace_question_marker(&mut attempt_text, id, "result", &attempted_question.result)?;
        if !correct && !question.explanation.trim().is_empty() {
            replace_question_block(
                &mut attempt_text,
                id,
                "feedback",
                question.explanation.trim(),
            )?;
        }
    }

    // Record everything that became known during this run, once each.
    let mut resolved = Vec::new();
    for id in &targets {
        if let Some(question) = attempted.iter().find(|question| &question.id == id)
            && (question.result == "correct" || question.result == "incorrect")
            && !already_scored(id)
        {
            resolved.push(ProgressRow {
                attempt: attempt_id.clone(),
                question: question.id.clone(),
                difficulty: question.difficulty,
                result: question.result.clone(),
            });
        }
    }
    append_progress_rows(&progress_path, &resolved)?;
    let final_history = read_progress_rows(&progress_path)?;

    // An attempt is only finished when every question is both answered and
    // banked; a question marked in the file but never submitted is not scored.
    let all_answered = attempted.iter().all(is_resolved);
    let all_recorded = attempted.iter().all(|question| {
        final_history
            .iter()
            .any(|row| row.attempt == attempt_id && row.question == question.id)
    });
    let complete = all_answered && all_recorded;
    if complete {
        replace_global_marker(&mut attempt_text, "submitted", "yes")?;
    }
    fs::write(&attempt_path, &attempt_text).map_err(io_error)?;

    // Suggestions must be unambiguous: every subject numbers its attempts from
    // a001, so always show the subject-qualified name in commands we print.
    let attempt_ref = format!("{subject}/{attempt_id}");
    report(
        &attempt_id,
        &attempt_ref,
        &quiz,
        &targets,
        &attempted,
        &resolved,
        &waiting,
        &history,
        &final_history,
        complete,
    );
    Ok(())
}

/// Print the per-question verdicts, the progress bar, points and streak.
#[allow(clippy::too_many_arguments)]
fn report(
    attempt_id: &str,
    attempt_ref: &str,
    quiz: &[Question],
    targets: &[String],
    attempted: &[AttemptQuestion],
    resolved: &[ProgressRow],
    waiting: &[String],
    history: &[ProgressRow],
    final_history: &[ProgressRow],
    complete: bool,
) {
    let mut streak = current_streak(history);
    let total = total_points(final_history);
    let mut run_points = 0_u32;
    let mut run_correct = 0_usize;
    let mut run_wrong = 0_usize;

    println!();
    for id in targets {
        let Some(question) = attempted.iter().find(|question| &question.id == id) else {
            continue;
        };
        let difficulty = question.difficulty.as_str();
        if waiting.contains(id) {
            println!("  ⏳ {id:<4} [{difficulty}]  free-response, not marked yet");
            continue;
        }
        match resolved.iter().find(|row| &row.question == id) {
            Some(row) if row.result == "correct" => {
                streak += 1;
                let points = base_points(row.difficulty);
                run_points += points;
                run_correct += 1;
                println!("  ✅ {id:<4} [{difficulty}]  +{points} pts    🔥 streak {streak}");
            }
            Some(_) => {
                streak = 0;
                run_wrong += 1;
                println!("  ❌ {id:<4} [{difficulty}]  +0 pts    streak reset");
                let explanation = quiz
                    .iter()
                    .find(|source| source.id == *id)
                    .map(|source| source.explanation.trim())
                    .unwrap_or_default();
                for line in explanation.lines().filter(|line| !line.trim().is_empty()) {
                    println!("       {line}");
                }
            }
            None => println!(
                "  •  {id:<4} [{difficulty}]  already scored ({})",
                question.result
            ),
        }
    }

    // Count banked questions, not merely resolved ones, so the bar agrees with
    // the points and streak sitting next to it on the same line.
    let done = final_history
        .iter()
        .filter(|row| row.attempt == attempt_id)
        .count();
    let total_questions = attempted.len();
    println!();
    println!(
        "  {} {done}/{total_questions}   ⭐ {total} pts   🔥 streak {streak}   🏆 best {}",
        progress_bar(done, total_questions, 24),
        best_streak(final_history)
    );

    if targets.len() > 1 && run_correct + run_wrong > 0 {
        println!("  this run: +{run_points} pts · {run_correct} correct · {run_wrong} missed");
    }

    if complete {
        let correct = attempted
            .iter()
            .filter(|question| question.result == "correct")
            .count();
        println!();
        println!("  🎉 attempt `{attempt_id}` complete — {correct}/{total_questions} correct");
        print_counts(attempted);
        let (level, into, need) = level_for(total);
        println!(
            "  level {level}  {} {into}/{need} pts to level {}",
            progress_bar(into as usize, need as usize, 20),
            level + 1
        );
    } else if !waiting.is_empty() {
        println!();
        println!("  {} unmarked: {}", waiting.len(), waiting.join(", "));
        println!(
            "  mark them, then:  recall submit {attempt_ref} {}",
            waiting[0]
        );
    } else if done < total_questions {
        println!();
        println!("  {done}/{total_questions} banked. Record the rest from the file with:");
        println!("    recall submit {attempt_ref}");
    }
}

/// Everything needed to act on a single question of an attempt.
struct AttemptView {
    path: PathBuf,
    subject: String,
    attempt_id: String,
    quiz: Vec<Question>,
    attempted: Vec<AttemptQuestion>,
}

impl AttemptView {
    /// `subject/attempt-id` — unambiguous across subjects in printed commands.
    fn reference(&self) -> String {
        format!("{}/{}", self.subject, self.attempt_id)
    }
}

fn load_attempt(root: &Path, query: &str) -> AppResult<AttemptView> {
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

fn index_question<'a>(
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

fn is_resolved(question: &AttemptQuestion) -> bool {
    question.result == "correct" || question.result == "incorrect"
}

/// Show the model answer so a free-response question can be self-checked.
fn reveal_question(root: &Path, args: &[String]) -> AppResult<()> {
    if args.len() != 2 {
        return Err("usage: recall reveal <attempt> <question-id>".into());
    }
    let view = load_attempt(root, &args[0])?;
    let (question, attempted) = index_question(&view, &args[1])?;
    let id = &question.id;

    // Don't leak the answer to a question that has not been attempted yet.
    if attempted.response.trim().is_empty() && !is_resolved(attempted) {
        return Err(format!(
            "`{id}` has no answer yet — write your response in the attempt file first"
        ));
    }

    let rule = question.answer.lines().next().unwrap_or("manual").trim();
    println!();
    println!("  ── {id} [{}] ──", attempted.difficulty.as_str());
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
fn mark_question(root: &Path, args: &[String]) -> AppResult<()> {
    if args.len() != 3 {
        return Err("usage: recall mark <attempt> <question-id> correct|incorrect".into());
    }
    let verdict = match args[2].trim().to_ascii_lowercase().as_str() {
        "correct" | "c" | "yes" | "y" | "right" | "1" => "correct",
        "incorrect" | "wrong" | "no" | "n" | "0" => "incorrect",
        other => {
            return Err(format!(
                "`{other}` is not a verdict; use correct or incorrect"
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
            "`{}` has no response yet — answer it in the attempt file first",
            question.id
        ));
    }

    let mut text = fs::read_to_string(&view.path).map_err(io_error)?;
    replace_question_marker(&mut text, &question.id, "result", verdict)?;
    fs::write(&view.path, text).map_err(io_error)?;

    // Hand off to the normal submit path so scoring lives in one place.
    submit_attempt(root, &[args[0].clone(), args[1].clone()])
}

fn dashboard(root: &Path) -> AppResult<()> {
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
        let total = rows.len();
        let correct = rows.iter().filter(|row| row.result == "correct").count();
        let points = total_points(&rows);
        let (level, into, need) = level_for(points);
        println!("{subject}: {correct}/{total} correct   ⭐ {points} pts   level {level}");
        for difficulty in [Difficulty::Easy, Difficulty::Medium, Difficulty::Hard] {
            let matching: Vec<_> = rows
                .iter()
                .filter(|row| row.difficulty == difficulty)
                .collect();
            let solved = matching
                .iter()
                .filter(|row| row.result == "correct")
                .count();
            println!("  {:6} {solved}/{}", difficulty.as_str(), matching.len());
        }
        println!(
            "  {} {into}/{need} to level {}",
            progress_bar(into as usize, need as usize, 20),
            level + 1
        );
        let streak = current_streak(&rows);
        if streak > 0 {
            println!(
                "  🔥 current streak {streak}   🏆 best {}",
                best_streak(&rows)
            );
        }
    }
    Ok(())
}

fn parse_quiz(text: &str) -> AppResult<Vec<Question>> {
    let blocks = question_blocks(text);
    let mut output = Vec::new();
    for (heading, body) in blocks {
        let (id, _difficulty) = parse_heading(&heading)?;
        let answer_start = body
            .find("<!-- answer:start -->")
            .ok_or_else(|| format!("question `{id}` is missing answer:start"))?;
        let answer_tail = &body[answer_start + "<!-- answer:start -->".len()..];
        let answer_end = answer_tail
            .find("<!-- answer:end -->")
            .ok_or_else(|| format!("question `{id}` is missing answer:end"))?;
        let prompt = body[..answer_start].trim().to_string();
        let answer = answer_tail[..answer_end].trim().to_string();
        let rest = &answer_tail[answer_end + "<!-- answer:end -->".len()..];
        let explanation = extract_block(rest, "explanation").unwrap_or_default();
        output.push(Question {
            id,
            heading,
            prompt,
            answer,
            explanation,
        });
    }
    Ok(output)
}

fn parse_attempt(text: &str) -> AppResult<Vec<AttemptQuestion>> {
    let blocks = question_blocks(text);
    let mut output = Vec::new();
    for (heading, body) in blocks {
        let (id, difficulty) = parse_heading(&heading)?;
        let response = extract_block(&body, "response")
            .ok_or_else(|| format!("question `{id}` is missing response markers"))?;
        let result = marker_value(&body, "result").unwrap_or_else(|| "pending".into());
        output.push(AttemptQuestion {
            id,
            difficulty,
            response,
            result,
        });
    }
    Ok(output)
}

fn question_blocks(text: &str) -> Vec<(String, String)> {
    let mut blocks = Vec::new();
    let mut heading: Option<String> = None;
    let mut body = String::new();
    for line in text.lines() {
        if line.starts_with("## ") {
            if let Some(previous) = heading.take() {
                blocks.push((previous, body.trim_end().to_string()));
                body.clear();
            }
            heading = Some(line.to_string());
        } else if heading.is_some() {
            body.push_str(line);
            body.push('\n');
        }
    }
    if let Some(previous) = heading {
        blocks.push((previous, body.trim_end().to_string()));
    }
    blocks
}

fn parse_heading(heading: &str) -> AppResult<(String, Difficulty)> {
    let content = heading.trim_start_matches("## ").trim();
    let left = content
        .rfind('[')
        .ok_or_else(|| format!("invalid question heading: {heading}"))?;
    let right = content
        .rfind(']')
        .ok_or_else(|| format!("invalid question heading: {heading}"))?;
    if right <= left {
        return Err(format!("invalid question heading: {heading}"));
    }
    let id = content[..left].trim().to_string();
    if id.is_empty() || id.contains(char::is_whitespace) {
        return Err(format!("question ID must be one word: `{id}`"));
    }
    let difficulty = Difficulty::parse(&content[left + 1..right])
        .ok_or_else(|| format!("invalid difficulty in `{heading}`"))?;
    Ok((id, difficulty))
}

fn grade_automatic(answer_block: &str, response: &str) -> AppResult<bool> {
    let mut lines = answer_block.lines();
    let rule = lines.next().unwrap_or("manual").trim();
    let expected: Vec<String> = lines
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .collect();
    match rule {
        "exact" => {
            let actual = normalize(response);
            Ok(expected.iter().any(|value| normalize(value) == actual))
        }
        "contains" => {
            let actual = normalize(response);
            Ok(!expected.is_empty()
                && expected
                    .iter()
                    .all(|value| actual.contains(&normalize(value))))
        }
        _ if rule.starts_with("numeric") => {
            let tolerance = rule
                .split_whitespace()
                .nth(1)
                .unwrap_or("0")
                .parse::<f64>()
                .map_err(|_| format!("invalid numeric tolerance: `{rule}`"))?;
            let target = expected
                .first()
                .ok_or_else(|| "numeric answer is missing its target".to_string())?
                .parse::<f64>()
                .map_err(|_| "numeric target is not a number".to_string())?;
            let Ok(actual) = response.trim().parse::<f64>() else {
                return Ok(false);
            };
            Ok((actual - target).abs() <= tolerance)
        }
        "manual" => Err("manual questions cannot be graded automatically".into()),
        _ => Err(format!("unknown grading rule `{rule}`")),
    }
}

fn normalize(value: &str) -> String {
    value
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_ascii_lowercase()
}

fn extract_block(text: &str, name: &str) -> Option<String> {
    let start_marker = format!("<!-- {name}:start -->");
    let end_marker = format!("<!-- {name}:end -->");
    let start = text.find(&start_marker)? + start_marker.len();
    let tail = &text[start..];
    let end = tail.find(&end_marker)?;
    Some(tail[..end].trim().to_string())
}

fn marker_value(text: &str, name: &str) -> Option<String> {
    let prefix = format!("<!-- {name}:");
    let start = text.find(&prefix)? + prefix.len();
    let tail = &text[start..];
    let end = tail.find("-->")?;
    Some(tail[..end].trim().to_string())
}

fn replace_global_marker(text: &mut String, name: &str, value: &str) -> AppResult<()> {
    let prefix = format!("<!-- {name}:");
    let start = text
        .find(&prefix)
        .ok_or_else(|| format!("missing marker `{name}`"))?;
    let end = text[start..]
        .find("-->")
        .ok_or_else(|| format!("unclosed marker `{name}`"))?
        + start
        + 3;
    text.replace_range(start..end, &format!("<!-- {name}: {value} -->"));
    Ok(())
}

fn question_range(text: &str, id: &str) -> AppResult<(usize, usize)> {
    let prefix = format!("## {id} [");
    let start = text
        .find(&prefix)
        .ok_or_else(|| format!("question `{id}` not found"))?;
    let next = text[start + prefix.len()..]
        .find("\n## ")
        .map(|offset| start + prefix.len() + offset + 1)
        .unwrap_or(text.len());
    Ok((start, next))
}

fn replace_question_marker(text: &mut String, id: &str, name: &str, value: &str) -> AppResult<()> {
    let (start, end) = question_range(text, id)?;
    let mut block = text[start..end].to_string();
    replace_global_marker(&mut block, name, value)?;
    text.replace_range(start..end, &block);
    Ok(())
}

fn replace_question_block(text: &mut String, id: &str, name: &str, value: &str) -> AppResult<()> {
    let (start, end) = question_range(text, id)?;
    let mut block = text[start..end].to_string();
    let start_marker = format!("<!-- {name}:start -->");
    let end_marker = format!("<!-- {name}:end -->");
    let content_start = block
        .find(&start_marker)
        .ok_or_else(|| format!("missing {name}:start"))?
        + start_marker.len();
    let content_end = block[content_start..]
        .find(&end_marker)
        .ok_or_else(|| format!("missing {name}:end"))?
        + content_start;
    block.replace_range(content_start..content_end, &format!("\n{}\n", value.trim()));
    text.replace_range(start..end, &block);
    Ok(())
}

/// One scored question, as stored in `progress.md`.
#[derive(Debug, Clone)]
struct ProgressRow {
    attempt: String,
    question: String,
    difficulty: Difficulty,
    result: String,
}

fn append_progress_rows(path: &Path, rows: &[ProgressRow]) -> AppResult<()> {
    if rows.is_empty() {
        return Ok(());
    }
    let mut text = if path.exists() {
        fs::read_to_string(path).map_err(io_error)?
    } else {
        "# Progress\n\n| date | attempt | question | difficulty | result |\n|---|---|---|---|---|\n"
            .to_string()
    };
    let date = unix_timestamp().to_string();
    for row in rows {
        text.push_str(&format!(
            "| {date} | {} | {} | {} | {} |\n",
            row.attempt,
            row.question,
            row.difficulty.as_str(),
            row.result
        ));
    }
    fs::write(path, text).map_err(io_error)
}

fn read_progress_rows(path: &Path) -> AppResult<Vec<ProgressRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let text = fs::read_to_string(path).map_err(io_error)?;
    let mut rows = Vec::new();
    for line in text.lines().filter(|line| line.starts_with("| ")) {
        let fields: Vec<_> = line.split('|').map(str::trim).collect();
        if fields.len() >= 7
            && let Some(difficulty) = Difficulty::parse(fields[4])
        {
            rows.push(ProgressRow {
                attempt: fields[2].to_string(),
                question: fields[3].to_string(),
                difficulty,
                result: fields[5].to_string(),
            });
        }
    }
    Ok(rows)
}

/// Points for a correct answer, weighted by difficulty.
fn base_points(difficulty: Difficulty) -> u32 {
    match difficulty {
        Difficulty::Easy => 10,
        Difficulty::Medium => 20,
        Difficulty::Hard => 30,
    }
}

fn total_points(rows: &[ProgressRow]) -> u32 {
    rows.iter()
        .filter(|row| row.result == "correct")
        .map(|row| base_points(row.difficulty))
        .sum()
}

/// Correct answers at the very end of the history.
fn current_streak(rows: &[ProgressRow]) -> u32 {
    rows.iter()
        .rev()
        .take_while(|row| row.result == "correct")
        .count() as u32
}

fn best_streak(rows: &[ProgressRow]) -> u32 {
    let mut best = 0;
    let mut run = 0;
    for row in rows {
        if row.result == "correct" {
            run += 1;
            best = best.max(run);
        } else {
            run = 0;
        }
    }
    best
}

/// 100 points per level: (current level, points into it, points needed).
fn level_for(points: u32) -> (u32, u32, u32) {
    (points / 100 + 1, points % 100, 100)
}

fn progress_bar(done: usize, total: usize, width: usize) -> String {
    let filled = if total == 0 {
        0
    } else {
        (done * width + total / 2) / total
    }
    .min(width);
    format!("{}{}", "█".repeat(filled), "░".repeat(width - filled))
}

/// Fisher-Yates with a small LCG, keeping the crate dependency-free.
fn shuffle<T>(items: &mut [T], seed: u64) {
    let mut state = seed | 1;
    for index in (1..items.len()).rev() {
        state = state
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        let swap_with = ((state >> 33) as usize) % (index + 1);
        items.swap(index, swap_with);
    }
}

fn random_seed() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| (duration.as_secs() << 20) ^ u64::from(duration.subsec_nanos()))
        .unwrap_or(0x5EED)
}

fn print_counts(questions: &[AttemptQuestion]) {
    for difficulty in [Difficulty::Easy, Difficulty::Medium, Difficulty::Hard] {
        let matching: Vec<_> = questions
            .iter()
            .filter(|question| question.difficulty == difficulty)
            .collect();
        let correct = matching
            .iter()
            .filter(|question| question.result == "correct")
            .count();
        println!("  {:6}: {correct}/{}", difficulty.as_str(), matching.len());
    }
}

fn collect_markdown(root: &Path, folder: &str) -> AppResult<Vec<PathBuf>> {
    let subjects = root.join("subjects");
    let mut output = Vec::new();
    for subject in fs::read_dir(subjects).map_err(io_error)? {
        let path = subject.map_err(io_error)?.path().join(folder);
        if !path.is_dir() {
            continue;
        }
        for entry in fs::read_dir(path).map_err(io_error)? {
            let path = entry.map_err(io_error)?.path();
            if path.extension().and_then(|value| value.to_str()) == Some("md") {
                output.push(path);
            }
        }
    }
    output.sort();
    Ok(output)
}

fn resolve_file(
    root: &Path,
    candidates: &[PathBuf],
    query: Option<&str>,
    kind: &str,
) -> AppResult<PathBuf> {
    if candidates.is_empty() {
        return Err(format!("no {kind}s found"));
    }
    if query.is_none() {
        if candidates.len() == 1 {
            return Ok(candidates[0].clone());
        }
        let available = candidates
            .iter()
            .map(|path| display_path(path, root))
            .collect::<Vec<_>>()
            .join("\n  ");
        return Err(format!("choose a {kind}:\n  {available}"));
    }
    let query = query.unwrap();
    let direct = PathBuf::from(query);
    let direct = if direct.is_absolute() {
        direct
    } else {
        root.join(direct)
    };
    if direct.is_file() {
        return Ok(direct);
    }
    let normalized = query.trim_end_matches(".md");
    let matches: Vec<_> = candidates
        .iter()
        .filter(|path| {
            let stem = path.file_stem().and_then(|v| v.to_str()).unwrap_or("");
            let subject = path
                .parent()
                .and_then(Path::parent)
                .and_then(Path::file_name)
                .and_then(|v| v.to_str())
                .unwrap_or("");
            stem == normalized || format!("{subject}/{stem}") == normalized
        })
        .cloned()
        .collect();
    match matches.as_slice() {
        [only] => Ok(only.clone()),
        [] => Err(format!("{kind} `{query}` not found")),
        _ => {
            // Every subject numbers its own attempts from a001, so the same id
            // can exist several times. Show the names that would disambiguate.
            let options = matches
                .iter()
                .filter_map(|path| {
                    let stem = path.file_stem()?.to_str()?;
                    let subject = path.parent()?.parent()?.file_name()?.to_str()?;
                    Some(format!("{subject}/{stem}"))
                })
                .collect::<Vec<_>>()
                .join(",  ");
            Err(format!(
                "{kind} `{query}` exists in more than one subject; try one of:  {options}"
            ))
        }
    }
}

fn next_attempt_id(directory: &Path) -> AppResult<String> {
    let mut highest = 0_u32;
    for entry in fs::read_dir(directory).map_err(io_error)? {
        let name = entry
            .map_err(io_error)?
            .file_name()
            .to_string_lossy()
            .into_owned();
        if let Some(number) = name
            .strip_prefix('a')
            .and_then(|value| value.strip_suffix(".md"))
            .and_then(|value| value.parse::<u32>().ok())
        {
            highest = highest.max(number);
        }
    }
    Ok(format!("a{:03}", highest + 1))
}

fn find_root() -> AppResult<PathBuf> {
    let current = env::current_dir().map_err(io_error)?;
    for candidate in current.ancestors() {
        if candidate.join(".recall").is_file() {
            return Ok(candidate.to_path_buf());
        }
    }
    Err("no Recall project found; run `recall subject add <subject>` first".into())
}

fn validate_slug(value: &str, label: &str) -> AppResult<String> {
    let normalized = value.trim().to_ascii_lowercase();
    if normalized.is_empty()
        || !normalized
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || ch == '-' || ch == '_')
    {
        return Err(format!(
            "{label} must contain only letters, numbers, `-`, or `_`"
        ));
    }
    Ok(normalized)
}

fn display_path(path: &Path, root: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .display()
        .to_string()
}

fn file_stem(path: &Path) -> AppResult<String> {
    path.file_stem()
        .and_then(|value| value.to_str())
        .map(str::to_string)
        .ok_or_else(|| "invalid file name".into())
}

fn unix_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

fn io_error(error: io::Error) -> String {
    error.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_and_grades_quiz() {
        let quiz = "# Test\n\n## q1 [easy]\n\nWhat?\n\n<!-- answer:start -->\nexact\nForty two\n42\n<!-- answer:end -->\n<!-- explanation:start -->\nBecause.\n<!-- explanation:end -->\n";
        let questions = parse_quiz(quiz).unwrap();
        assert_eq!(questions.len(), 1);
        assert!(grade_automatic(&questions[0].answer, "  FORTY   two ").unwrap());
        assert!(!grade_automatic(&questions[0].answer, "41").unwrap());
    }

    #[test]
    fn numeric_tolerance_works() {
        assert!(grade_automatic("numeric 0.1\n3.14", "3.2").unwrap());
        assert!(!grade_automatic("numeric 0.01\n3.14", "3.2").unwrap());
    }

    #[test]
    fn parses_attempt() {
        let attempt = "# Test\n\n## q1 [hard]\nQuestion\n<!-- response:start -->\nMy answer\n<!-- response:end -->\n<!-- result: pending -->\n";
        let questions = parse_attempt(attempt).unwrap();
        assert_eq!(questions[0].response, "My answer");
        assert_eq!(questions[0].difficulty, Difficulty::Hard);
    }

    fn row(question: &str, difficulty: Difficulty, result: &str) -> ProgressRow {
        ProgressRow {
            attempt: "a001".into(),
            question: question.into(),
            difficulty,
            result: result.into(),
        }
    }

    #[test]
    fn awards_points_by_difficulty() {
        assert_eq!(base_points(Difficulty::Easy), 10);
        assert_eq!(base_points(Difficulty::Medium), 20);
        assert_eq!(base_points(Difficulty::Hard), 30);
    }

    #[test]
    fn only_correct_answers_score() {
        let rows = vec![
            row("q1", Difficulty::Easy, "correct"),
            row("q2", Difficulty::Hard, "incorrect"),
            row("q3", Difficulty::Medium, "correct"),
        ];
        assert_eq!(total_points(&rows), 30);
    }

    #[test]
    fn best_streak_survives_a_miss() {
        let rows = vec![
            row("q1", Difficulty::Easy, "correct"),
            row("q2", Difficulty::Easy, "correct"),
            row("q3", Difficulty::Easy, "correct"),
            row("q4", Difficulty::Easy, "incorrect"),
            row("q5", Difficulty::Easy, "correct"),
        ];
        assert_eq!(best_streak(&rows), 3);
        assert_eq!(current_streak(&rows), 1);
    }

    #[test]
    fn levels_advance_every_hundred_points() {
        assert_eq!(level_for(0), (1, 0, 100));
        assert_eq!(level_for(120), (2, 20, 100));
    }

    #[test]
    fn progress_bar_fills_proportionally() {
        assert_eq!(progress_bar(0, 4, 4), "░░░░");
        assert_eq!(progress_bar(2, 4, 4), "██░░");
        assert_eq!(progress_bar(4, 4, 4), "████");
        assert_eq!(progress_bar(0, 0, 4), "░░░░");
    }

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

    #[test]
    fn shuffle_keeps_every_element() {
        let mut items: Vec<usize> = (0..20).collect();
        shuffle(&mut items, 0xDEAD_BEEF);
        let mut sorted = items.clone();
        sorted.sort_unstable();
        assert_eq!(sorted, (0..20).collect::<Vec<_>>());
    }
}
