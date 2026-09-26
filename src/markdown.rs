//! Reading and patching the Markdown that Recall owns.
//!
//! Quizzes and attempts share one block grammar: a `## id [difficulty]`
//! heading followed by an `answer` block, an `explanation` block, an optional
//! `hint` block, and -- in attempts -- `response`, `result`, `tries` and
//! `graded` markers. This module parses both and does the in-place marker
//! surgery that every command relies on.

use crate::model::*;

pub(crate) fn parse_quiz(text: &str) -> AppResult<Vec<Question>> {
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
        let hint = extract_block(rest, "hint").filter(|value| !value.trim().is_empty());
        output.push(Question {
            id,
            heading,
            prompt,
            answer,
            explanation,
            hint,
        });
    }
    Ok(output)
}

pub(crate) fn parse_attempt(text: &str) -> AppResult<Vec<AttemptQuestion>> {
    let blocks = question_blocks(text);
    let mut output = Vec::new();
    for (heading, body) in blocks {
        let (id, difficulty) = parse_heading(&heading)?;
        let response = extract_block(&body, "response")
            .ok_or_else(|| format!("question `{id}` is missing response markers"))?;
        let result = marker_value(&body, "result").unwrap_or_else(|| "pending".into());
        let tries = marker_value(&body, "tries")
            .and_then(|value| value.trim().parse::<u32>().ok())
            .unwrap_or(0);
        let graded = marker_value(&body, "graded")
            .and_then(|value| value.trim().parse::<u64>().ok())
            .unwrap_or(0);
        let notes = extract_block(&body, "notes").unwrap_or_default();
        output.push(AttemptQuestion {
            id,
            difficulty,
            response,
            result,
            tries,
            graded,
            notes,
        });
    }
    Ok(output)
}

