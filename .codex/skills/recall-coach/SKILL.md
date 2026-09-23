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

## Mark an attempt

Open the requested attempt and its `source-quiz`. Only mark questions whose source answer rule is `manual` and whose result remains `pending`.

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

Do not change the response, question, answer scheme, difficulty, or progress file. Do not award partial status in the MVP: if a material requirement is missing, mark it incorrect. When finished, tell the user to rerun `recall submit <attempt-id>`.
