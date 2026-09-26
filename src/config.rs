//! User-tunable scoring settings loaded from `recall.toml` at the project root.

use crate::model::AppResult;
use std::{fs, path::Path, sync::OnceLock};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ScoringConfig {
    pub(crate) rewards: [u32; 3],
    pub(crate) retry_divisor: u32,
    pub(crate) wrong_answer: i64,
    pub(crate) tier_lp: [i64; 7],
    pub(crate) elite_lp: i64,
    pub(crate) accuracy_gates: [u32; 9],
    pub(crate) bonus_per_question: u32,
    pub(crate) bonus_accuracy_power: u32,
}

impl Default for ScoringConfig {
    fn default() -> Self {
        Self {
            rewards: [20, 30, 40],
            retry_divisor: 2,
            wrong_answer: 20,
            tier_lp: [60, 65, 75, 85, 110, 125, 180],
            elite_lp: 250,
            // Bronze through Challenger; Iron has no gate.
            accuracy_gates: [45, 55, 60, 65, 70, 75, 80, 85, 90],
            bonus_per_question: 25,
            bonus_accuracy_power: 2,
        }
    }
}

static CONFIG: OnceLock<ScoringConfig> = OnceLock::new();

pub(crate) fn get() -> &'static ScoringConfig {
    CONFIG.get_or_init(ScoringConfig::default)
}

pub(crate) fn load(root: &Path) -> AppResult<()> {
    let path = root.join("recall.toml");
    let config = if path.exists() {
        parse(&fs::read_to_string(path).map_err(|error| error.to_string())?)?
    } else {
        ScoringConfig::default()
    };
    CONFIG
        .set(config)
        .map_err(|_| "scoring config loaded more than once".into())
}

fn parse(text: &str) -> AppResult<ScoringConfig> {
    let mut config = ScoringConfig::default();
    let mut section = "";
    for (index, raw) in text.lines().enumerate() {
        let line = raw.split('#').next().unwrap_or("").trim();
        if line.is_empty() {
            continue;
        }
        if line.starts_with('[') && line.ends_with(']') {
            section = &line[1..line.len() - 1];
            continue;
        }
        let (key, value) = line
            .split_once('=')
            .ok_or_else(|| format!("recall.toml:{}: expected `key = value`", index + 1))?;
        let name = format!("{}.{}", section, key.trim());
        let value = value.trim();
        match name.as_str() {
            "rewards.easy" => config.rewards[0] = number(value, &name)?,
            "rewards.medium" => config.rewards[1] = number(value, &name)?,
            "rewards.hard" => config.rewards[2] = number(value, &name)?,
            "rewards.retry_divisor" => config.retry_divisor = number(value, &name)?,
            "penalties.wrong_answer" => config.wrong_answer = number(value, &name)?,
            "ranks.lp_per_division" => config.tier_lp = array(value, &name)?,
            "ranks.elite_lp_per_rank" => config.elite_lp = number(value, &name)?,
            "accuracy_gates.bronze" => config.accuracy_gates[0] = number(value, &name)?,
            "accuracy_gates.silver" => config.accuracy_gates[1] = number(value, &name)?,
            "accuracy_gates.gold" => config.accuracy_gates[2] = number(value, &name)?,
            "accuracy_gates.platinum" => config.accuracy_gates[3] = number(value, &name)?,
            "accuracy_gates.emerald" => config.accuracy_gates[4] = number(value, &name)?,
            "accuracy_gates.diamond" => config.accuracy_gates[5] = number(value, &name)?,
            "accuracy_gates.master" => config.accuracy_gates[6] = number(value, &name)?,
            "accuracy_gates.grandmaster" => config.accuracy_gates[7] = number(value, &name)?,
            "accuracy_gates.challenger" => config.accuracy_gates[8] = number(value, &name)?,
            "paper_bonus.lp_per_question" => config.bonus_per_question = number(value, &name)?,
            "paper_bonus.accuracy_power" => config.bonus_accuracy_power = number(value, &name)?,
            _ => {
                return Err(format!(
                    "recall.toml:{}: unknown setting `{name}`",
                    index + 1
                ));
            }
        }
    }
    validate(&config)?;
    Ok(config)
}

fn number<T: std::str::FromStr>(value: &str, name: &str) -> AppResult<T> {
    value
        .parse()
        .map_err(|_| format!("invalid number for `{name}`: `{value}`"))
}

fn array<const N: usize>(value: &str, name: &str) -> AppResult<[i64; N]> {
    let values: Vec<i64> = value
        .trim_matches(|ch| ch == '[' || ch == ']')
        .split(',')
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .map(|part| number(part, name))
        .collect::<AppResult<_>>()?;
    values
        .try_into()
        .map_err(|values: Vec<i64>| format!("`{name}` needs {N} values, found {}", values.len()))
}

fn validate(config: &ScoringConfig) -> AppResult<()> {
    if config.retry_divisor < 2 {
        return Err("`rewards.retry_divisor` must be at least 2".into());
    }
    if config.wrong_answer < 0 || config.elite_lp <= 0 || config.tier_lp.iter().any(|v| *v <= 0) {
        return Err("LP costs and penalties must be positive".into());
    }
    if config.bonus_accuracy_power == 0 {
        return Err("`paper_bonus.accuracy_power` must be at least 1".into());
    }
    if config.accuracy_gates.iter().any(|v| *v > 100)
        || config
            .accuracy_gates
            .windows(2)
            .any(|pair| pair[0] > pair[1])
    {
        return Err("accuracy gates must be ascending percentages from 0 to 100".into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parses_overrides_and_keeps_defaults() {
        let config = parse("[rewards]\neasy = 24\n[accuracy_gates]\nchallenger = 90\n").unwrap();
        assert_eq!(config.rewards, [24, 30, 40]);
        assert_eq!(config.accuracy_gates[8], 90);
    }
}
