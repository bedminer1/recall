# Recall

A local, Markdown-first active-recall CLI. Rust handles attempts, deterministic grading and League-Points scoring. Codex generates quizzes and marks free-response questions through the project skill in `.codex/skills/recall-coach/`.

## Source layout

`src/main.rs` is only the entry point and the command table. Each concern lives in its own
file, so a change normally means opening one of them:

| File | Owns |
|---|---|
| `src/model.rs` | shared data types (`Question`, `AttemptQuestion`, `ProgressRow`, `Difficulty`, `Accuracy`) |
| `src/markdown.rs` | parsing quizzes and attempts, and the in-place marker surgery |
| `src/grading.rs` | grading one response, plus the hint shown when it is wrong |
| `src/scoring.rs` | League Points, the rank ladder, accuracy |
| `src/store.rs` | `progress.md` rows, project discovery, small helpers |
| `src/report.rs` | everything printed to the terminal |
| `src/commands/subject.rs` | `subject add`, `add` |
| `src/commands/start.rs` | `start` |
| `src/commands/submit.rs` | `submit` |
| `src/commands/attempt.rs` | `reveal`, `mark`, `giveup` |
| `src/cs2100/baseconv.rs` | the `baseconv` subcommand |

Unit tests sit in a `#[cfg(test)] mod tests` at the bottom of the module they cover.

## Build

```sh
cargo build
cargo test
cargo clippy --all-targets -- -D warnings
```

The executable is `target/debug/recall`. Optionally install it with:

```sh
cargo install --path .
```

## Commands

```sh
recall subject add <subject>               create a subject
recall add <subject> <file>                move a resource into the subject
recall add <subject> <file> --copy         copy instead of move
recall start [quiz] [--take N]             new attempt, optionally only N questions
recall submit <attempt> [qid ...]          grade one question, or everything
recall reveal <attempt> <qid>              show the model answer (self-check)
recall mark <attempt> <qid> verdict        correct | incorrect | retry
recall giveup <attempt> <qid>              show the answer and take the miss
recall baseconv <value> [-b N] [-i base]   CS2100 number-system conversions
recall <subject> rank                      graph that subject's rank history
recall                                     dashboard
```

## Workflow

```sh
recall subject add cs2100
recall add cs2100 /path/to/slides.pdf          # moves the file into resources/
recall add cs2100 /path/to/slides.pdf --copy   # keeps the original in place
```

`recall add` moves the source file into `subjects/<subject>/resources/` rather than
copying it, so the original path no longer exists afterwards. Pass `--copy` when you
want to keep the file where it is.

Use any LLM to generate a quiz from the resources or mark free responses in an attempt.
The LLM edits only the attempt's result and feedback; `recall submit` is the tool that owns
the ledger, LP, rank changes, and paper bonuses. Then work through it one question at a time:

```sh
recall start mips-basics --take 5     # a short session: 5 questions drawn at random
# Open the path printed by the command and fill ONE response block.
recall submit cs2100/a001 q3          # verdict, points and streak come back immediately

# Free-response questions resolve either way:
recall reveal cs2100/a001 q4          # show the model answer, then decide yourself
recall mark cs2100/a001 q4 correct
recall mark cs2100/a001 q4 retry      # park it for another go without scoring a miss
# ...or ask Codex to check that question, then:
recall submit cs2100/a001 q4

# Stuck on an automatic question after a few tries:
recall giveup cs2100/a001 q3          # shows the worked answer and takes the miss

recall                                # points, level, streak and per-difficulty accuracy
```

Every subject numbers its attempts from `a001`, so once you have more than one subject you
must pass the attempt as `subject/attempt-id`. If you forget, Recall lists the candidates
that would work.

`recall submit <attempt>` with no question ids grades every unanswered automatic question
and detects any `correct` or `incorrect` verdict an LLM has written but which has not yet
reached the ledger. Blank questions simply wait, so one unanswered question never blocks
the rest. Name ids to grade specific questions instead
(`recall submit <attempt> q11 q12`). Every question is scored exactly once, so re-running a
`submit` never double-counts, and re-grading an unchanged answer never re-charges the
penalty.

### Retries and hints

A wrong answer is not final. Automatic questions move to a `retry` state, so the question
stays open, while the feedback block gets a hint rather than the answer. A quiz can carry an
authored hint per question in a `<!-- hint:start -->` / `<!-- hint:end -->` block; when it
has none, Recall falls back to a mechanical nudge derived from the grading rule — *too low* /
*too high* / *so close, check your rounding* for numbers, *so close, check the formatting*
for short answers, *you have 2 of 3 required ideas* for multi-part ones. Blank responses are
not treated as attempts at all.

