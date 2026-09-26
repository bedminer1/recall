//! Everything that touches the project directory.
//!
//! Reading and appending `progress.md`, finding the project root, resolving a
//! quiz or attempt by name, and the small helpers the rest of the crate uses.

use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{self};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::markdown::*;
use crate::model::*;
use crate::scoring::*;

pub(crate) fn append_progress_rows(path: &Path, rows: &[ProgressRow]) -> AppResult<()> {
    if rows.is_empty() {
        return Ok(());
    }
    let mut text = if path.exists() {
        fs::read_to_string(path).map_err(io_error)?
    } else {
        "# Progress\n\n| date | attempt | question | difficulty | result | tries |\n|---|---|---|---|---|---|\n"
            .to_string()
    };
    let date = unix_timestamp().to_string();
    for row in rows {
        text.push_str(&format!(
            "| {date} | {} | {} | {} | {} | {} |\n",
            row.attempt,
            row.question,
            row.difficulty.as_str(),
            row.result,
            row.tries
        ));
    }
    fs::write(path, text).map_err(io_error)
}

pub(crate) fn read_progress_rows(path: &Path) -> AppResult<Vec<ProgressRow>> {
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
            // Rows written before retries existed have no tries column.
            let tries = fields
                .get(6)
                .and_then(|value| value.parse::<u32>().ok())
                .unwrap_or(1);
            rows.push(ProgressRow {
                attempt: fields[2].to_string(),
                question: fields[3].to_string(),
                difficulty,
                result: fields[5].to_string(),
                tries,
            });
        }
    }
    Ok(rows)
}

/// Map every attempt to the quiz it came from, so history can be read per quiz.
pub(crate) fn quizzes_by_attempt(root: &Path, subject: &str) -> AppResult<HashMap<String, String>> {
    let mut map = HashMap::new();
    let attempts = root.join("subjects").join(subject).join("attempts");
    if !attempts.is_dir() {
        return Ok(map);
    }
    for entry in fs::read_dir(&attempts).map_err(io_error)? {
        let path = entry.map_err(io_error)?.path();
        if path.extension().and_then(|value| value.to_str()) != Some("md") {
            continue;
        }
        let Ok(text) = fs::read_to_string(&path) else {
            continue;
        };
        if let (Some(attempt), Some(quiz)) = (
            marker_value(&text, "attempt-id"),
            marker_value(&text, "source-quiz"),
        ) {
            map.insert(attempt, quiz_stem_of(&quiz));
        }
    }
    Ok(map)
}

/// Fisher-Yates with a small LCG, keeping the crate dependency-free.
pub(crate) fn shuffle<T>(items: &mut [T], seed: u64) {
    let mut state = seed | 1;
    for index in (1..items.len()).rev() {
        state = state
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        let swap_with = ((state >> 33) as usize) % (index + 1);
        items.swap(index, swap_with);
    }
}

pub(crate) fn random_seed() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| (duration.as_secs() << 20) ^ u64::from(duration.subsec_nanos()))
        .unwrap_or(0x5EED)
}

pub(crate) fn collect_markdown(root: &Path, folder: &str) -> AppResult<Vec<PathBuf>> {
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

pub(crate) fn resolve_file(
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

pub(crate) fn next_attempt_id(directory: &Path) -> AppResult<String> {
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

pub(crate) fn find_root() -> AppResult<PathBuf> {
    let current = env::current_dir().map_err(io_error)?;
    for candidate in current.ancestors() {
        if candidate.join(".recall").is_file() {
            return Ok(candidate.to_path_buf());
        }
    }
    Err("no Recall project found; run `recall subject add <subject>` first".into())
}

pub(crate) fn validate_slug(value: &str, label: &str) -> AppResult<String> {
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

pub(crate) fn display_path(path: &Path, root: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .display()
        .to_string()
}

pub(crate) fn file_stem(path: &Path) -> AppResult<String> {
    path.file_stem()
        .and_then(|value| value.to_str())
        .map(str::to_string)
        .ok_or_else(|| "invalid file name".into())
}

pub(crate) fn unix_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

pub(crate) fn io_error(error: io::Error) -> String {
    error.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shuffle_keeps_every_element() {
        let mut items: Vec<usize> = (0..20).collect();
        shuffle(&mut items, 0xDEAD_BEEF);
        let mut sorted = items.clone();
        sorted.sort_unstable();
        assert_eq!(sorted, (0..20).collect::<Vec<_>>());
    }
}
