# CS4243 — Midterm-Style Applied Synthesis

Exam-format practice modelled on the Midterm 23/24 and 25/26 papers: fill-in-the-blank computation, multiple choice, and system design. Numbers are fresh so the answers must actually be worked out.

## q1 [medium]

A camera captures uncompressed RGB images at 1920 × 1080 pixels with 8 bits per channel. Using 1 MB = 1,000,000 bytes and 1 byte = 8 bits:

(a) How many MB are required to store one raw frame?
(b) If you instead halve both the width and the height, what bit-depth per channel would keep the storage unchanged?

Give (a) in MB and (b) in bits, as two numbers.

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
(a) Raw storage is b = M · N · k bits. Per channel: 1920 × 1080 × 8 = 16,588,800 bits = 2,073,600 bytes ≈ 2.0736 MB. With three RGB channels: 3 × 2.0736 = **6.2208 MB** (equivalently 49,766,400 bits).

(b) Halving width and height multiplies the pixel count by 0.25. To keep b = M·N·k constant, k must compensate by the reciprocal: 0.25M · 0.25N · k′ = M · N · k ⟹ k′ = 0.25k = 0.25 × 8 = **2 bits per channel**. The spatial reduction by 4× must be paid for by a 4× intensity reduction.

(Note the distinction from lossy compression: this calculation is about raw/bitmap storage; JPEG achieves smaller files by discarding DCT information, not by this kind of resampling.)
<!-- explanation:end -->

## q2 [hard]

An image of a dark scene has 90% of its pixels at intensity 10, and the remaining pixels split evenly (2.5% each) across intensities 8, 60, 200 and 230.

(a) How many unique intensity values remain after histogram equalization?
(b) What does intensity 8 map to? What does intensity 10 map to? Give both output values.

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
Histogram equalization maps each intensity to its **cumulative proportion** (the CDF) scaled by the maximum intensity, so out(v) = round(255 · P(intensity ≤ v)). It cannot create intensity levels that were not present in the input, so the number of unique output values is bounded by the number of unique *input* values.

Cumulative proportions, processing levels in ascending order:

| intensity | proportion | cumulative CDF | out = 255 × CDF |
|---|---|---|---|
| 8 | 0.025 | 0.025 | 6 |
| 10 | 0.900 | 0.925 | 236 |
| 60 | 0.025 | 0.950 | 242 |
| 200 | 0.025 | 0.975 | 249 |
| 230 | 0.025 | 1.000 | 255 |

(a) The input has 5 unique levels {8, 10, 60, 200, 230}, so the output has **5 unique values** {6, 236, 242, 249, 255} — equalization redistributes existing levels, it does not invent new ones.

(b) Intensity **8 maps to 6** (the first cumulative proportion, 0.025 × 255 = 6.4 ≈ 6) and intensity **10 maps to 236** (0.925 × 255 = 235.9 ≈ 236).

The instructive point: the *dominant* dark level is stretched far away from its neighbours, because the CDF jumps steeply across the 90% mass at intensity 10 — which is precisely why equalization dramatically expands the contrast of levels that occur frequently.
<!-- explanation:end -->

## q3 [hard]

Consider the 5×5 greyscale image of a diagonal edge (1 = white, 0 = black):

```
1 1 1 1 0
1 1 1 0 0
1 1 0 0 0
1 0 0 1 0
0 0 0 0 0
```

Let (r, c) denote (row, column) with indexing starting at 1. Use the Sobel kernels

```
Sx = [-1 0 1; -2 0 2; -1 0 1]     Sy = [1 2 1; 0 0 0; -1 -2 -1]
```

Computing gradients without border padding:

(a) How many **unique** gradient vectors (Gx, Gy) are present?
(b) What is the gradient **magnitude** at element (4, 4)?
(c) What is the gradient **orientation** at element (3, 3), in degrees?

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
Without padding the valid output is 3 × 3, i.e. positions (2,2) to (4,4). Convolving gives

```
Gx = [-1 -3 -3;  -3 -2 -1;  -3  1  0]
Gy = [ 1  3  3;   3  2 -1;   3  1  0]
```

(a) Collecting the distinct pairs (Gx, Gy): (−3, 3), (−2, 2), (−1, 1), (−1, −1), (1, 1), (0, 0). That is **6 unique gradient vectors**.

(b) At element (4, 4) the corresponding response is (Gx, Gy) = (0, 0) — the 3×3 neighbourhood there contains two white pixels on the anti-diagonal and two more that cancel the contributions, so everything cancels. Magnitude = √(0² + 0²) = **0**. (Intuitively, the isolated 1 at (4,4) is neutralised by the 1 at (3,3) under these kernels.)

