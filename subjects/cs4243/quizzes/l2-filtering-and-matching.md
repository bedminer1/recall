# CS4243 L2 — Point Processing, Filtering & Matching (midterm-style arithmetic)

Mixed conceptual and computational questions in the style of the L2 review questions and the midterm fill-in-the-blank section.

## q1 [easy]

An input image has shape (H, W, C) = (256, 256, 3). You apply **one** 5×5×3 filter with stride 2 and zero-padding of 2 pixels on each spatial side. What is the output shape (H_out, W_out, C_out)?

A. (256, 256, 1)
B. (128, 128, 3)
C. (128, 128, 1)
D. (126, 126, 1)

<!-- answer:start -->
exact
C
<!-- answer:end -->
<!-- explanation:start -->
Spatial size: H_out = (H + 2P − K)/S + 1 = (256 + 4 − 5)/2 + 1 = 255/2 + 1 = 127 + 1 = 128. The same holds for W, giving 128 × 128. Depth: the filter has depth 3, matching the input's 3 channels, and there is only one filter, so C_out = 1. Option D is the no-padding result, and B forgets that a single filter produces a single channel.
<!-- explanation:end -->

## q2 [medium]

A Gaussian filter has σ = 3 and a 7×7 kernel. The kernel is widened to 15×15 while σ stays 3, and both are normalized to sum to one. Which statement is most accurate?

A. The two filters must produce identical outputs because σ is unchanged.
B. The larger kernel better captures the Gaussian tails; once the support is sufficiently large, further increases have little effect.
C. The larger kernel has a larger effective σ because kernel size determines the variance.
D. The larger kernel produces less smoothing because its weights are spread over more pixels.

<!-- answer:start -->
exact
B
<!-- answer:end -->
<!-- explanation:start -->
σ determines the *shape* (variance) of the Gaussian; kernel size determines only how much of that Gaussian is sampled. A 7×7 kernel truncates the tails more aggressively than 15×15, so their outputs differ slightly (A is false), but for σ = 3 a 15×15 support already covers the distribution well, so going wider changes little. (C) confuses kernel size with variance, and (D) is wrong because the smoothing *scale* is set by σ, not by how many pixels the weights are spread over.
<!-- explanation:end -->

## q3 [medium]

Images A and B have the same minimum and maximum intensity values but different intensity distributions. Histogram stretching and histogram equalization are each applied independently to both images. Which statement is correct?

A. Both operations apply the same mapping to A and B.
B. Both operations generally apply different mappings to A and B.
C. Equalization applies the same mapping to A and B, but stretching generally applies different mappings.
D. Stretching applies the same mapping to A and B, but equalization generally applies different mappings.

<!-- answer:start -->
exact
D
<!-- answer:end -->
<!-- explanation:start -->
Histogram stretching is a fixed affine map determined only by the endpoints: with the same min and max, A and B get exactly the same transformation. Equalization maps each level to its cumulative probability, which depends on the *full histogram* — how often each intensity occurs — so different distributions give generally different mappings. (C) is the reversed version of the correct answer.
<!-- explanation:end -->

## q4 [medium]

Let T be a non-constant template and suppose an image patch is given by p = a·T + b for constants a > 0 and b. Using exact arithmetic and **zero-mean, unit-norm** normalized cross-correlation, which statement is correct?

A. The normalized score is 1, while the raw correlation score can change with both a and b.
B. The normalized score is a, while the raw correlation score is unaffected by b.
C. The normalized score is 0 because subtracting the mean removes the matching pattern.
D. The normalized score depends only on b, while the raw correlation score depends only on a.

<!-- answer:start -->
exact
A
<!-- answer:end -->
<!-- explanation:start -->
The patch is a positive affine transform of the template, so it is perfectly correlated with it: after subtracting the mean (which removes b) and dividing by the norm (which removes a), the zero-mean unit-norm NCC is exactly 1. Raw correlation computes Σ p·T, which scales with a and shifts with b, so it changes with both. (C) is the classic misconception — subtracting the mean removes the *offset*, not the pattern.
<!-- explanation:end -->

## q5 [hard]

