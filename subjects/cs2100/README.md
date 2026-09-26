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

## Somewhere to think

Every attempt gives you two places to write that grading never reads:

- **`Working notes`** at the top of the attempt, for shortcuts and traps worth remembering across the whole paper — *"leading hex digit 8-F means negative"*, *"the branch offset counts from PC+4, in words"*.
- **A `notes` area directly above each answer**, for your working: the reasoning you'd otherwise lose, the options you ruled out, where you got stuck. Work up there, then write the answer underneath.

Both sit inside `<!-- notes:start -->` / `<!-- notes:end -->` markers. Nothing reads them, nothing overwrites them, and no marker counts as your answer — only the `response` block does. Write as much as you like, and keep it after the question is marked so the next visit to that question starts from your own notes.

The `feedback` block is the tool's side (hints and explanations land there), but if you write in it anyway nothing is lost — the tool appends rather than replaces.

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

Every question in the cs2100 quizzes carries its own written hint, which is what you see the first time you get it wrong. It names the concept, the step or the trap without giving the answer away — for example *"the top 4 bits come from PC+4, not from the target, so work out PC+4 first"*. Edit your response in the attempt file and submit again as many times as you like. A blank response isn't an attempt either, so leaving something untouched costs nothing.

If a quiz has no authored hint, Recall falls back to a mechanical one derived from the grading rule: numbers get *too low* / *too high* / *so close, check your rounding*, short answers get *so close, check the formatting*, and multi-part answers get *you have 2 of 3 required ideas*.

If you're truly stuck, `recall giveup cs2100/a002 q13` shows the worked explanation and takes the miss.

## One command, no argument to update

```sh
recall submit cs2100/a002
```

With no question id this grades **every question you have answered that has no verdict yet**. So the rhythm is: write answers for as many questions as you like, then hit up-arrow and enter — the same command every time, nothing to retype. It gives a verdict and a hint per question, then one aggregate line:

```
  [+] q2  +15 LP
  [~] q3  Two tens.   -20 LP
      edit it, then:  recall submit cs2100/a002 q3
  [+] q4  +10 LP   (try 2, after -20 LP)

  this run  +5 LP   2 correct   1 to retry   0 missed
  cs2100            50%  (2/4)   all quizzes
  midterm-ay2425    50%  (2/4)   this quiz
  Iron IV          35 LP to Iron III   (grade D)
```

A question left blank simply waits — it never blocks the ones after it, so you can skip a question and come back to it. A question still open from a previous round is picked up again automatically, and re-grading an identical answer never charges the penalty twice. Settled questions are skipped, so you never re-grade what you already finished.

If you want to grade specific questions, name them: `recall submit cs2100/a002 q11 q12 q13`.

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

Every correct answer earns **LP**, and LP is what sets your rank for that subject. The base is **20 easy / 30 medium / 40 hard**, and **every retry halves it, floored**:

| Difficulty | 1st try | 2nd try | 3rd try | 4th try | 5th try |
|---|---|---|---|---|---|
| easy | +20 | +10 | +5 | +2 | +1 |
| medium | +30 | +15 | +7 | +3 | +1 |
| hard | +40 | +20 | +10 | +5 | +2 |

Getting a question wrong costs a flat **-20 LP, charged on every wrong attempt**, on top of the halved reward if you later redeem it. So a medium question answered wrong then right nets `-20 + 15 = -5`: retrying still beats giving up, but a first-try answer is where the value is.

A blank response is not an attempt, so leaving a question untouched costs nothing.

**Accuracy counts the first attempt only.** Once you get a question wrong the first time it is a loss on the record, even if you then redeem it — the retry is for the LP, not for the accuracy. `progress.md` keeps both: a `wrong` row per failed attempt, and one final `correct`/`incorrect` row per question.

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
