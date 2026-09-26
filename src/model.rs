//! Shared data types.
//!
//! These are the shapes that move between modules: a parsed `Question` from a
//! quiz, an `AttemptQuestion` carrying a student's response, a `ProgressRow`
//! from the ledger, and the small value types used for reporting.

pub(crate) type AppResult<T> = Result<T, String>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl Difficulty {
    pub(crate) fn parse(value: &str) -> Option<Self> {
        match value.trim().to_ascii_lowercase().as_str() {
            "easy" => Some(Self::Easy),
            "medium" => Some(Self::Medium),
            "hard" => Some(Self::Hard),
            _ => None,
        }
    }

    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::Easy => "easy",
            Self::Medium => "medium",
            Self::Hard => "hard",
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Question {
    pub(crate) id: String,
    pub(crate) heading: String,
    pub(crate) prompt: String,
    pub(crate) answer: String,
    pub(crate) explanation: String,
    /// Optional authored nudge, used in preference to the mechanical one.
    pub(crate) hint: Option<String>,
}

#[derive(Debug, Clone)]
pub(crate) struct AttemptQuestion {
    pub(crate) id: String,
    pub(crate) difficulty: Difficulty,
    pub(crate) response: String,
    pub(crate) result: String,
    pub(crate) tries: u32,
    /// Hash of the response as last graded, so re-grading an unchanged answer
    /// from a full sweep cannot charge the wrong-answer penalty twice.
    pub(crate) graded: u64,
    /// The student's own working. Never graded, only read back to explain why a
    /// question with notes but no answer could not be graded.
    pub(crate) notes: String,
}

/// The two accuracies worth showing: the whole subject, and the quiz in hand.
pub(crate) struct Summary {
    pub(crate) subject: String,
    pub(crate) subject_accuracy: Accuracy,
    pub(crate) quiz: String,
    pub(crate) quiz_accuracy: Accuracy,
}

pub(crate) fn is_resolved(question: &AttemptQuestion) -> bool {
    question.result == "correct" || question.result == "incorrect"
}

/// One scored question, as stored in `progress.md`.
#[derive(Debug, Clone)]
pub(crate) struct ProgressRow {
    pub(crate) attempt: String,
    pub(crate) question: String,
    pub(crate) difficulty: Difficulty,
    pub(crate) result: String,
    pub(crate) tries: u32,
}

/// Correct-out-of-total for one scope: a whole subject, or a single quiz.
#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct Accuracy {
    pub(crate) correct: usize,
    pub(crate) total: usize,
}

impl Accuracy {
    pub(crate) fn percent(&self) -> u32 {
        if self.total == 0 {
            0
        } else {
            ((self.correct * 100 + self.total / 2) / self.total) as u32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrong_answers_are_not_terminal() {
        let question = |result: &str| AttemptQuestion {
            id: "q1".into(),
            difficulty: Difficulty::Easy,
            response: "nope".into(),
            result: result.into(),
            tries: 1,
            graded: 0,
            notes: String::new(),
        };
        // retry is the non-final state; only a verdict settles a question
        assert!(!is_resolved(&question("retry")));
        assert!(!is_resolved(&question("pending")));
        assert!(is_resolved(&question("incorrect")));
        assert!(is_resolved(&question("correct")));
    }
}
