//! League Points, the rank ladder, and accuracy.
//!
//! Points by difficulty and tries, the flat penalty for a wrong attempt, the
//! Iron-to-Challenger ladder mapped onto course grades, and the first-attempt
//! accuracy rule.

use std::collections::HashSet;
use std::path::Path;

use crate::model::*;

/// Full points for a first-try correct answer, weighted by difficulty.
pub(crate) fn base_points(difficulty: Difficulty) -> u32 {
    match difficulty {
        Difficulty::Easy => 20,
        Difficulty::Medium => 30,
        Difficulty::Hard => 40,
    }
}

/// Points for a correct answer: the base, halved and floored for every extra try.
///
/// So 30 becomes 30, 15, 7, 3, 1 across four retries. Retrying is never
/// punished to zero, because redeeming yourself should always beat giving up.
pub(crate) fn points_for(difficulty: Difficulty, result: &str, tries: u32) -> u32 {
    if result != "correct" {
        return 0;
    }
    let mut points = base_points(difficulty);
    for _ in 1..tries.max(1) {
        if points <= 1 {
            return 0;
        }
        points /= 2;
    }
    points
}

// ---------------------------------------------------------------- ranked play

/// Tiers that have three divisions each, lowest first.
pub(crate) const DIVISION_TIERS: [&str; 7] = [
    "Iron", "Bronze", "Silver", "Gold", "Platinum", "Emerald", "Diamond",
];

/// Each normal tier runs III -> II -> I. Division IV is intentionally omitted
/// to keep the early climb brisk.
pub(crate) const DIVISIONS_PER_TIER: u32 = 3;

/// LP required for one division in each tier. Early ranks move quickly, the
/// middle of the ladder ramps smoothly, and Diamond is deliberately sticky.
pub(crate) const TIER_LP_PER_DIVISION: [i64; 7] = [60, 65, 75, 85, 110, 125, 180];

/// LP required for each step at Master and above.
pub(crate) const ELITE_LP_PER_STEP: i64 = 250;

/// Number of divisions below Master (7 tiers x 3 divisions).
pub(crate) const DIVISION_STEPS: u32 = 21;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Rank {
    pub(crate) tier: &'static str,
    pub(crate) division: Option<u32>,
    /// League Points earned inside the current division.
    pub(crate) lp: i64,
    /// League Points required to leave the current division/rank.
    pub(crate) cost: i64,
    /// Position on the ladder, so promotions and demotions compare cleanly.
    pub(crate) step: u32,
    /// The course grade this rank is meant to correspond to.
    pub(crate) grade: &'static str,
}

impl Rank {
    pub(crate) fn label(&self) -> String {
        match self.division {
            Some(division) => format!("{} {}", self.tier, roman(division)),
            None => self.tier.to_string(),
        }
    }

    /// Roughly the tier's colour in the game.
    pub(crate) fn color(&self) -> &'static str {
        match self.tier {
            "Iron" => "38;5;245",
            "Bronze" => "38;5;130",
            "Silver" => "38;5;250",
            "Gold" => "38;5;220",
            "Platinum" => "38;5;44",
            "Emerald" => "38;5;41",
            "Diamond" => "38;5;39",
            "Master" => "38;5;135",
            "Grandmaster" => "38;5;160",
            _ => "38;5;51",
        }
    }
}

pub(crate) fn roman(division: u32) -> &'static str {
    match division {
        1 => "I",
        2 => "II",
        3 => "III",
        _ => "IV",
    }
}

pub(crate) fn tier_grade(tier: &str) -> &'static str {
    match tier {
        "Iron" => "D",
        "Bronze" => "D+",
        "Silver" => "C-",
        "Gold" => "C",
        "Platinum" => "C+",
        "Emerald" => "B-",
        "Diamond" => "B",
        "Master" => "A-",
        "Grandmaster" => "A",
        _ => "A+",
    }
}

/// Rank for a signed LP total. LP floors at zero, so Iron IV is rock bottom.
pub(crate) fn rank_for(total_lp: i64) -> Rank {
    let mut lp = total_lp.max(0);
    let mut step = 0_u32;
    loop {
        let cost = lp_cost_for_step(step);
        if lp < cost {
            break;
        }
        lp -= cost;
        step += 1;
    }
    let cost = lp_cost_for_step(step);
    if step < DIVISION_STEPS {
        let tier = DIVISION_TIERS[(step / DIVISIONS_PER_TIER) as usize];
        return Rank {
            tier,
            division: Some(DIVISIONS_PER_TIER - step % DIVISIONS_PER_TIER),
            lp,
            cost,
            step,
            grade: tier_grade(tier),
        };
    }
    let tier = match step - DIVISION_STEPS {
        0 => "Master",
        1 => "Grandmaster",
        _ => "Challenger",
    };
    Rank {
        tier,
        division: None,
        lp,
        cost,
        step,
        grade: tier_grade(tier),
    }
}