pub(crate) fn question_blocks(text: &str) -> Vec<(String, String)> {
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

pub(crate) fn parse_heading(heading: &str) -> AppResult<(String, Difficulty)> {
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

pub(crate) fn extract_block(text: &str, name: &str) -> Option<String> {
    let start_marker = format!("<!-- {name}:start -->");
    let end_marker = format!("<!-- {name}:end -->");
    if let Some(start) = text.find(&start_marker) {
        let content_start = start + start_marker.len();
        let tail = &text[content_start..];
        let end = tail.find(&end_marker)?;
        return Some(tail[..end].trim().to_string());
    }
    let marker = format!("<!-- {name} -->");
    let content_start = text.find(&marker)? + marker.len();
    let tail = &text[content_start..];
    let next_marker = tail.find("<!-- ");
    let next_question = tail.find("\n## ");
    let end = [next_marker, next_question]
        .into_iter()
        .flatten()
        .min()
        .unwrap_or(tail.len());
    Some(tail[..end].trim().to_string())
}

pub(crate) fn marker_value(text: &str, name: &str) -> Option<String> {
    let prefix = format!("<!-- {name}:");
    let start = text.find(&prefix)? + prefix.len();
    let tail = &text[start..];
    let end = tail.find("-->")?;
    Some(tail[..end].trim().to_string())
}

pub(crate) fn replace_global_marker(text: &mut String, name: &str, value: &str) -> AppResult<()> {
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

pub(crate) fn question_range(text: &str, id: &str) -> AppResult<(usize, usize)> {
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

pub(crate) fn replace_question_marker(
    text: &mut String,
    id: &str,
    name: &str,
    value: &str,
) -> AppResult<()> {
    let (start, end) = question_range(text, id)?;
    let mut block = text[start..end].to_string();
    replace_global_marker(&mut block, name, value)?;
    text.replace_range(start..end, &block);
    Ok(())
}

/// Set a per-question marker, inserting it when the attempt predates it.
///
/// Attempts written before the `tries` marker existed are still valid work in
/// progress, so a missing marker must never make them unusable.
pub(crate) fn upsert_question_marker(
    text: &mut String,
    id: &str,
    name: &str,
    value: &str,
) -> AppResult<()> {
    let (start, end) = question_range(text, id)?;
    let mut block = text[start..end].to_string();
    if block.contains(&format!("<!-- {name}:")) {
        replace_global_marker(&mut block, name, value)?;
    } else {
        // Slot it in after the result marker so the block keeps its shape.
        let position = block
            .find("<!-- result:")
            .and_then(|offset| block[offset..].find("-->").map(|tail| offset + tail + 3))
            .unwrap_or(block.len());
        block.insert_str(position, &format!("\n<!-- {name}: {value} -->"));
    }
    text.replace_range(start..end, &block);
    Ok(())
}

pub(crate) fn replace_question_block(
    text: &mut String,
    id: &str,
    name: &str,
    value: &str,
) -> AppResult<()> {
    let (start, end) = question_range(text, id)?;
    let mut block = text[start..end].to_string();
    let (content_start, content_end) =
        if let Some(marker_start) = block.find(&format!("<!-- {name}:start -->")) {
            let content_start = marker_start + format!("<!-- {name}:start -->").len();
            let end_marker = format!("<!-- {name}:end -->");
            let content_end = block[content_start..]
                .find(&end_marker)
                .ok_or_else(|| format!("missing {name}:end"))?
                + content_start;
            (content_start, content_end)
        } else {
            let marker = format!("<!-- {name} -->");
            let content_start = block
                .find(&marker)
                .ok_or_else(|| format!("missing {name} marker"))?
                + marker.len();
            let content_end = block[content_start..]
                .find("<!-- ")
                .or_else(|| block[content_start..].find("\n## "))
                .map(|offset| content_start + offset)
                .unwrap_or(block.len());
            (content_start, content_end)
        };
    let trailing = if name == "feedback" { "\n\n\n\n" } else { "\n" };
    block.replace_range(
        content_start..content_end,
        &format!("\n{}{trailing}", value.trim()),
    );
    text.replace_range(start..end, &block);
    Ok(())
}

/// Add to a block rather than replacing it.
///
/// The `feedback` block is where hints land, but students write their own
/// working in there too, so nothing already present is ever thrown away.
pub(crate) fn append_question_block(
    text: &mut String,
    id: &str,
    name: &str,
    value: &str,
) -> AppResult<()> {
    let value = value.trim();
    if value.is_empty() {
        return Ok(());
    }
    let (start, end) = question_range(text, id)?;
    let existing = extract_block(&text[start..end], name).unwrap_or_default();
    if existing.contains(value) {
        return Ok(());
    }
    let combined = if existing.trim().is_empty() {
        value.to_string()
    } else {
        format!("{}\n\n{value}", existing.trim())
    };
    replace_question_block(text, id, name, &combined)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grading::*;

    #[test]
    fn parses_and_grades_quiz() {
        let quiz = "# Test\n\n## q1 [easy]\n\nWhat?\n\n<!-- answer:start -->\nexact\nForty two\n42\n<!-- answer:end -->\n<!-- explanation:start -->\nBecause.\n<!-- explanation:end -->\n";
        let questions = parse_quiz(quiz).unwrap();
        assert_eq!(questions.len(), 1);
        assert!(grade_automatic(&questions[0].answer, "  FORTY   two ").unwrap());
        assert!(!grade_automatic(&questions[0].answer, "41").unwrap());
    }

    #[test]
    fn parses_attempt() {
        let attempt = "# Test\n\n## q1 [hard]\nQuestion\n<!-- response:start -->\nMy answer\n<!-- response:end -->\n<!-- result: pending -->\n";
        let questions = parse_attempt(attempt).unwrap();
        assert_eq!(questions[0].response, "My answer");
        assert_eq!(questions[0].difficulty, Difficulty::Hard);
    }

    #[test]
    fn parses_single_marker_attempt_blocks() {
        let attempt = "## q1 [hard]\nQuestion\n<!-- response -->\nMy answer\n<!-- notes -->\nScratch\n<!-- result: pending --> <!-- tries: 0 -->\n<!-- feedback -->\n";
        let questions = parse_attempt(attempt).unwrap();
        assert_eq!(questions[0].response, "My answer");
        assert_eq!(questions[0].notes, "Scratch");
        assert_eq!(questions[0].result, "pending");
    }

    #[test]
    fn feedback_updates_leave_questions_visually_separated() {
        let mut attempt = "## q1 [easy]\nQuestion\n<!-- response -->\nA\n<!-- notes -->\n<!-- result: retry --> <!-- tries: 1 -->\n<!-- feedback -->\nOld hint\n\n## q2 [easy]\nNext\n<!-- response -->\n\n<!-- notes -->\n<!-- result: pending --> <!-- tries: 0 -->\n<!-- feedback -->\n".to_string();
        append_question_block(&mut attempt, "q1", "feedback", "New hint").unwrap();
        assert!(attempt.contains("Old hint\n\nNew hint\n\n\n\n## q2"));
    }
}
