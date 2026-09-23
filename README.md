# Recall

A local, Markdown-first active-recall CLI. Rust handles attempts, deterministic grading, and simple Easy/Medium/Hard progress. Codex generates quizzes and marks free-response questions through the project skill in `.codex/skills/recall-coach/`.

## Build

```sh
cargo build
```

The executable is `target/debug/recall`. Optionally install it with:

```sh
cargo install --path .
```

## Workflow

```sh
recall subject add cs2100
recall add cs2100 /path/to/slides.pdf
```

Ask Codex to use the Recall Coach skill and generate a quiz from the resources. Then:

```sh
recall start mips-basics
# Open the path printed by the command and fill the response blocks.
recall submit a001
recall
```

If an attempt contains free-response questions, the first submission grades automatic questions and names the pending ones. Ask Codex to mark the attempt, then rerun the same `submit` command.

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