/// Cost of the division represented by a zero-based ladder step.
pub(crate) fn lp_cost_for_step(step: u32) -> i64 {
    if step < DIVISION_STEPS {
        TIER_LP_PER_DIVISION[(step / DIVISIONS_PER_TIER) as usize]
    } else {
        ELITE_LP_PER_STEP
    }
}

/// Total LP needed to reach a ladder step from Iron IV.
pub(crate) fn lp_to_reach_step(step: u32) -> i64 {
    (0..step).map(lp_cost_for_step).sum()
}

/// Highest ladder step permitted by first-attempt accuracy. LP still chooses
/// the position below this ceiling, so neither practice volume nor accuracy
/// alone is enough for a high rank.
pub(crate) fn accuracy_step_cap(accuracy: Accuracy) -> u32 {
    let percent = accuracy.percent();
    match percent {
        0..=44 => 2,   // Iron I
        45..=54 => 5,  // Bronze I
        55..=59 => 8,  // Silver I
        60..=64 => 11, // Gold I
        65..=69 => 14, // Platinum I
        70..=74 => 17, // Emerald I
        75..=79 => 20, // Diamond I
        80..=84 => 21, // Master (A-)
        85..=91 => 22, // Grandmaster (A)
        _ => u32::MAX, // Challenger (A+)
    }
}

pub(crate) fn rank_for_performance(total_lp: i64, accuracy: Accuracy) -> Rank {
    let raw = rank_for(total_lp);
    let cap = accuracy_step_cap(accuracy);
    if raw.step <= cap {
        raw
    } else {
        rank_for(lp_to_reach_step(cap))
    }
}

pub(crate) fn accuracy_required_for_step(step: u32) -> u32 {
    match step {
        0..=2 => 0,
        3..=5 => 45,
        6..=8 => 55,
        9..=11 => 60,
        12..=14 => 65,
        15..=17 => 70,
        18..=20 => 75,
        21 => 80,
        22 => 85,
        _ => 92,
    }
}

/// The rank one step up from the current LP total.
///
/// Computed from the clamped total, so sitting at Iron IV on negative LP still
/// reports Iron III as the next rung rather than Iron IV again.
pub(crate) fn next_rank(lp: i64) -> Rank {
    let rank = rank_for(lp);
    rank_for(lp.max(0) + (rank.cost - rank.lp))
}

/// Flat cost of getting a question wrong, charged on every wrong attempt.
pub(crate) const WRONG_PENALTY: i64 = 20;

/// LP over a whole history.
///
/// Each wrong attempt costs `WRONG_PENALTY`; solving it afterwards still pays
/// the halved reward, so redeeming yourself recovers some of the cost. A
/// question that was never charged a wrong attempt (rows written before
/// retries existed) has its miss charged once, on the final row.
pub(crate) fn total_lp(rows: &[ProgressRow]) -> i64 {
    let mut charged: HashSet<(String, String)> = HashSet::new();
    let mut lp = 0_i64;
    for row in rows {
        let key = (row.attempt.clone(), row.question.clone());
        match row.result.as_str() {
            "wrong" => {
                charged.insert(key);
                lp -= WRONG_PENALTY;
            }
            "correct" => lp += i64::from(points_for(row.difficulty, "correct", row.tries)),
            "incorrect" => {
                if charged.insert(key) {
                    lp -= WRONG_PENALTY;
                }
            }
            "bonus" => lp += i64::from(row.tries),
            _ => {}
        }
    }
    lp
}

/// Accuracy where each question counts once, by how its *first* attempt went.
///
/// Getting a question wrong the first time is a loss that stands, even if the
/// question is later redeemed -- the retry is for the LP, not for the record.
pub(crate) fn accuracy_of<'a>(rows: impl Iterator<Item = &'a ProgressRow>) -> Accuracy {
    let mut seen: HashSet<(String, String)> = HashSet::new();
    let mut accuracy = Accuracy::default();
    for row in rows {
        if row.result == "bonus" {
            continue;
        }
        if !seen.insert((row.attempt.clone(), row.question.clone())) {
            continue;
        }
        accuracy.total += 1;
        if row.result == "correct" {
            accuracy.correct += 1;
        }
    }
    accuracy
}

/// One-time reward when a paper is completed. Squaring accuracy makes a clean
/// finish meaningfully more valuable without letting low scores farm LP.
pub(crate) fn paper_completion_bonus(accuracy: Accuracy) -> u32 {
    if accuracy.total == 0 {
        return 0;
    }
    let correct = accuracy.correct as u64;
    let total = accuracy.total as u64;
    let numerator = 25_u64 * total * correct * correct;
    let denominator = total * total;
    ((numerator + denominator / 2) / denominator) as u32
}