(c) At element (3, 3) the response is (Gx, Gy) = (−2, 2), so orientation = atan2(Gy, Gx) = atan2(2, −2) = **135°**. This is exactly the 45° line running up-and-to-the-left, and it passes the intuitive check: the gradient points from dark toward bright, and from (3,3) the brightness increases going up and to the left (cells (2,2), (1,1) are 1s), with x decreasing and y increasing. Computing it directly avoids any sign confusion: Gx < 0 because intensity falls as x increases, Gy > 0 because intensity rises as y increases, which lands the direction of steepest increase in the upper-left quadrant at 135°.

Useful shortcut: rather than convolving all 9 positions, count the number of distinct 3×3 neighbourhoods in the valid region and exploit kernel symmetries to collapse identical responses.
<!-- explanation:end -->

## q4 [hard]

You are building a texture classification system using a texton dictionary and brute-force nearest-neighbour classification with sum-of-squared differences. Your training set has 25 images, each 64 × 64 pixels. A filter bank of 8 filters, each 9 × 9, is applied to every image **with 4 pixels of zero padding on each side**. Pixel-wise filter responses are clustered with K-means into 40 textons, and each image is represented by a single 40-bin histogram.

Fill in the blanks. If a quantity cannot be determined, write -1.

(a) The dimensionality of the feature vectors fed into K-means is ____.
(b) The total number of feature vectors fed into K-means is ____.
(c) The number of squared-difference terms computed to classify one test image against the entire training set is ____.

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
(a) **8.** At each pixel location the responses of all 8 filters form one feature vector, so the dimensionality equals the number of filters. The 9 × 9 spatial size of each filter is irrelevant to the feature dimensionality — it affects *where* responses exist, not how many numbers describe each location.

(b) **102,400.** Compute the response grid size per filter from the output-size formula: out = (n + 2p − k)/s + 1 = (64 + 2·4 − 9)/1 + 1 = 63 + 1 = 64. So with 4 pixels of padding on each side the response map is the full 64 × 64, giving 64 × 64 = 4096 response positions per image, one feature vector at each. Across all 25 training images: 4096 × 25 = **102,400 feature vectors** fed into K-means.

Two classic errors to avoid here: (i) forgetting the padding term, which would give 64 − 9 + 1 = 56 per side, 3136 positions and 78,400 vectors total; and (ii) reporting the per-image count (4096) instead of the total asked for across the training set.

(c) **200.** Classifying a test image means comparing its 40-bin histogram against the 40-bin histogram of every training image. Each comparison costs one squared-difference term per bin = 40 terms, computed against 25 training images: 40 × 25 = **200** terms.

(Method summary: feature-vector *count* uses the spatial response grid; classification *cost* uses the histogram dimensionality and the number of training images. Confusing the two — e.g. using 8 filters where 40 bins belong — is the usual mistake.)
<!-- explanation:end -->

## q5 [easy]

You are detecting a specific orange basketball in a video feed where lighting varies from bright sunlight to shadow, changing pixel intensities. Which colour representation is most robust for defining "orange" independently of brightness?

A. Raw RGB values
B. Greyscale intensity with weights {0.8, 0.2, 0}
C. Greyscale intensity with weights {0.299, 0.587, 0.114}
D. Normalized RGB
E. The V channel of HSV

<!-- answer:start -->
exact
D
<!-- answer:end -->
<!-- explanation:start -->
Normalized RGB divides each channel by the sum (r + g + b = 1), which separates colour from intensity: an object's normalized colour stays consistent as illumination strength changes. Raw RGB mixes colour and brightness; greyscale discards colour entirely; and V in HSV is the intensity axis (max of R, G, B), so it varies with illumination rather than describing hue.
<!-- explanation:end -->

## q6 [medium]

Recall the image formation model f(x, y) = i(x, y) · r(x, y). A flat, uniform sheet of white paper lies on a desk lit by a lamp positioned to the left. As you scan from the left edge of the paper to the right edge, how do the components change?

A. r(x, y) decreases; i(x, y) remains constant
B. r(x, y) remains constant; i(x, y) decreases
C. r(x, y) increases; i(x, y) decreases
D. r(x, y) decreases; i(x, y) increases
E. Both remain constant

<!-- answer:start -->
exact
B
<!-- answer:end -->
<!-- explanation:start -->
Reflectance r ∈ [0, 1] is a material property; uniform white paper has high, constant r. Illumination i is the incident light, which falls off with distance from the lamp, so it decreases left to right. The observed darkening is therefore an illumination effect, not a surface change — the classic reason shading must be discounted for material/colour identification.
<!-- explanation:end -->

