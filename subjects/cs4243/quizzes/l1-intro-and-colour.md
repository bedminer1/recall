# CS4243 L1 — Introduction, Images & Colour Representations

Answer every question. Difficulty tags: `[easy]` one idea directly, `[medium]` combines steps or applies a definition, `[hard]` transfer / debugging / multi-step reasoning.

## q1 [easy]

In the simplified JPEG pipeline from Lecture 1, which operation **directly discards** image information?

A. Dividing the image into 8×8 blocks
B. Representing each block with a discrete cosine transform (DCT)
C. Setting some DCT weights to zero
D. Reordering the DCT weights using a zig-zag scan

<!-- answer:start -->
exact
C
<!-- answer:end -->
<!-- explanation:start -->
Quantization — setting some DCT weights to zero — is the lossy step that actually throws information away. Blocking (A) only partitions the image, the DCT (B) is an invertible transform that loses nothing by itself, and the zig-zag scan (D) only reorders coefficients so the zeros group together for efficient encoding.
<!-- explanation:end -->

## q2 [easy]

A camera captures uncompressed RGB images at 1000 × 1000 pixels with 8 bits per channel. Using 1 MB = 1,000,000 bytes and 1 byte = 8 bits, how many MB are required to store one raw frame? Give the number only.

<!-- answer:start -->
numeric 0
3
<!-- answer:end -->
<!-- explanation:start -->
Raw storage is b = M · N · k bits. For one channel: 1000 × 1000 × 8 bits = 8,000,000 bits = 1,000,000 bytes = 1 MB. An RGB image has three channels, so 3 × 1 MB = 3 MB. Note the contrast with the compressed JPEG example in the lecture, where the same content fits in a fraction of this.
<!-- explanation:end -->

## q3 [medium]

A raw RGB image is 1000 × 1000 pixels at 8 bits per channel. You halve both the width and the height, and you want the total storage to stay exactly the same by changing only the bit-depth per channel. What must the new bit-depth per channel be? Give the value only.

<!-- answer:start -->
numeric 0
32
<!-- answer:end -->
<!-- explanation:start -->
Storage is b = M · N · k. Halving both spatial dimensions divides the pixel count by 4, so keeping b fixed forces the bit depth to rise by the same factor of 4:

0.25 · M · N · k′ = M · N · k ⟹ k′ = 4k = 4 × 8 = **32 bits per channel**.

Check the absolute numbers: the original is 1000·1000·3·8 = 24,000,000 bits, and the halved image at 32 bits per channel is 500·500·3·32 = 24,000,000 bits. Identical, as required.

The reasoning worth internalizing: **spatial resolution and intensity resolution trade off linearly, not independently.** Cutting the pixel count by 4× forces a 4× increase in bit depth to compensate. The useful direction in practice is the reverse — you normally *shrink* the bit depth to buy back the storage that downsampling saves, and 32 bits per channel is far more precision than any display or sensor uses. The arithmetic answer is 32; whether anyone would want it is a separate question.

A common wrong answer is 2, which comes from applying the factor of 4 in the wrong direction (that would be the bit depth you could afford if you *wanted* storage to fall 16×).
<!-- explanation:end -->

## q4 [easy]

A Bayer sensor contains M×N photosites, each measuring only one colour component, yet after demosaicking the image is represented as an M×N×3 RGB array. Which statement best explains this?

A. The sensor directly measured all three colour components at every location.
B. Demosaicking estimates the two missing colour components at each location using nearby measurements.
C. Each sensor measurement is copied into the red, green, and blue channels.
D. Demosaicking triples the spatial resolution of the image.

<!-- answer:start -->
exact
B
<!-- answer:end -->
<!-- explanation:start -->
Demosaicking interpolates the two missing channel values at each site from neighbouring photosites, which is why a full M×N×3 colour image comes from only M×N measurements. It increases the number of *represented colour values*, not the number of *spatial sample locations* — so (D) is wrong, and (C) is wrong because a single measurement is one colour, not all three.
<!-- explanation:end -->

## q5 [easy]

A grayscale image is reduced from 256 possible intensity levels to only 16. Which visual artifact is most likely to appear?

A. Visible bands appear across smoothly varying regions.
B. Halos appear around sharp edges.
C. Random speckle appears in uniform regions.
D. Regular 8×8 block boundaries become visible.

<!-- answer:start -->
exact
A
<!-- answer:end -->
<!-- explanation:start -->
A smoothly varying region needs many intensity levels to be rendered smoothly; with too few levels it is quantized into visible bands. This is called **false contouring** or colour banding. (B) is ringing, (D) is JPEG blocking — both compression artifacts, not the direct consequence of low intensity resolution.
<!-- explanation:end -->

## q6 [medium]

A drone captures a night image of a dark forest: 94.1% of pixels have intensities uniformly distributed between 10 and 15, and the remaining 5.9% are bright sky pixels with intensity 255. After applying histogram equalization, how many unique intensity values will the image have? Give the number only.

