# Recall

Recall turns your notes, slides, and past papers into a local ranked practice loop.

Your quizzes and answers are plain Markdown. An AI can create or mark them, but the CLI owns the score: it gives useful hints, prevents double-counting, tracks first-attempt accuracy, awards LP, and turns completed papers into satisfying rank-ups.

This README is mostly a real demo. The final section is the one-minute user guide.

## A real study session

This question came from a CS2100 midterm practice paper:

```md
## q11 [medium]

*q = &(A[1]);
**q++ = 100;

What is printed?

A. 1 2 3 4
B. 1 2 3 5
C. 1 100 3 4
D. 1 2 3 100
E. None of the above

<!-- response -->
A

<!-- notes -->
<!-- result: pending --> <!-- tries: 0 -->
<!-- feedback -->
```

Run one command:

```sh
recall submit cs2100/a003
```

The wrong answer stays open instead of revealing the solution:

```text
[~] q11  Decide what *q = &(A[1]) changes, then remember that
          in **q++ the ++ applies only after the dereference is used.   -20 LP

          edit it, then: recall submit cs2100/a003
```

That hint points at the misconception without spoiling the answer. After tracing the post-increment, the response becomes `C`:

```text
[+] q11  +7 LP   (try 3, after -40 LP)
```

Recall remembers that the first attempt was wrong, so repeatedly guessing cannot inflate accuracy. Retrying still earns some LP because correcting a mistake is useful.

At the end of a real 30-question CS2109 paper:

```text
this run  +448 LP   3 correct   0 to retry   0 missed
cs2109     83%  (45/54)
Master     217 LP to Grandmaster

+==================================+
|          R A N K   U P           |
|     Diamond III  ->  Master      |
|        counts as grade A-        |
+==================================+

attempt `a004` complete - 70% (21/30)
PAPER COMPLETE  +368 LP accuracy bonus
```

The completion bonus scales with both paper size and accuracy. High ranks also require first-attempt accuracy, so grinding easy questions cannot produce an A:

- Master / A− requires 80%
- Grandmaster / A requires 85%
- Challenger / A+ requires 90%

## One-minute user guide

### 1. Install

From this repository:

```sh
cargo install --path .
```

Scoring is yours to tune: edit `recall.toml` to change accuracy gates, LP per division, easy/medium/hard rewards, retry scaling, wrong-answer penalties, or the paper-completion bonus. Changes apply on the next run.

### 2. Give your files to an AI

Put the source material somewhere the AI can read, then paste this prompt and fill in the brackets:

```text
Set up a Recall quiz for me in this repository.

Subject: [cs2100]
Source files: [paths to my PDFs, notes, or past paper]
Quiz name: [midterm-practice]
Focus: [topics or "everything"]
Length: [number of questions]
Difficulty mix: [for example 20% easy, 60% medium, 20% hard]

Create the subject folders if needed. Use the existing Recall Markdown format.
Put answer rules, worked explanations, and helpful non-spoiling hints in the quiz.
Create an attempt file with the questions but no revealed answers.
Tell me the attempt reference when finished.
```

For free-response marking, ask the AI:

```text
Mark the answered questions in [subject/attempt].

Compare them with the source quiz. Edit only each question's result and feedback.
Use result: correct or result: incorrect. Preserve my responses and notes.
Do not edit progress.md or calculate LP; Recall will do that.
```

### 3. Answer and submit

Write inside each `<!-- response -->` block. Optional scratch work goes under `<!-- notes -->`.

Then keep running:

```sh
recall submit cs2100/a001
```

That single command:

- grades new automatic answers;
- detects verdicts written by an AI;
- adds hints for retryable mistakes;
- updates the ledger exactly once;
- awards LP, ranks, and the completed-paper bonus.

Blank questions are ignored, so you can answer in any order. Running the same command again never scores the same verdict twice.

## What stays local

Everything lives under `subjects/<subject>/` as readable Markdown:

```text
subjects/
  cs2100/
    resources/   your source material
    quizzes/     questions, answers, explanations, hints
    attempts/    your responses, notes, results, feedback
    progress.md  the append-only scoring ledger
```

No account, database, or hosted service is required.
