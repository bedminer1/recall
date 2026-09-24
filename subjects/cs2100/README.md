# CS2100 — active recall set (midterm-focused)

Built from **Adi Yoga's CS2100 extended notes**: <https://www.comp.nus.edu.sg/~adi-yoga/CS2100/notes/>

Scope was deliberately narrowed to the three areas you called out as weak:

| Area | Quiz | Questions | Mix | Marking |
|---|---|---|---|---|
| Number systems / data representation | `number-systems` | 24 | 7 easy / 11 medium / 6 hard | 20 manual, 4 auto |
| Tracing MIPS | `mips-tracing` | 25 | 7 easy / 11 medium / 7 hard | 14 manual, 11 auto |
| Single-cycle control (1-bit ALU, multicycle decoding, control unit) | `control-unit` | 24 | 7 easy / 11 medium / 6 hard | 12 manual, 12 auto |

Condensed source notes for each area sit in `resources/` and double as the answer-key reference:

- `resources/notes-data-representation.md`
- `resources/notes-mips.md`
- `resources/notes-control.md`

## The loop: one question at a time

Quizzes are long, so take them in short sessions and get a verdict after **every** question.

```sh
recall start number-systems --take 5
```

That draws 5 questions at random from the 24. Omit `--take` to get the whole quiz.

Open the attempt file it prints, write your answer inside the `<!-- response:start -->` / `<!-- response:end -->` block for **one** question, then:

```sh
recall submit cs2100/a001 q7
```

The attempt is written `subject/attempt-id` because every subject numbers its attempts from `a001` — with `cs4243` in the same repo, a bare `a001` is ambiguous and Recall will list the candidates instead of guessing.

You get the verdict immediately:

```
  ✅ q7   [medium]  +20 pts    🔥 streak 3

  ███████░░░░░░░░░░░░░░░░░ 3/11   ⭐ 210 pts   🔥 streak 3   🏆 best 5
  8 question(s) left in this attempt.
```

Wrong answers print the explanation on the spot and reset the streak. You don't have to finish the attempt to bank the points — each resolved question is written to `progress.md` as soon as it's known.

You can also grade several at once:

```sh
recall submit cs2100/a001 q7 q8 q9      # just those three
recall submit cs2100/a001               # whatever is still outstanding
```

## Free-response questions

Roughly two-thirds of the questions are free-response (tracing, derivations, explanations) — they can't be graded by string matching. Two ways to resolve them, both are per-question:

**Self-check (fastest, no LLM round trip):**

```sh
recall reveal cs2100/a001 q7            # shows your answer next to the model answer + reasoning
recall mark cs2100/a001 q7 correct      # or: incorrect
```

`reveal` refuses to show anything until you've actually written a response, so it can't spoil an unanswered question.

**LLM marking:** ask Codex (in this repo) to mark that one question using the Recall Coach skill. It flips that question's result in the attempt file, then:

```sh
recall submit cs2100/a001 q7
```

Marking is honest either way — `recall mark` only records a verdict you chose, and nothing is scored twice.

## Points and progress

| Difficulty | Points |
|---|---|
| easy | 10 |
| medium | 20 |
| hard | 30 |

Only correct answers score. Consecutive correct answers build a streak (a wrong answer resets it, the best streak is remembered). Every 100 points is a level.

```sh
recall
```

```
cs2100: 18/30 correct   ⭐ 420 pts   level 5
  easy   8/9
  medium 7/14
  hard   3/7
  ████████████████░░░░ 20/100 to level 6
  🔥 current streak 3   🏆 best 7
```

## Accuracy notes

Every machine-checkable value in the quizzes (encodings, decodings, byte layouts, branch offsets, trace results) was independently recomputed rather than trusted. The source notes themselves contain errors; the ones that matter are recorded in each notes file's "source issues" section, notably:

- **Control:** this course's simplified processor supports only `add`, `sub`, `and`, `or`, `slt`, `beq`, `lw`, `sw`. There is **no `j` instruction and no `Jump` control signal** in the design, and the ALU selector is two bits named `ALUop1`/`ALUop0`. The source's "Supported Instructions" table also misprints `beq` as taking `$rd`.
- **Data representation:** the source's 4-bit **Excess-8 table is wrong** (its second column duplicates Excess-7); the corrected table is in the notes.
- **MIPS:** the source prints `lw $t1,12($t0)` as `0x22D5FFCE`; the correct encoding is `0x8D09000C`. Several other source typos are listed there too.

These quizzes test the *corrected* content.

## Helper: `recall baseconv`

Number-system conversions, including the signed representations this course examines. The code lives in `src/cs2100/baseconv.rs` so all CS2100 code sits together.

```sh
recall baseconv 0b0110010            # 50 = 0x32
recall baseconv 0xFB -b 8            # that pattern as unsigned / 2C / 1C / SM
recall baseconv -5 -b 8 -b 16        # encode -5 at each width, plus sign extension
recall baseconv -i bin "1010 1111"   # spaced binary with the base forced
```

`-b N` is repeatable (1..=64). A non-negative value that fits in N bits is treated as a *pattern* and read back under all three signed schemes; a negative value is *encoded* under each scheme.

## Rule that keeps the format working

The Rust parser treats every line starting with `## ` as a new question. If you add your own questions to a file in `quizzes/`, keep exactly one `## qN [easy|medium|hard]` heading per question, always include an `answer:start`/`answer:end` and `explanation:start`/`explanation:end` pair, and make the first line inside the answer block one of `exact`, `numeric <tolerance>`, `contains`, or `manual`. Never start any other line with `## `.
