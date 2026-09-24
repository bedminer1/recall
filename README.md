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
recall mark <attempt> <qid> correct|incorrect
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
# ...or ask Codex to mark that question, then:
recall submit cs2100/a001 q4

recall                                # points, level, streak and per-difficulty accuracy
```

Every subject numbers its attempts from `a001`, so once you have more than one subject you
must pass the attempt as `subject/attempt-id`. If you forget, Recall lists the candidates
that would work.

`recall submit <attempt>` with no question ids sweeps everything still outstanding, which
is the quickest way to finish an attempt. Every question is scored exactly once, so
re-running a `submit` never double-counts.

### Scoring

Correct answers are worth 10 (easy), 20 (medium) or 30 (hard) points. Consecutive correct
answers build a streak; a wrong answer resets it while the best streak is remembered. Every
100 points is a level. Resolved questions are appended to `subjects/<subject>/progress.md`
as they happen, so nothing is lost by stopping partway through an attempt.

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