You apply a 2D Laplacian filter with kernel [0 1 0; 1 −4 1; 0 1 0] to a region of an image. The image is a large uniform area of intensity 100 on the left and a large uniform area of intensity 200 on the right, with a single vertical step edge between them. Without any border padding, what is the response at a pixel far from the edge, and what is the response exactly on the edge column?

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
Far from the edge the 3×3 neighbourhood is uniform, so the Laplacian gives 0·100 + 1·100 + 0·100 + 1·100 − 4·100 + 1·100 + 0 + 1·100 + 0 = 100(1 + 1 − 4 + 1 + 1) = 0. The kernel sums to zero, so any constant region responds with 0. This is the key property: a zero-sum (derivative) kernel has no response to flat areas.

Exactly on the edge, the neighbourhood mixes both sides. For a pixel in the 200 region adjacent to the step, the centre and its left neighbour sit on opposite sides: the response becomes non-zero — the Laplacian has a positive lobe on one side of the edge and a negative lobe on the other. The edge is *between* those two lobes, which is exactly where the Laplacian crosses zero, motivating edge localization by zero-crossings of the second derivative.
<!-- explanation:end -->

## q6 [medium]

A student applies a 3×3 median filter to a region corrupted by salt-and-pepper noise, where no two corrupted pixels lie within each other's 8-neighbourhood. What is the maximum number of unique intensity values in the output?

A. 2
B. 3
C. 9
D. 512

<!-- answer:start -->
exact
A
<!-- answer:end -->
<!-- explanation:start -->
A 3×3 window contains 9 pixels, of which at most one is corrupted. The median is the 5th smallest value, and a single extreme value (0 or 255) can never be the 5th of 9 when the other 8 are the original image values — so the corrupted pixel is always rejected and the median returns an uncorrupted value. With only the two original levels (e.g. 100 and 200) surviving, the output has 2 unique values. This is the classic reason median filtering is so effective against salt-and-pepper noise.
<!-- explanation:end -->

## q7 [hard]

The same image (levels 100 and 200) is instead corrupted by additive Gaussian noise N(0, 20²), again with at most one corrupted pixel per 8-neighbourhood. A 3×3 median filter is applied. Explain why the output can contain many more unique values than the salt-and-pepper case, and why every output value must still lie within [100, 200].

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
With additive Gaussian noise the corrupted pixel is not an extreme 0 or 255 but an arbitrary value drawn from a continuous distribution. Once it is *inside* the range spanned by the uncorrupted neighbours, it can legitimately be the median of the 9 values, so it is no longer rejected. The 5th-smallest of nine values therefore varies continuously from window to window, producing many distinct outputs.

Every output must lie within [100, 200] because the median is one of the nine values in the window, and the only values that can ever be the median must lie between the lowest and highest uncorrupted values present. A corrupted value below 100 or above 200 would have to be the 5th of 9, but there are always at least four uncorrupted pixels on each side (the window contains at most one corrupted pixel), so such outliers are always rejected. Hence the output is confined to the original dynamic range even though its values become varied.
<!-- explanation:end -->

## q8 [medium]

Consider a 55×55 normalized box filter applied to an image made of large blocks (50×50 squares of intensity 200) with thin 5-pixel bars of intensity 100. Why does the output have only **one** unique intensity value?

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
A normalized box filter returns the *average* of the window. The kernel is sized (55×55, larger than a 50×50 square plus the 5-pixel bars) so that every window position covers the same proportion of bright and dark content — the kernel always contains the same number of 200-pixels and 100-pixels regardless of where it is centred. Averaging a fixed mixture of the two levels yields the same weighted average everywhere, so the output is a constant. Contrast this with a small 3×3 kernel, which samples many distinct local mixtures and therefore produces several unique values.
<!-- explanation:end -->

## q9 [medium]

