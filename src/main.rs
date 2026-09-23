use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::{SystemTime, UNIX_EPOCH};

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
        [command, subject, file] if command == "add" => add_resource(&find_root()?, subject, file),
        [command] if command == "start" => start_attempt(&find_root()?, None),
        [command, quiz] if command == "start" => start_attempt(&find_root()?, Some(quiz)),
        [command, attempt] if command == "submit" => submit_attempt(&find_root()?, attempt),
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
           recall subject add <subject>\n\
           recall add <subject> <file>\n\
           recall start [quiz-name|subject/quiz-name]\n\
           recall submit <attempt-id|path>\n\
           recall                         show progress\n"
    );
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

fn add_resource(root: &Path, raw_subject: &str, raw_file: &str) -> AppResult<()> {
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
    fs::copy(&source, &destination).map_err(io_error)?;
    println!("Added resource: {}", display_path(&destination, root));
    Ok(())
}

fn start_attempt(root: &Path, query: Option<&String>) -> AppResult<()> {
    let quizzes = collect_markdown(root, "quizzes")?;
    let quiz = resolve_file(root, &quizzes, query.map(String::as_str), "quiz")?;
    let subject_dir = quiz
        .parent()
        .and_then(Path::parent)
        .ok_or_else(|| "quiz is not inside a subject".to_string())?;
    let subject = subject_dir
        .file_name()
        .and_then(|v| v.to_str())
        .ok_or_else(|| "invalid subject path".to_string())?;
    let quiz_text = fs::read_to_string(&quiz).map_err(io_error)?;
    let questions = parse_quiz(&quiz_text)?;
    if questions.is_empty() {
        return Err("quiz contains no questions".into());
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
    for question in questions {
        output.push_str(&format!(
            "{}\n\n{}\n\n<!-- response:start -->\n\n<!-- response:end -->\n<!-- result: pending -->\n<!-- feedback:start -->\n\n<!-- feedback:end -->\n\n",
            question.heading,
            question.prompt.trim()
        ));
    }
    fs::write(&attempt_path, output).map_err(io_error)?;
    println!("Attempt `{attempt_id}` is ready.");
    println!("Open: {}", display_path(&attempt_path, root));
    Ok(())
}

fn submit_attempt(root: &Path, query: &str) -> AppResult<()> {
    let attempts = collect_markdown(root, "attempts")?;
    let attempt_path = resolve_file(root, &attempts, Some(query), "attempt")?;
    let mut attempt_text = fs::read_to_string(&attempt_path).map_err(io_error)?;
    if marker_value(&attempt_text, "submitted").as_deref() == Some("yes") {
        println!("This attempt was already submitted; progress was not recorded twice.");
        return Ok(());
    }
    let quiz_relative = marker_value(&attempt_text, "source-quiz")
        .ok_or_else(|| "attempt is missing its source-quiz marker".to_string())?;
    let quiz_path = root.join(&quiz_relative);
    let quiz_text = fs::read_to_string(&quiz_path)
        .map_err(|_| format!("source quiz not found: {quiz_relative}"))?;
    let questions = parse_quiz(&quiz_text)?;
    let mut attempts_parsed = parse_attempt(&attempt_text)?;
    let mut pending = Vec::new();

    for attempted in &mut attempts_parsed {
        let question = questions
            .iter()
            .find(|question| question.id == attempted.id)
            .ok_or_else(|| format!("question `{}` is missing from source quiz", attempted.id))?;
        let grading_type = question.answer.lines().next().unwrap_or("manual").trim();
        if grading_type != "manual" {
            let result = grade_automatic(&question.answer, &attempted.response)?;
            attempted.result = if result { "correct" } else { "incorrect" }.into();
            replace_question_marker(
                &mut attempt_text,
                &attempted.id,
                "result",
                &attempted.result,
            )?;
            if !result && !question.explanation.trim().is_empty() {
                replace_question_block(
                    &mut attempt_text,
                    &attempted.id,
                    "feedback",
                    question.explanation.trim(),
                )?;
            }
        } else if attempted.result != "correct" && attempted.result != "incorrect" {
            pending.push(attempted.id.clone());
        }
    }

    fs::write(&attempt_path, &attempt_text).map_err(io_error)?;
    if !pending.is_empty() {
        println!("Automatic questions were graded.");
        println!("Pending Codex review: {}", pending.join(", "));
        println!(
            "Ask Codex to mark `{}` using the Recall skill, then run `recall submit {}` again.",
            display_path(&attempt_path, root),
            file_stem(&attempt_path)?
        );
        return Ok(());
    }

    let mut attempt_text = fs::read_to_string(&attempt_path).map_err(io_error)?;
    let completed = parse_attempt(&attempt_text)?;
    if completed
        .iter()
        .any(|question| question.result != "correct" && question.result != "incorrect")
    {
        return Err("every question must be marked correct or incorrect".into());
    }
    let subject = marker_value(&attempt_text, "subject")
        .ok_or_else(|| "attempt is missing its subject marker".to_string())?;
    let attempt_id = marker_value(&attempt_text, "attempt-id")
        .ok_or_else(|| "attempt is missing its attempt-id marker".to_string())?;
    append_progress(root, &subject, &attempt_id, &completed)?;
    replace_global_marker(&mut attempt_text, "submitted", "yes")?;
    fs::write(&attempt_path, &attempt_text).map_err(io_error)?;

    let correct = completed
        .iter()
        .filter(|question| question.result == "correct")
        .count();
    println!(
        "Submitted `{attempt_id}`: {correct}/{} correct",
        completed.len()
    );
    print_counts(&completed);
    Ok(())
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
        let rows = read_progress(&entry.path().join("progress.md"))?;
        let total = rows.len();
        let correct = rows
            .iter()
            .filter(|(_, result)| result == "correct")
            .count();
        println!("{subject}: {correct}/{total} correct");
        for difficulty in [Difficulty::Easy, Difficulty::Medium, Difficulty::Hard] {
            let matching: Vec<_> = rows.iter().filter(|(d, _)| *d == difficulty).collect();
            let solved = matching
                .iter()
                .filter(|(_, result)| result == "correct")
                .count();
            println!("  {:6} {solved}/{}", difficulty.as_str(), matching.len());
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

fn append_progress(
    root: &Path,
    subject: &str,
    attempt: &str,
    questions: &[AttemptQuestion],
) -> AppResult<()> {
    let progress = root.join("subjects").join(subject).join("progress.md");
    let mut text = fs::read_to_string(&progress).map_err(io_error)?;
    let date = unix_timestamp().to_string();
    for question in questions {
        text.push_str(&format!(
            "| {date} | {attempt} | {} | {} | {} |\n",
            question.id,
            question.difficulty.as_str(),
            question.result
        ));
    }
    fs::write(progress, text).map_err(io_error)
}

fn read_progress(path: &Path) -> AppResult<Vec<(Difficulty, String)>> {
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
            rows.push((difficulty, fields[5].to_string()));
        }
    }
    Ok(rows)
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
        _ => Err(format!(
            "{kind} `{query}` is ambiguous; include the subject name"
        )),
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
}
