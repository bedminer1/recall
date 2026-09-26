//! Grading one response, and the hint shown when it is wrong.
//!
//! `grade_automatic` applies the quiz's answer rule (`exact`, `numeric`,
//! `contains`). `hint_text` prefers a quiz's own authored hint and otherwise
//! falls back to `hint_for`, a mechanical nudge that points at the mistake
//! without giving the answer away.

use crate::model::*;

pub(crate) fn grade_automatic(answer_block: &str, response: &str) -> AppResult<bool> {
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

/// A nudge that points at the mistake without handing over the answer.
///
/// The hint gets more specific as `tries` grows, so a wrong answer invites
/// another attempt instead of ending the question.
pub(crate) fn hint_for(answer_block: &str, response: &str, tries: u32) -> String {
    let mut lines = answer_block.lines();
    let rule = lines.next().unwrap_or("manual").trim();
    let expected: Vec<String> = lines
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .collect();
    let actual = normalize(response);

    match rule {
        "exact" => {
            let squash = |value: &str| {
                value
                    .to_ascii_lowercase()
                    .chars()
                    .filter(char::is_ascii_alphanumeric)
                    .collect::<String>()
            };
            if expected
                .iter()
                .any(|value| squash(value) == squash(&actual))
            {
                return "so close - the content is right but the formatting is not. Check spacing, capitals or separators.".into();
            }
            if tries >= 3
                && let Some(first) = expected.first()
            {
                let head = first.chars().next().unwrap_or('?');
                return format!(
                    "still not it - the answer is {} characters long and starts with `{head}`.",
                    first.chars().count()
                );
            }
            if tries >= 2 {
                return "not it yet - compare your answer against what the question actually asks for. The form matters as much as the value.".into();
            }
            "not it yet - re-read the question, fix your response and submit again.".into()
        }
        "contains" => {
            let have = expected
                .iter()
                .filter(|value| actual.contains(&normalize(value)))
                .count();
            if have == 0 {
                "none of the required ideas are in your answer yet - the concept may be off entirely.".into()
            } else {
                format!(
                    "you have {have} of {} required ideas. Which one is missing?",
                    expected.len()
                )
            }
        }
        _ if rule.starts_with("numeric") => {
            let tolerance = rule
                .split_whitespace()
                .nth(1)
                .unwrap_or("0")
                .parse::<f64>()
                .unwrap_or(0.0);
            let Some(target) = expected.first().and_then(|value| value.parse::<f64>().ok()) else {
                return "not it yet - try again.".into();
            };
            let Ok(value) = response.trim().parse::<f64>() else {
                return "that is not a number - write the value as a plain number, then submit again."
                    .into();
            };
            if (value - target).abs() <= tolerance.max(0.0) * 3.0 {
                return "so close - check your rounding or the last digit.".into();
            }
            if value < target {
                "too low - your value is smaller than the answer.".into()
            } else {
                "too high - your value is larger than the answer.".into()
            }
        }
        _ => "not it yet - try again.".into(),
    }
}

/// The nudge to show: the quiz's own wording when it has one, otherwise the
/// mechanical hint derived from the grading rule.
pub(crate) fn hint_text(question: &Question, response: &str, tries: u32) -> String {
    match question.hint.as_deref() {
        Some(hint) if !hint.trim().is_empty() => hint.trim().to_string(),
        _ => hint_for(&question.answer, response, tries),
    }
}

pub(crate) fn normalize(value: &str) -> String {
    value
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_ascii_lowercase()
}

/// FNV-1a over the normalised response: enough to tell whether the student
/// changed their answer since it was last graded.
pub(crate) fn response_hash(value: &str) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;
    for byte in normalize(value).as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numeric_tolerance_works() {
        assert!(grade_automatic("numeric 0.1\n3.14", "3.2").unwrap());
        assert!(!grade_automatic("numeric 0.01\n3.14", "3.2").unwrap());
    }

    #[test]
    fn hint_reveals_direction_but_not_the_answer() {
        let numeric = "numeric 0\n10.5";
        let hint = hint_for(numeric, "4", 1);
        assert!(hint.contains("too low"), "{hint}");
        assert!(!hint.contains("10.5"), "hint leaked the answer: {hint}");

        let high = hint_for(numeric, "99", 1);
        assert!(high.contains("too high"), "{high}");

        let exact = "exact\nforty two\n42";
        assert!(hint_for(exact, "forty  two", 1).contains("formatting"));
        assert!(!hint_for(exact, "banana", 1).contains("42"));
    }
}
