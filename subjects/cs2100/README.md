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

You get the verdict immediately, then just three numbers: your accuracy across cs2100, your accuracy on that quiz, and the LP left to your next rank.

```
  [+] q11  +20 LP

  cs2100            82%  (9/11)   all quizzes
  midterm-ay2425    82%  (9/11)   this quiz
  Bronze IV        40 LP to Bronze III   (grade D+)
```

**A wrong answer is not final.** You get a nudge, not the answer:

```
  [~] q13  too low - your value is smaller than the answer.
      edit it, then:  recall submit cs2100/a002 q13
```

Nudges are mechanical, so they never give the answer away: numbers get *too low* / *too high* / *so close, check your rounding*; short answers get *so close, check the formatting*, or on a third miss the length and first character; multi-part answers get *you have 2 of 3 required ideas*. Edit the response in the attempt file and submit again as many times as you like. A blank response isn't an attempt either, so leaving something unanswered costs nothing.

If you're truly stuck, `recall giveup cs2100/a002 q13` shows the worked explanation and takes the miss.

You can also grade several at once:

```sh
recall submit cs2100/a002 q11 q12 q13    # just those three
recall submit cs2100/a002                # whatever is still outstanding
```

## Free-response questions

Roughly two-thirds of the questions are free-response (tracing, derivations, explanations) — they can't be graded by string matching, so they don't get automatic nudges. Two ways through:

**Ask for a hint, not a verdict.** Ask Codex to check your answer using the Recall Coach skill. By default it replies with a pointer only ("you handled the exponent — what happens to the mantissa when you normalise?") and leaves the question open, so you can revise and ask again. Say you're done, or ask it to mark, when you want the verdict.

**Self-check:**

```sh
recall reveal cs2100/a001 q7            # your answer next to the model answer + reasoning
recall mark cs2100/a001 q7 correct      # or: incorrect
recall mark cs2100/a001 q7 retry        # not good enough, but don't score it — try again
```

`reveal` refuses to show anything until you've actually written a response, so it can't spoil an unanswered question. Note that seeing the model answer ends the game for that question — `retry` is for when you want another go without peeking.

Marking is honest either way, and nothing is scored twice.

## Points and progress

Every correct answer earns **LP**, and LP is what sets your rank for that subject. The base is **15 easy / 20 medium / 30 hard**, and **every retry halves it, floored**:

| Difficulty | 1st try | 2nd try | 3rd try | 4th try | 5th try | miss |
|---|---|---|---|---|---|---|
| easy | +15 | +7 | +3 | +1 | 0 | -7 |
| medium | +20 | +10 | +5 | +2 | +1 | -10 |
| hard | +30 | +15 | +7 | +3 | +1 | -15 |

A retry always beats giving up, but a miss costs half the question's value, so ranks can fall as well as rise.

## Ranks

Each subject has its own ladder, 40 LP per division. It runs the game's tiers, and the top of it is deliberately calibrated to the grades you want:

```
Iron IV .. Iron I          D
Bronze IV .. Bronze I      D+
Silver IV .. Silver I      C-
Gold IV .. Gold I          C
Platinum IV .. Platinum I  C+
Emerald IV .. Emerald I    B-
Diamond IV .. Diamond I    B
Master                     A-
Grandmaster                A
Challenger                 A+
```

So **Master is A-, Grandmaster is A, Challenger is A+**. Crossing a boundary prints a banner, up or down:

```
  +==================================+
  |          R A N K   U P           |
  |       Silver I  ->  Gold IV      |
  |       counts as grade C          |
  +==================================+
```

Reaching Challenger takes about 1200 LP, and the three cs2100 quiz sets are worth 1440 LP if every answer is first try — so it is reachable, but only by actually knowing the material.

The dashboard is deliberately just accuracy and rank, one subject at a time:

```sh
recall
```

```
Recall progress

  cs2100            82%  (9/11)
  Bronze IV        40 LP to Bronze III
  cs4243            67%  (2/3)
  Iron IV          15 LP to Iron III
```

Set `NO_COLOR=1` if you want the output plain, and colours only appear on a terminal, never in a pipe or file.

## Accuracy notes

Every machine-checkable value in the quizzes (encodings, decodings, byte layouts, branch offsets, trace results) was independently recomputed rather than trusted. The source notes themselves contain errors; the ones that matter are recorded in each notes file's "source issues" section, notably:

- **Control:** this course's simplified processor supports only `add`, `sub`, `and`, `or`, `slt`, `beq`, `lw`, `sw`. There is **no `j` instruction and no `Jump` control signal** in the design, and the ALU selector is two bits named `ALUop_1`/`ALUop_0`. The source's "Supported Instructions" table also misprints `beq` as taking `$rd`.
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

## Notation

Subscripts are always written with a leading underscore: `ALUop_1`/`ALUop_0`, `ALUcontrol_2`, `F_0`–`F_5`, `Op_5`–`Op_0`, `Ctrl_0`–`Ctrl_8`, `Operation_1`/`Operation_0`. Without the underscore these read as names rather than bit indices — `ALUop1` looks like "ALU operation 1" rather than "bit 1 of `ALUop`" — which is exactly what made parts of the control-unit material confusing.

MIPS register *names* keep their normal spelling: `$t0`, `$s0`, `$v0`, `$r1`, and datapath labels such as `RR2`/`RD2`. Only genuine bit and field indices take the underscore.

## Rule that keeps the format working

The Rust parser treats every line starting with `## ` as a new question. If you add your own questions to a file in `quizzes/`, keep exactly one `## qN [easy|medium|hard]` heading per question, always include an `answer:start`/`answer:end` and `explanation:start`/`explanation:end` pair, and make the first line inside the answer block one of `exact`, `numeric <tolerance>`, `contains`, or `manual`. Never start any other line with `## `.