Two Gaussian filters are both normalized to sum to one. Filter A is a 7×7 kernel with σ = 3; filter B is a 15×15 kernel with the same σ = 3. A student claims: "since both kernels are normalized to sum to one, they must produce identical outputs." Explain precisely what is wrong with this reasoning.

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
Normalization only guarantees that each kernel has unit total weight, so neither kernel changes the overall brightness of a constant region. It does **not** force the weights to be distributed identically across pixels. A 7×7 Gaussian with σ = 3 puts its unit mass into 49 weights; a 15×15 Gaussian with the same σ spreads its unit mass over 225 weights, with a much smaller value at each position and non-trivial weight where the 7×7 kernel had none. Different weight *distributions* give different weighted averages, so the outputs differ — the larger kernel captures more of the Gaussian's tails, giving slightly more smoothing and better suppression of high-frequency noise.
<!-- explanation:end -->

## q10 [hard]

You must choose between a 3×3 box filter, a 3×3 Gaussian filter, and a 3×3 median filter for each of the following tasks. Justify each choice in one or two sentences.

(a) Removing isolated salt-and-pepper noise.
(b) Producing a smooth, isotropic blur with minimal ringing.
(c) A separable implementation for a fast, large-scale blur where speed is the priority.

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
(a) **Median filter.** It is a rank/order-statistic filter, not a weighted average, so an isolated extreme value can never be the median and is discarded entirely rather than smeared across the neighbourhood. Box and Gaussian filters would average the outlier into neighbouring pixels, spreading the corruption.

(b) **Gaussian filter.** It is a smooth, radially symmetric (isotropic) low-pass kernel whose weights decay with distance, so it produces no directional bias and essentially no ringing. A box filter has abrupt weight discontinuities at its boundary, which produces more ringing and block-like artifacts.

(c) **Box filter.** Its kernel is uniform and therefore separable into two identical 1D passes (a row average followed by a column average), and its running-sum form makes large kernels very cheap. A Gaussian is also separable in principle, but a box filter is the cheapest; the median filter is **not** separable at all, since ranking cannot be decomposed into independent 1D operations.
<!-- explanation:end -->

## q11 [hard]

You are building a texture classification system. The training set has 100 images of size 100×100. A filter bank of 10 filters, each 11×11, is applied to every image with no border padding. The pixel-wise filter responses are clustered with K-means into 50 textons, and each image is represented by one 50-bin histogram. If a quantity cannot be determined, write -1.

(a) What is the dimensionality of each feature vector fed into K-means?
(b) How many feature vectors in total are fed into K-means?
(c) How many squared-difference terms are computed to classify one test image against all training images?

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
(a) **10.** At each pixel location the response of every one of the 10 filters forms one feature vector, so the dimensionality is the number of filters. The spatial size of the filter is irrelevant to the feature dimensionality.

(b) **810,000.** With no padding, an 11×11 filter applied to a 100×100 image produces a valid output region of (100 − 11 + 1) × (100 − 11 + 1) = 90 × 90 = 8100 pixel positions. Each position yields one feature vector, so each image contributes 8100 vectors, and 100 images contribute 8100 × 100 = 810,000 vectors.

(c) **5000.** Each image is summarized as a 50-bin histogram, so comparing two histograms costs one squared-difference term per bin — 50 terms. Classifying a test image requires comparing against every training image: 50 × 100 = 5000 terms. (Note the contrast with (b): the histogram collapses the 8100 per-image vectors into a single 50-dimensional descriptor, which is the whole point of the texton representation.)
<!-- explanation:end -->

## q12 [hard]

A classmate claims: "Histogram equalization always makes an image's histogram flat, so afterwards all 256 intensity levels are present." Evaluate this claim and explain what equalization actually does to the histogram of a *discrete* image.

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
The claim is false for discrete images. Histogram equalization maps each intensity to its cumulative probability, which spreads out the most *frequent* levels and compresses the rarest ones — so the histogram becomes more *uniformly distributed*, not necessarily exactly flat. Two consequences follow: (1) equalization cannot create levels that were absent from the input, so the number of unique output values is bounded by the number of unique input values — a dark image whose pixels take only a handful of distinct intensities still has only that many distinct intensities afterwards, however many levels the 0–255 range nominally offers; and (2) in the continuous limit the transformation does produce a uniform density, which is where the misconception comes from. For a discrete histogram, large gaps appear where the CDF jumps steeply, which is exactly the signature used to identify equalization in histogram-diagnosis questions.
<!-- explanation:end -->