After three tries Recall suggests `recall giveup <attempt> <qid>` to see the worked answer
and take the miss.

### Scoring and ranks

Correct answers earn LP: **20 easy / 30 medium / 40 hard**, with **every retry halving the
value, floored**. A wrong answer costs a flat **-20 LP on every wrong attempt**, so the
ladder moves both ways.

| Difficulty | 1st try | 2nd try | 3rd try | 4th try | 5th try |
|---|---|---|---|---|---|
| easy | +20 | +10 | +5 | +2 | +1 |
| medium | +30 | +15 | +7 | +3 | +1 |
| hard | +40 | +20 | +10 | +5 | +2 |

Because the penalty is per attempt, a wrong answer followed by a correct one can still net
negative — retrying beats giving up, but a first-try answer is where the value is.

**Accuracy counts each question once, by how its first attempt went.** A question you got
wrong first time stays a loss on the record even if you redeem it; the retry pays LP, not
accuracy. `subjects/<subject>/progress.md` records a `wrong` row per failed attempt plus one
final `correct`/`incorrect` row, so the ledger shows both. Rows written before retries
existed are still read correctly: a lone `incorrect` row is charged one penalty.

Each subject tracks its own rank on a progressive ladder. Normal tiers have three divisions
(III, II, I), keeping the early climb quick; each tier then asks for more LP so progress
becomes meaningfully harder near Diamond:

| Tier | LP per division |
|---|---:|
| Iron | 60 |
| Bronze | 65 |
| Silver | 75 |
| Gold | 85 |
| Platinum | 110 |
| Emerald | 125 |
| Diamond | 180 |
| Master and above | 250 per rank |

High ranks also have a first-attempt accuracy gate, so they cannot be reached by grinding
enough questions. LP and accuracy must both qualify:

| Rank ceiling | Minimum accuracy | NUS interpretation |
|---|---:|---|
| Bronze | 45% | D+ |
| Silver | 55% | C- |
| Gold | 60% | C |
| Platinum | 65% | C+ |
| Emerald | 70% | B- / median anchor |
| Diamond | 75% | B |
| Master | 80% | A- |
| Grandmaster | 85% | A / upper-quartile anchor |
| Challenger | 92% | A+ / exceptional mastery |

Finishing a paper awards a one-time accuracy bonus: `25 × questions × accuracy²`, rounded
to the nearest LP. A 10-question paper pays +250 LP at 100%, +203 LP at 90%, and +160 LP
at 80%. The squared accuracy makes a strong finish feel substantial without rewarding
low-accuracy completion nearly as much. Existing completed papers are not backfilled.

The tiers map onto course grades at the top of the ladder:

```
Iron - Bronze - Silver - Gold - Platinum - Emerald - Diamond   (D .. B)
Master (A-)  -  Grandmaster (A)  -  Challenger (A+)
```

Crossing a boundary prints an ASCII banner, coloured by the tier, for promotions and
demotions alike. Tier colours are ANSI, emitted only when stdout is a terminal; set
`NO_COLOR=1` to disable them.

After each question the output is deliberately short: the verdict and LP, then your
accuracy across the whole subject, your accuracy on that quiz, and the LP left to your next
rank.


## Quiz format

```md
# MIPS basics

## q1 [easy]

Which register is hardwired to zero?

<!-- answer:start -->
exact
$zero
$0
<!-- answer:end -->
<!-- explanation:start -->
Register 0, conventionally named `$zero`, always reads as zero.
<!-- explanation:end -->

## q2 [hard]

Explain how an `lw` instruction moves through the single-cycle datapath.

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
A correct answer follows instruction fetch, register read, immediate extension, ALU address calculation, memory read, and write-back to `rt`, with the relevant control choices.
<!-- explanation:end -->
<!-- hint:start -->
Remember the immediate is sign-extended before it reaches the ALU, and the address is base plus that value.
<!-- hint:end -->
```

A `hint` block is optional but worth writing: it is what the student sees the first time they
get the question wrong, so it should name the concept, the step or the trap without giving
the answer away. Questions without one fall back to the mechanical hint.

Answer schemes stay in the quiz file. Attempts contain only questions, response areas,
results, and feedback.

### Notes

Attempts also carry scratch space that grading never reads: a `Working notes` section in the
header, and a `notes` block directly above each `response`, both wrapped in
`<!-- notes:start -->` / `<!-- notes:end -->`. Students write their reasoning and shortcuts
there and then answer below it; only the `response` block is ever graded, so notes are safe
even when they contain the answer. Keep those blocks intact when editing an attempt by hand.

The `feedback` block is the tool's channel for hints and explanations, but writes to it are
appends rather than replacements, so anything a student typed there by hand survives a
submit.
