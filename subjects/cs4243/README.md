# CS4243 — active recall set (midterm-focused)

Built from the Lecture 1–6 slides, the in-lecture and review question decks, and the two released midterm papers (23/24 and 25/26). All 21 source PDFs were **moved** out of `~/Downloads` into `resources/` with `recall add` (move semantics).

## Quizzes

| Lecture | Quiz | Questions | Mix |
|---|---|---|---|
| L1 — Introduction, images & colour | `l1-intro-and-colour` | 12 | 5 easy / 5 medium / 2 hard |
| L2 — Point processing, filtering & matching | `l2-filtering-and-matching` | 12 | 2 easy / 5 medium / 5 hard |
| L3 — Gradients, edges & Hough | `l3-gradients-edges-hough` | 12 | 3 easy / 5 medium / 4 hard |
| L4 — Filter banks, multiscale & texture | `l4-filter-banks-and-texture` | 9 | 2 easy / 4 medium / 3 hard |
| L5 — CNNs & object detection | `l5-cnns-and-detection` | 12 | 3 easy / 5 medium / 4 hard |
| L6 — Image segmentation | `l6-segmentation` | 12 | 1 easy / 5 medium / 6 hard |
| Exam-format synthesis | `midterm-style-synthesis` | 12 | 1 easy / 4 medium / 7 hard |

**81 questions total.** Most free-response questions are `manual` (marked by Codex via the Recall Coach skill); the unambiguous ones use `exact` or `numeric` and are graded instantly. `midterm-style-synthesis` deliberately mirrors the exam structure: fill-in-the-blank computation, multiple choice, and system design.

## How to run it

```sh
recall start l3-gradients-edges-hough    # or any quiz name above
recall submit a001                       # grade automatic questions
recall                                   # dashboard: accuracy by difficulty
```

Open the attempt file printed by `recall start` and write each answer inside its `<!-- response:start -->` / `<!-- response:end -->` block. If free-response questions remain, the first `submit` grades the automatic ones and lists the pending IDs; ask Codex to mark the attempt, then rerun the same `submit`.

## Resources

Moved from `~/Downloads` (21 PDFs): lecture decks `L01`–`L06`, the question decks (`L1`–`L4` review, `L3`/`L4`/`L5`/`L6` in-lecture), and `Midterm2324{QsOnly,FullRelease}.pdf` + `Midterm2526{QsOnly,FullRelease}.pdf`.

## Source caveats worth knowing

- **Mean-shift clustering is explicitly marked "not part of syllabus this year"** in the L6 changelog, so it is excluded from the quizzes even though it appears in the older 25/26 midterm paper. Panoptic segmentation is likewise not named in the L6 deck; only semantic vs. instance is covered.
- The L6 deck provides **no connectivity enforcement** in SLIC (isolated pixels are possible; connected-components cleanup is the standard remedy), and it contains no texton/K-means-count arithmetic — that arithmetic appears only in the midterm papers, so those questions are labelled exam-style.
- **Sobel sign convention varies between decks.** `L03_Gradients` defines `Gx = [1 0 -1; 2 0 -2; 1 0 -1]`, `Gy = [1 2 1; 0 0 0; -1 -2 -1]`, while the 25/26 midterm paper prints the negated versions. Only the sign of the resulting vector differs, so gradient *orientation* answers depend on the convention used; the synthesis quiz states its kernels explicitly.
- The **23/24 paper shows some Laplacian/media-filter rationales as images** ("(d) is Gaussian smoothing", a blurry photo), so those specific visual reasoning questions were generalised into text-based questions rather than copied.
- Two internal inconsistencies in the L2 decks were noted during extraction: the "Same" padding value differs between the filtering and convolution-extras decks (`p=1` vs `p=2` for a 3×3 kernel), and some NCC slides say "minima" where the FAQ says the identical normalised pattern gives the largest score. The quizzes follow the printed output-size formula `N_out = (N_in + 2p − K)/s + 1`.
- Arithmetic in the computational questions was independently recomputed (storage, equalisation mapping, Sobel responses on the 5×5 edge image, SLIC grid spacing, MLP/convolution parameter counts, Dice↔IoU) rather than trusted from the slides.

## Rule that keeps the quizzes working

Because `recall start` generates a **separate attempt per quiz** and questions are answered in isolation, **every question stem must be self-contained.** Two questions originally violated this and have been fixed:

- The storage-trade question asked for "the same as before" without ever stating a baseline bit depth, so it was unanswerable as written. It now states the full setup (1000 × 1000, 8 bits per channel) in the stem. Its answer was also **wrong at first**: halving both dimensions divides the pixel count by 4, so holding storage constant requires the bit depth to *rise* 4×, to **32 bits per channel** — not fall to 2. The "2 bits" figure applies the factor of 4 in the wrong direction.
- Two questions began "Continue the scenario above" / "In the L2 review question on…", referring to material outside their own stem. Both now restate the scenario inline.

If you add questions, keep each one independent: state all given numbers, avoid "the previous question", "as before", or references to a deck's question by name, and make sure the answer follows from the stem alone.