<!-- answer:start -->
numeric 0
7
<!-- answer:end -->
<!-- explanation:start -->
Histogram equalization redistributes the intensity levels that are *already present* according to the cumulative distribution; it cannot invent new levels. The levels present are {10, 11, 12, 13, 14, 15, 255} — that is 7 unique values. The "94.1%" detail matters: it implies the image has enough pixels that quantization collisions are impossible, so no two input levels collapse onto the same output.
<!-- explanation:end -->

## q7 [hard]

A night image has dark pixels occupying intensities 10, 11, 12, 13, 14 and 15, together accounting for a proportion of 240/255 of the image and spread uniformly across those six levels. The remaining pixels are bright sky at intensity 255, accounting for a proportion of 15/255. Under histogram equalization, what output intensity does an input value of 12 map to? Give the number only.

<!-- answer:start -->
numeric 0.5
120
<!-- answer:end -->
<!-- explanation:start -->
Equalization maps an intensity to its cumulative probability (scaled to the output range). The dark pixels total 240/255 spread uniformly over 6 levels, so each dark level has proportion (240/255)/6 = 40/255. Values 10, 11, and 12 contribute cumulatively: 40/255 + 40/255 + 40/255 = 120/255. Mapping that cumulative proportion onto 0–255 gives 120.
<!-- explanation:end -->

## q8 [medium]

An algorithm must detect a specific orange basketball in a video feed where illumination varies drastically between bright sunlight and deep shadow, changing pixel intensities. Which colour representation is most robust for defining "orange" independently of brightness?

A. Raw RGB values
B. Greyscale intensity using weights wR = 0.8, wG = 0.2, wB = 0
C. Greyscale intensity using weights wR = 0.299, wG = 0.587, wB = 0.114
D. Normalized RGB
E. The V channel of HSV

<!-- answer:start -->
exact
D
<!-- answer:end -->
<!-- explanation:start -->
Normalized RGB divides by the sum of the channels (r + g + b = 1), which removes the shading/intensity component so that colour ratios stay consistent as illumination changes. Raw RGB (A) entangles colour with intensity, greyscale (B, C) discards colour entirely, and V in HSV (E) *is* the intensity-like component, so it varies with illumination.
<!-- explanation:end -->

## q9 [medium]

Recall the image formation model f(x, y) = i(x, y) · r(x, y). A flat, uniform sheet of white paper lies on a desk, lit by a lamp positioned to the left. As you scan from the left edge of the paper to the right edge, how do the two components behave?

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
Reflectance r(x, y) ∈ [0, 1] is a property of the material. The paper is uniform and white, so r is roughly constant (high). Illumination i(x, y) is the incident light, which falls off with distance from the lamp, so it decreases moving away from the left. The observed darkening to the right is therefore an illumination effect, not a change in the surface.
<!-- explanation:end -->

## q10 [hard]

A student converts a colour image to greyscale, then reports that the conversion "has reduced the spatial resolution of the image by a factor of three." Identify what is actually wrong with this claim, and state what information is genuinely lost.

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
Spatial resolution is unchanged: the greyscale image still has the same M×N grid of pixel locations, one intensity per location. What is lost is *colour* information — the three per-pixel channel values (R, G, B) collapse into a single weighted intensity I = wR·R + wG·G + wB·B. A correct description would be that intensity resolution/representation of colour is reduced (three channels to one), not that the image was downsampled. Note also that with the usual weights (e.g. 0.299, 0.587, 0.114) the mapping is many-to-one: distinct colours with the same weighted sum become indistinguishable.
<!-- explanation:end -->

## q11 [medium]

An image has minimum intensity 100 and maximum 200. After a point-processing operation, some output pixel values are negative. Which operation was definitively applied?

A. Histogram stretching
B. Image normalization
C. Gamma correction with γ < 1
D. Histogram equalization
E. Median filtering

<!-- answer:start -->
exact
B
<!-- answer:end -->
<!-- explanation:start -->
Image normalization (whitening) rescales and offsets so the result is zero-mean with unit variance; shifting by the mean pushes any pixel below the mean to a negative value. Histogram stretching maps the minimum to 0 and the maximum to the top of the range, so it stays non-negative. Gamma correction with γ < 1 is a power law on non-negative inputs and stays non-negative. Median filtering only replaces a pixel with another value already in the image, so nothing sinks below 100.
<!-- explanation:end -->

## q12 [hard]

Consider the definition of HSV. You are given one pixel with RGB = (150, 30, 200). State the V component and explain why V alone cannot be used to identify the colour of an object under varying illumination, then name the component you would use instead.

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
V = max(R, G, B) = 200. V is the achromatic "value" axis running from black to white, so it encodes the intensity/brightness of the pixel rather than its hue — two completely different colours can share the same V, and the same colour under dimmer light gets a smaller V. To identify colour irrespective of illumination you should use **H (hue)**, the "pure" colour angle from 0 to 360, optionally with S (saturation, the purity from white to fully saturated) to distinguish washed-out from vivid versions. V should be normalized away or used only for shading.
<!-- explanation:end -->