pub(crate) fn quiz_stem_of(quiz_relative: &str) -> String {
    Path::new(quiz_relative)
        .file_stem()
        .and_then(|value| value.to_str())
        .unwrap_or(quiz_relative)
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn row(question: &str, difficulty: Difficulty, result: &str) -> ProgressRow {
        row_tries(question, difficulty, result, 1)
    }

    fn row_tries(question: &str, difficulty: Difficulty, result: &str, tries: u32) -> ProgressRow {
        ProgressRow {
            attempt: "a001".into(),
            question: question.into(),
            difficulty,
            result: result.into(),
            tries,
        }
    }

    #[test]
    fn each_retry_halves_the_points_and_floors() {
        // medium is 30, then 15, 7, 3, 1, then nothing left to halve
        let medium: Vec<u32> = (1..=6)
            .map(|tries| points_for(Difficulty::Medium, "correct", tries))
            .collect();
        assert_eq!(medium, vec![30, 15, 7, 3, 1, 0]);
        let easy: Vec<u32> = (1..=6)
            .map(|tries| points_for(Difficulty::Easy, "correct", tries))
            .collect();
        assert_eq!(easy, vec![20, 10, 5, 2, 1, 0]);
        let hard: Vec<u32> = (1..=6)
            .map(|tries| points_for(Difficulty::Hard, "correct", tries))
            .collect();
        assert_eq!(hard, vec![40, 20, 10, 5, 2, 1]);
        // a miss is always worth nothing, however many tries it took
        assert_eq!(points_for(Difficulty::Hard, "incorrect", 1), 0);
        assert_eq!(points_for(Difficulty::Hard, "incorrect", 4), 0);
    }

    #[test]
    fn rank_ladder_maps_to_grades() {
        // Iron III is the floor
        assert_eq!(rank_for(-500).label(), "Iron III");
        assert_eq!(rank_for(0).grade, "D");
        // Early divisions are cheap, then each tier becomes more demanding.
        assert_eq!(rank_for(59).label(), "Iron III");
        assert_eq!(rank_for(60).label(), "Iron II");
        assert_eq!(rank_for(179).label(), "Iron I");
        assert_eq!(rank_for(180).label(), "Bronze III");
        assert_eq!(lp_cost_for_step(0), 60);
        assert_eq!(lp_cost_for_step(18), 180);
        assert_eq!(lp_cost_for_step(DIVISION_STEPS), 250);
        // the grades the student actually cares about
        let master = rank_for(lp_to_reach_step(DIVISION_STEPS));
        assert_eq!((master.label().as_str(), master.grade), ("Master", "A-"));
        let gm = rank_for(lp_to_reach_step(DIVISION_STEPS + 1));
        assert_eq!((gm.label().as_str(), gm.grade), ("Grandmaster", "A"));
        let challenger = rank_for(lp_to_reach_step(DIVISION_STEPS + 2));
        assert_eq!(
            (challenger.label().as_str(), challenger.grade),
            ("Challenger", "A+")
        );
    }

    #[test]
    fn high_ranks_require_mastery_as_well_as_lp() {
        let lp = lp_to_reach_step(DIVISION_STEPS + 2);
        assert_eq!(
            rank_for_performance(
                lp,
                Accuracy {
                    correct: 84,
                    total: 100
                }
            )
            .label(),
            "Master"
        );
        assert_eq!(
            rank_for_performance(
                lp,
                Accuracy {
                    correct: 85,
                    total: 100
                }
            )
            .label(),
            "Grandmaster"
        );
        assert_eq!(
            rank_for_performance(
                lp,
                Accuracy {
                    correct: 92,
                    total: 100
                }
            )
            .label(),
            "Challenger"
        );
    }

    #[test]
    fn misses_cost_lp_so_ranks_can_fall() {
        let good = vec![
            row("q1", Difficulty::Hard, "correct"),
            row("q2", Difficulty::Hard, "correct"),
            row("q3", Difficulty::Hard, "correct"),
        ];
        assert_eq!(total_lp(&good), 120);
        let mut bad = good.clone();
        bad.push(row("q4", Difficulty::Hard, "incorrect"));
        assert_eq!(total_lp(&bad), 120 - 20);
        assert!(rank_for(total_lp(&bad)).step < rank_for(total_lp(&good)).step);
    }

    #[test]
    fn a_wrong_attempt_costs_twenty_and_counts_as_a_loss() {
        let rows = vec![
            row_tries("q1", Difficulty::Hard, "wrong", 1),
            row_tries("q1", Difficulty::Hard, "correct", 2),
        ];
        // -20 for the first miss, then the halved hard reward
        assert_eq!(total_lp(&rows), -20 + 20);
        // counted once, as a loss, even though it was redeemed
        let accuracy = accuracy_of(rows.iter());
        assert_eq!(
            (accuracy.correct, accuracy.total, accuracy.percent()),
            (0, 1, 0)
        );
    }

    #[test]
    fn every_wrong_attempt_is_charged() {
        let rows = vec![
            row_tries("q1", Difficulty::Easy, "wrong", 1),
            row_tries("q1", Difficulty::Easy, "wrong", 2),
            row_tries("q1", Difficulty::Easy, "correct", 3),
        ];
        assert_eq!(total_lp(&rows), -20 - 20 + 5);
        assert_eq!(accuracy_of(rows.iter()).total, 1);
    }

    #[test]
    fn legacy_miss_rows_are_charged_once() {
        // rows written before per-attempt penalties: a single "incorrect" row
        let rows = vec![row("q1", Difficulty::Medium, "incorrect")];
        assert_eq!(total_lp(&rows), -20);
        assert_eq!(accuracy_of(rows.iter()).percent(), 0);
    }

    #[test]
    fn first_attempt_decides_the_record() {
        let rows = [
            row("q1", Difficulty::Easy, "correct"),
            row("q1", Difficulty::Easy, "wrong"),
            row("q2", Difficulty::Easy, "wrong"),
            row("q2", Difficulty::Easy, "correct"),
        ];
        let accuracy = accuracy_of(rows.iter());
        assert_eq!((accuracy.correct, accuracy.total), (1, 2));
    }

    #[test]
    fn awards_points_by_difficulty() {
        assert_eq!(base_points(Difficulty::Easy), 20);
        assert_eq!(base_points(Difficulty::Medium), 30);
        assert_eq!(base_points(Difficulty::Hard), 40);
    }

    #[test]
    fn only_correct_answers_earn_and_misses_cost() {
        let rows = vec![
            row("q1", Difficulty::Easy, "correct"),
            row("q2", Difficulty::Hard, "incorrect"),
            row("q3", Difficulty::Medium, "correct"),
        ];
        // +20 and +30 earned, then the flat 20 for the missed hard question
        assert_eq!(total_lp(&rows), 30);
    }

    #[test]
    fn accuracy_percentages_round_sensibly() {
        assert_eq!(
            Accuracy {
                correct: 0,
                total: 0
            }
            .percent(),
            0
        );
        assert_eq!(
            Accuracy {
                correct: 1,
                total: 2
            }
            .percent(),
            50
        );
        assert_eq!(
            Accuracy {
                correct: 9,
                total: 14
            }
            .percent(),
            64
        );
        assert_eq!(
            Accuracy {
                correct: 18,
                total: 25
            }
            .percent(),
            72
        );
        let rows = [
            row("q1", Difficulty::Easy, "correct"),
            row("q2", Difficulty::Easy, "incorrect"),
            row("q3", Difficulty::Easy, "correct"),
        ];
        let accuracy = accuracy_of(rows.iter());
        assert_eq!(
            (accuracy.correct, accuracy.total, accuracy.percent()),
            (2, 3, 67)
        );
    }

    #[test]
    fn paper_bonus_scales_quadratically_with_accuracy() {
        assert_eq!(
            paper_completion_bonus(Accuracy {
                correct: 10,
                total: 10
            }),
            250
        );
        assert_eq!(
            paper_completion_bonus(Accuracy {
                correct: 9,
                total: 10
            }),
            203
        );
        assert_eq!(
            paper_completion_bonus(Accuracy {
                correct: 8,
                total: 10
            }),
            160
        );
        assert_eq!(
            paper_completion_bonus(Accuracy {
                correct: 0,
                total: 0
            }),
            0
        );

        let rows = vec![
            row("q1", Difficulty::Easy, "correct"),
            ProgressRow {
                attempt: "a001".into(),
                question: "__paper_bonus__".into(),
                difficulty: Difficulty::Easy,
                result: "bonus".into(),
                tries: 203,
            },
        ];
        assert_eq!(total_lp(&rows), 223);
        assert_eq!(accuracy_of(rows.iter()).total, 1);
    }

    #[test]
    fn one_question_bank_reaches_master_but_not_challenger() {
        // cs2100's three quiz sets, if every answer lands first try.
        let bank = (7 * 20 + 11 * 30 + 6 * 40)   // number-systems
                + (7 * 20 + 11 * 30 + 7 * 40)        // mips-tracing
                + (7 * 20 + 11 * 30 + 6 * 40); // control-unit
        assert!(bank >= lp_to_reach_step(DIVISION_STEPS));
        assert!(bank < lp_to_reach_step(DIVISION_STEPS + 2));
    }
}