## q7 [medium]

Which of the following operations can produce **negative** output pixel values when applied to an image whose input intensities lie in [100, 200]? Select all that apply, and justify.

- Histogram stretching
- Image normalization (standardization)
- Gamma correction with γ < 1
- Median filtering with a 3×3 kernel
- A Sobel derivative filter

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
**Image normalization** and **a Sobel derivative filter** can both produce negative values.

- **Image normalization (standardization)** rescales so the result is zero-mean with unit variance; every pixel below the image mean is pushed below zero. (The provided min and max are irrelevant to this.)
- **A Sobel derivative filter** is a zero-sum derivative kernel: its response is a signed difference of intensities, so one side of an edge gives positive and the other negative responses. The sign encodes the direction of intensity change.
- **Histogram stretching** maps min → 0 and max → 255, so it stays non-negative.
- **Gamma correction with γ < 1** is a power law applied to non-negative inputs, so it stays non-negative.
- **Median filtering** only replaces a pixel with a value already present in the neighbourhood, so nothing can fall below 100.

The important distinction is *display* versus *computation*: signed derivative responses must be retained during computation (they carry orientation and polarity information). Only for display should you rescale, shift zero to mid-grey, or take absolute magnitudes — clipping negatives to zero destroys information irreversibly.
<!-- explanation:end -->

## q8 [medium]

Consider line detection in a noisy edge map with spurious edge pixels. Why is the Hough transform more suitable than least-squares line fitting that minimises the average squared distance?

A. Least squares cannot represent vertical lines, whereas Hough uses a normal form for all orientations.
B. Least squares is computationally more expensive than Hough for large images.
C. Least squares minimises a global squared error and is outlier-sensitive, whereas Hough accumulates local votes in parameter space, making it robust to scattered noise.
D. Least squares requires the number of lines to be known in advance.
E. Least squares assumes Gaussian noise on the dependent variable, which is violated by binary edge maps.

<!-- answer:start -->
exact
C
<!-- answer:end -->
<!-- explanation:start -->
Squaring the distance makes least squares extremely sensitive to outliers: a few spurious edge pixels far from the true line contribute quadratically and drag the fit away. The Hough transform instead has every edge point vote locally for all lines consistent with it; genuine collinear points reinforce one peak while scattered noise votes randomly and never accumulates a majority, so the true line emerges as a local maximum.

(A) is a real limitation of y = mx + b but not the reason asked for here — the normal parameterization ρ = x cos θ + y sin θ is what fixes it. (D) is false: least squares fits one line without being told how many exist. (B) and (E) are not the operative reasons.
<!-- explanation:end -->

## q9 [hard]

You must find all intersections formed by straight roads of at least 2 km, where the intersection angle satisfies 30° ≤ θ < 60°. Each pixel represents 100 m. Outline a Hough-based method, state a key assumption, then explain how you would restrict detection to **highways only**, given that highways and minor roads differ mainly in line thickness rather than colour.

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
**Method.**
1. **Preprocess** the map into an edge image (e.g. Canny), excluding weak-contrast structures such as lake boundaries, grey borders and tick marks so they cannot vote.
2. **Vote for lines** using the normal parameterization (ρ, θ), accumulating over all edge points in a 2D (ρ, θ) accumulator — this avoids the infinite-range problem of slope/intercept and represents vertical lines.
3. **Extract line peaks**, keeping only lines with sufficient votes. Vote count corresponds to the number of supporting edge pixels, i.e. line length; since 2 km at 100 m/pixel is 20 pixels, enforcing a minimum vote count enforces the length constraint.
4. **Check candidate pairs.** For each pair of accepted lines: compute the intersection angle and keep the pair only if 30° ≤ θ < 60°; verify the intersection point lies within the image; and confirm both composing lines individually satisfy the length constraint.

**Key assumption:** the preprocessing reliably excludes non-road structures (lake edges, borders) so that only road edges vote, and road segments are locally straight over the relevant distances. A secondary assumption is that the centreline/edge of each road produces a well-defined Hough peak.

**Limiting to highways only.** Highways are drawn thicker than minor roads. Increase the Gaussian smoothing σ in the preprocessing stage until the thin minor-road lines are smoothed away and no longer generate edges, so only thick highway edges contribute votes. Colour filtering is *not* appropriate here because highways and regular roads share the same dark blue colour — thickness, not colour, is the discriminating cue. Note also that simply raising the Canny threshold is insufficient/incorrect, since that removes edges by contrast rather than by line width.
<!-- explanation:end -->

