# CS4243 L3 — Gradients, Edges & the Hough Transform

Built from the L3 review and in-lecture questions. Focus: derivative filters, Canny, and voting-based line detection.

## q1 [easy]

A dark-to-bright **vertical** edge is present in an image. Which statement is correct?

A. Gx will be strongest, and the gradient points across the edge.
B. Gx will be strongest, and the gradient points along the edge.
C. Gy will be strongest, and the gradient points along the edge.
D. Gx and Gy must have equal magnitudes.

<!-- answer:start -->
exact
A
<!-- answer:end -->
<!-- explanation:start -->
Gx measures the rate of intensity change in the x-direction, so it captures vertical edges (the intensity changes as you move horizontally across the edge). The gradient is perpendicular to the edge — it points in the direction of increasing intensity, i.e. across the edge from dark to bright, not along it.
<!-- explanation:end -->

## q2 [easy]

Why does the Sobel x-filter contain negative weights on one side and positive weights on the other?

A. To compute the difference between intensities on opposite sides
B. To ensure that all output values remain between 0 and 255
C. To detect only bright lines on dark backgrounds
D. To make the filter invariant to edge orientation

<!-- answer:start -->
exact
A
<!-- answer:end -->
<!-- explanation:start -->
The mirrored signs make the kernel compute a weighted difference between the brightness on the left and the brightness on the right. A large difference means a vertical edge is present. (B) is false because the response is signed, encoding the *direction* of change; (C) is false because the sign of the response distinguishes dark-to-bright from bright-to-dark; (D) is false because the x-filter is deliberately orientation-selective.
<!-- explanation:end -->

## q3 [easy]

Which Canny step is primarily responsible for producing **thin** edges?

A. Gaussian smoothing
B. Point interpolation
C. Non-maximum suppression
D. Hysteresis thresholding

<!-- answer:start -->
exact
C
<!-- answer:end -->
<!-- explanation:start -->
After computing the gradient magnitude, non-maximum suppression keeps only pixels that are a local maximum along the gradient direction, suppressing neighbouring weaker responses and thinning the broad gradient ridge to roughly one pixel. Gaussian smoothing (A) is for noise reduction, and hysteresis (D) decides which surviving edges to keep — neither thins the edge.
<!-- explanation:end -->

## q4 [medium]

Suppose we increase the Gaussian σ used for smoothing before computing image gradients. What is the most likely effect?

A. Greater noise sensitivity and more precise localization
B. Greater noise sensitivity and less precise localization
C. Less noise sensitivity and more precise localization
D. Less noise sensitivity and less precise localization

<!-- answer:start -->
exact
D
<!-- answer:end -->
<!-- explanation:start -->
A larger σ suppresses more high-frequency content, so the gradient becomes less sensitive to noise. But the same smoothing spreads an intensity step over a wider region, so the location of maximum gradient (the edge) becomes broader and harder to pin down. This is the classic smoothing trade-off: noise robustness versus localization accuracy.
<!-- explanation:end -->

## q5 [medium]

A horizontal derivative filter produces both positive and negative values. Before computing the gradient **orientation**, what should you do?

A. Retain the signed values.
B. Set all negative values to zero.
C. Replace every value by its absolute magnitude.
D. Rescale the values independently into the range 0–255.

<!-- answer:start -->
exact
A
<!-- answer:end -->
<!-- explanation:start -->
The sign of each partial derivative encodes which direction the intensity is increasing, so it must be preserved for atan2(Gy, Gx) to give the correct quadrant. Taking absolute values (C) or clipping negatives (B) destroys the quadrant information and would restrict orientations to a single half-plane. Absolute magnitude is legitimate only *after* the orientation has been computed, or when you deliberately want direction-agnostic edge strength.
<!-- explanation:end -->

## q6 [medium]

For an ideal intensity step that has first been smoothed, where is the edge located?

A. At a zero-crossing of the first derivative
B. At a maximum of the first-derivative magnitude
C. At a maximum of the Laplacian magnitude
D. Where both the first derivative and the Laplacian are zero

<!-- answer:start -->
exact
B
<!-- answer:end -->
<!-- explanation:start -->
After smoothing, the step becomes a ramp. Over the ramp the first derivative is a bump, and its **magnitude** peaks where the intensity changes most rapidly — the centre of the edge. The Laplacian (second derivative) has a positive lobe on one side and a negative lobe on the other with a zero-crossing between them, so it localizes the edge at a *zero-crossing*, not a maximum (C is wrong). Option (D) is wrong because the first derivative is at its maximum magnitude there, not zero.
<!-- explanation:end -->

## q7 [hard]

A pixel P has gradient magnitude 80 and a horizontal gradient direction. Its two neighbours along the gradient direction have magnitudes 90 and 70. Canny is run with a high threshold of 75 and a low threshold of 40. What happens to P?

A. It becomes a strong edge because 80 exceeds the high threshold.
B. It becomes a weak edge because one neighbouring response is larger.
C. It is removed during non-maximum suppression.
D. It is retained only if connected to the neighbouring pixel with magnitude 90.

<!-- answer:start -->
exact
C
<!-- answer:end -->
<!-- explanation:start -->
Canny applies non-maximum suppression **before** thresholding. Since the neighbour with magnitude 90 is larger and lies along the gradient direction, P is not a local maximum and is suppressed — the fact that 80 > 75 never comes into play. Options (A) and (D) mistakenly assume thresholding happens first; hysteresis connectivity is only considered for pixels that survive non-maximum suppression.
<!-- explanation:end -->

