---
name: recall-coach
description: Generate Recall Markdown quizzes from subject resources and mark pending free-response attempts. Use inside a Recall study repository when the user asks for a quiz, imports questions, or asks to mark an attempt.
---

# Recall Coach

Work only inside the requested subject under `subjects/<subject>/`.

## Generate a quiz

Read the relevant files in `resources/` and create one Markdown file in `quizzes/`. Do not modify attempts or progress.

Use stable one-word question IDs and exactly this structure:

```markdown
# Descriptive quiz title

## q1 [easy]

Question text.

<!-- answer:start -->
exact
accepted answer
another accepted answer
<!-- answer:end -->
<!-- explanation:start -->
Concise explanation of the correct reasoning.
<!-- explanation:end -->
```

Allowed difficulties are `easy`, `medium`, and `hard`.

The first line inside the answer block is one grading rule:

- `exact`: each following non-empty line is an accepted complete answer.
- `numeric TOLERANCE`: the next line is the expected number.
- `contains`: every following non-empty line must occur in the response after case and whitespace normalization. Use only for genuinely unambiguous short responses.
- `manual`: Codex must mark the answer using the explanation as the answer scheme.

Prefer automatic rules when correctness is unambiguous. Use `manual` for reasoning, derivations, explanations, proofs, or answers where wording can vary. Never weaken a question just to make automatic grading possible.

Create questions that test recall and application rather than copied wording. Easy questions test one idea directly; medium questions combine steps or require application; hard questions require transfer, comparison, debugging, or multi-step reasoning. Base answers on the supplied resources. Make explanations useful after a mistake without referring to a student's future answer.

After writing the quiz, tell the user its short name so they can run `recall start <name>`.

## Give a hint (default when a free-response answer is wrong)

The student retries rather than being handed the answer, so **do not lead with the verdict or the model answer**. When they ask you to check a free-response response, first judge it silently, then reply with a hint only:

- Say what direction is right, if anything ("your datapath ordering is correct so far").
- Name the single missing or mistaken idea as a question or a pointer, not as the answer. For example: "you handled the exponent — what happens to the mantissa when you normalise?"
- Never state the corrected answer, and never paste the explanation's conclusion.
- Keep it to two or three sentences.

Do not edit the attempt file when giving a hint. Leave the result as `pending`.

Only when the student says they are done, asks for the answer, or asks you to mark it, do you reveal or mark. If they want to try again, they can either edit their response and ask you to re-check, or run:

```sh
recall mark <attempt-id> <question-id> retry
```

which parks the question without scoring it as a miss.

## Mark an attempt

Open the requested attempt and its `source-quiz`. Only mark questions whose source answer rule is `manual` and whose result remains `pending`.

Scope the work to what was asked: if the user names one question, mark only that one. Marking a single question is the normal case, because the user works one question at a time and expects feedback immediately.

Compare the response with the frozen answer and explanation. Replace:

```markdown
<!-- result: pending -->
```

with exactly one of:

```markdown
<!-- result: correct -->
<!-- result: incorrect -->
```

Inside that question's `feedback` block, write a short explanation containing:

1. what was correct in the response, if anything;
2. the specific missing or mistaken idea;
3. the corrected reasoning.

A blank response is not an attempt: leave it `pending` and ask them to write something first.

Do not change the response, question, answer scheme, difficulty, or progress file. Do not award partial status in the MVP: if a material requirement is missing, mark it incorrect.

When finished, tell the user the verdict and the exact command to record it:

```sh
recall submit <attempt-id> <question-id>
```

If you marked several questions, list each verdict with its id and tell them to run `recall submit <attempt-id>` with no question id to record them all at once.

`recall mark` and `recall giveup` are the user's own paths. Do not run them on their behalf — if they ask you to mark a question, edit the attempt file as described above.