## q10 [hard]

You apply standard K-means in a 5D feature space of (R, G, B, x, y) to segment an image into 100 regions. The result has irregular boundaries and converges slowly. You switch to SLIC superpixels instead. What key limitation of standard K-means does SLIC address?

A. K-means treats spatial and colour dimensions uniformly; SLIC modifies the distance metric to balance them differently.
B. K-means evaluates distances from every pixel to every cluster centre globally; SLIC restricts distance computations to a local 2s × 2s region around each centre.
C. K-means uses Euclidean distance; SLIC replaces it with a perceptually uniform metric in alternate colour spaces.
D. K-means optimises a non-convex objective; SLIC reformulates it as a convex optimisation.
E. K-means updates centres using the mean; SLIC updates them using the median for robustness.

<!-- answer:start -->
exact
B
<!-- answer:end -->
<!-- explanation:start -->
The limitation described in the problem is **slow convergence and irregular boundaries caused by global distance evaluation**, and SLIC's headline fix is exactly to restrict each centre's search to a local 2s × 2s window, which massively reduces computation and promotes spatial compactness. Both algorithms optimise the same non-convex objective and both update centres by the mean (so D and E are false). SLIC *does* weight spatial and colour components differently via the compactness parameter m, and it can be applied in other colour spaces — but those are not the fundamental limitation described here; they are distractors that describe true statements about SLIC while answering the wrong question.
<!-- explanation:end -->

## q11 [medium]

An encoder–decoder network produces a segmentation map with the same spatial dimensions as the input, yet thin structures and object boundaries are poorly localized. Which single change most directly addresses this, and why?

A. Upsample the final prediction to a higher resolution than the input.
B. Add skip connections that combine spatially aligned high-resolution encoder features with decoder features.
C. Increase the number of output classes.
D. Replace the softmax with a sigmoid.

<!-- answer:start -->
exact
B
<!-- answer:end -->
<!-- explanation:start -->
The problem is not output *resolution* but lost *detail*: pooling and striding in the encoder destroyed fine spatial information, and upsampling cannot recreate it. Skip connections copy spatially aligned encoder features into the decoder, restoring the fine-grained evidence needed to localize boundaries precisely. (A) merely adds more samples without adding information; (C) changes the label space and cannot sharpen boundaries; (D) changes the activation, not the available spatial detail.
<!-- explanation:end -->

## q12 [hard]

A segmentation model achieves 97% pixel-wise accuracy on a dataset where the object of interest covers 3% of pixels. Explain why this figure is not evidence of a good model, state which metrics you would report instead and what each reveals, and explain why a soft overlap loss is often preferred for training.

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
**Why 97% is not evidence.** Pixel accuracy is dominated by the majority class. A degenerate model predicting "background" for every pixel scores 97% here while detecting nothing at all. The metric cannot distinguish that failure from a genuinely good model, so it is uninformative under class imbalance. This is exactly why overlap-based measures are preferred.

**Metrics to report.**
- **IoU** = |A ∩ B| / |A ∪ B| over the foreground: penalises both missed foreground and added background, and is not inflated by the majority class.
- **Dice** = 2|A ∩ B| / (|A| + |B|): ranks predictions identically to IoU but yields numerically larger values (IoU = Dice/(2 − Dice); e.g. Dice 0.8 ⟺ IoU ≈ 0.67), so never compare the two numbers directly.
- **Precision and recall:** precision exposes false positives (over-segmentation), recall exposes false negatives (missed object). Both can be high individually while the other fails.
- Report **per class and averaged**, and state the **aggregation** — pooling all pixels differs from averaging per image or per object, because ten missed pixels is most of a small object but negligible in a large one.

**Why a soft overlap loss for training.** Training needs a differentiable signal optimised by gradient descent. Pixel-averaged cross-entropy is differentiable and softly penalises low confidence in the correct class, but it too is dominated by the many background pixels. A **soft Dice / overlap loss** computed on predicted probabilities directly rewards overlap between the predicted region and the ground truth, focusing learning on the region of interest rather than on the abundant background.

Finally, note that the training objective and the evaluation metric are deliberately different: loss is computed on **probabilities**, while IoU/Dice are computed on **thresholded hard masks**. Probabilities can improve for many epochs without crossing the decision threshold, so the training loss may fall while hard-mask Dice stays flat — not necessarily a bug, but persistent divergence is a reason to inspect predictions directly.
<!-- explanation:end -->