## q8 [easy]

In the Hough transform, an image-space point corresponds to:

A. One set of parameters for a single line
B. All possible lines passing through that point
C. One image gradient
D. One fixed orientation

<!-- answer:start -->
exact
B
<!-- answer:end -->
<!-- explanation:start -->
A single point does not determine a unique line — infinitely many lines pass through it. So the point votes for every line that could pass through it, and in the (ρ, θ) parameter space those possibilities trace out a sinusoid. A line emerges as a peak where many points' sinusoids intersect.
<!-- explanation:end -->

## q9 [medium]

Consider the normal parameterization ρ = x cos θ + y sin θ. Which parameter pair describes two straight lines that are **perpendicular** to each other?

A. θ₂ = θ₁
B. θ₂ = θ₁ + π/2
C. θ₂ = θ₁ + π, ρ₂ = −ρ₁
D. θ₂ = θ₁ + π/4, ρ₂ = ρ₁

<!-- answer:start -->
exact
B
<!-- answer:end -->
<!-- explanation:start -->
θ is the orientation of the line's **normal**, so two lines are perpendicular exactly when their normals differ by π/2 (mod π) — which is option B. Option A gives two parallel lines; option C (θ₂ = θ₁ + π, ρ₂ = −ρ₁) is the *same* line written differently, since (ρ, θ) and (−ρ, θ + π) denote an identical line; option D is a 45° relationship. This is why a normal-form parameterization handles all orientations, including vertical lines, which y = mx + b cannot represent.
<!-- explanation:end -->

## q10 [hard]

Consider line detection in a noisy edge map containing spurious edge pixels. Explain why the Hough transform is more suitable than least-squares line fitting that minimises the average squared distance.

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
Least squares squares the distance of every point from the line, which heavily penalises far-away points. Noisy, spurious edge pixels are exactly such outliers, and because their contribution grows quadratically they drag the fitted line away from the true edge.

The Hough transform instead lets every edge point cast local votes for all parameter combinations consistent with it. Genuine collinear points all vote for the same (ρ, θ) cell and reinforce a peak, while scattered noise votes randomly across parameter space and never accumulates a consistent majority. The true line therefore emerges as a distinct local maximum. In short: least squares is a global, outlier-sensitive fit, whereas Hough accumulation is a robust, local-evidence vote.
<!-- explanation:end -->

## q11 [hard]

In the standard circle Hough transform the radius is unknown, so the accumulator is 3D over (a, b, r). If the gradient orientation φ at an edge point (xᵢ, yᵢ) is used to constrain the possible circle centres, what geometric shape do that point's votes form in the 3D accumulator space?

A. One 3D cone
B. One 2D circle
C. Two 2D circles that intersect each other
D. Two discrete points
E. Two straight lines projecting through the 3D space

<!-- answer:start -->
exact
E
<!-- answer:end -->
<!-- explanation:start -->
Using the gradient, the centre must lie on the line through the edge point along the gradient direction. For a given radius r the centre is at a = xᵢ − r cos φ, b = yᵢ − r sin φ (or a = xᵢ + r cos φ, b = yᵢ + r sin φ on the opposite side). Since xᵢ, yᵢ and φ are fixed, a and b vary **linearly** with r, so each family traces a straight line through the 3D (a, b, r) accumulator. The two candidate sides give two straight lines. Without the gradient constraint each point would vote for an entire cone, which is the expensive case the gradient direction is used to avoid.
<!-- explanation:end -->

## q12 [hard]

You are given a road map where each pixel represents 100 m. You must find every intersection formed by two straight roads of length at least 2 km, where the intersection angle satisfies 30° ≤ θ < 60°. Outline a Hough-transform-based method, state a key assumption, and explain how you would restrict detection to **highways only**, given that highways and minor roads differ mainly in line thickness rather than colour.

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
Method:
1. **Preprocess** the map into an edge map (e.g. Canny) and exclude weak-contrast structures such as lake boundaries and grey borders, so they cannot vote.
2. **Vote for lines** using the normal parameterization (ρ, θ), building a 2D accumulator over all edge points.
3. **Detect line peaks** and convert each back to image space, keeping only lines with sufficient votes — a vote count corresponds to line length, so requiring enough votes enforces the ≥ 2 km (≥ 20 pixel) minimum length.
4. **Check intersections pairwise.** For each pair of accepted lines compute the intersection angle and keep the pair only if 30° ≤ θ < 60°; verify the intersection point lies inside the image and that both composing lines individually satisfy the length constraint.

Key assumption: light-grey structures (lake boundaries, borders) and minor roads can be reliably excluded by the preprocessing/voting stage, or equivalently that the edges of the roads of interest are strong enough to form clear Hough peaks. A further assumption is that roads are locally straight over the relevant length.

Restricting to highways only: highways are drawn with thicker lines, so increase the Gaussian σ in the preprocessing stage until thin minor roads are smoothed away and no longer produce edges; only the thick highway edges then contribute votes. Colour filtering is not appropriate here because highways and regular roads share the same dark blue colour — thickness, not colour, is the discriminating cue.
<!-- explanation:end -->
