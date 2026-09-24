# Recall

A local, Markdown-first active-recall CLI. Rust handles attempts, deterministic grading, points and streaks. Codex generates quizzes and marks free-response questions through the project skill in `.codex/skills/recall-coach/`.

## Build

```sh
cargo build
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

Ask Codex to use the Recall Coach skill and generate a quiz from the resources. Then work
through it one question at a time:

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

`recall submit <attempt>` with no question ids sweeps everything still outstanding, which
is the quickest way to finish an attempt. Every question is scored exactly once, so
re-running a `submit` never double-counts.

### Retries and hints

A wrong answer is not final. Automatic questions move to a `retry` state, which keeps the
question open and keeps the streak alive, and the feedback block gets a mechanical nudge
rather than the answer — *too low* / *too high* / *so close, check your rounding* for
numbers, *so close, check the formatting* for short answers, *you have 2 of 3 required
ideas* for multi-part ones. Blank responses are not treated as attempts at all.

The hint escalates with the number of tries, and after three misses Recall suggests
`recall giveup <attempt> <qid>` to see the worked answer and take the miss.

### Scoring and ranks

Correct answers earn LP: **15 easy / 20 medium / 30 hard**, with **every retry halving the
value, floored**. A miss costs half the question's value, so the ladder moves both ways.

| Difficulty | 1st try | 2nd try | 3rd try | 4th try | 5th try | miss |
|---|---|---|---|---|---|---|
| easy | +15 | +7 | +3 | +1 | 0 | -7 |
| medium | +20 | +10 | +5 | +2 | +1 | -10 |
| hard | +30 | +15 | +7 | +3 | +1 | -15 |

Each subject tracks its own rank, 40 LP per division, over the game's tiers with the top of
the ladder mapped onto course grades:

```
Iron - Bronze - Silver - Gold - Platinum - Emerald - Diamond   (D .. B)
Master (A-)  -  Grandmaster (A)  -  Challenger (A+)
```

Crossing a boundary prints an ASCII banner, coloured by the tier, for promotions and
demotions alike. Tier colours are ANSI, emitted only when stdout is a terminal; set
`NO_COLOR=1` to disable them.

After each question the output is deliberately short: the verdict and LP, then your
accuracy across the whole subject, your accuracy on that quiz, and the LP left to your next
rank. Scored questions are appended to `subjects/<subject>/progress.md` (with their try
count) as they happen, so nothing is lost by stopping partway through an attempt.


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
```

Answer schemes stay in the quiz file. Attempts contain only questions, response areas, results, and feedback.
