# CS4243 L4 — Filter Banks, Multiscale Reasoning & Texture

Covers Gabor filters, image pyramids, texture representation by pooling, and the hand-designed → learned transition.

## q1 [easy]

A filter bank is best described as:

A. A single kernel applied at many image locations
B. A collection of multiple filter kernels covering combinations of orientations and scales
C. A classifier trained on texture vectors
D. An image pyramid

<!-- answer:start -->
exact
B
<!-- answer:end -->
<!-- explanation:start -->
A filter bank is a *set* of kernels chosen to cover different patterns — typically across orientations, frequencies and scales. Applying one filter bank to an image gives each pixel a K-dimensional response vector r(x, y) = [r₁(x, y), …, r_K(x, y)], one component per filter. (A) describes convolution itself, not a bank; (C) is the classifier stage that sits *after* filtering; (D) is a multiscale image representation, a separate tool.
<!-- explanation:end -->

## q2 [medium]

Two Gabor filters have the same orientation and the same spatial frequency, but Filter B has a larger Gaussian σ than Filter A. Which statement is correct?

A. Filter B has more widely spaced stripes.
B. Filter B examines a larger neighbourhood, but its stripe spacing is unchanged.
C. The filters are identical because their frequencies are equal.
D. Filter B detects a different orientation.

<!-- answer:start -->
exact
B
<!-- answer:end -->
<!-- explanation:start -->
The two parameters do different jobs: the **sinusoid frequency** sets the stripe spacing, while the **Gaussian envelope σ** sets how large a neighbourhood the filter looks at. A larger σ widens the envelope, so the filter observes more repetitions and uses more spatial context, but the stripes stay in the same places. (C) is wrong because different envelopes mean different responses even at equal frequency.
<!-- explanation:end -->

## q3 [medium]

Why must a Gaussian smoothing be applied *before* downsampling an image to construct a pyramid?

A. To increase image contrast
B. To decrease image contrast
C. To suppress detail that the lower-resolution grid cannot represent reliably
D. To make all objects appear at the same size

<!-- answer:start -->
exact
C
<!-- answer:end -->
<!-- explanation:start -->
Downsampling removes spatial samples. If fine high-frequency detail is still present, it gets misrepresented as false coarse patterns in the lower-resolution image — this is **aliasing**. Pre-smoothing removes the detail that the coarser grid cannot carry, so the remaining content is representable. Note that smoothing *reduces* contrast in fine detail rather than increasing it, and objects become smaller (not equal-sized) at higher pyramid levels.
<!-- explanation:end -->

## q4 [medium]

Two image patches contain the same local texture elements, but those elements occur at slightly different positions. Why can their **pooled** texture vectors still be similar?

A. Pooling summarizes which responses occur and how strongly, rather than their exact positions.
B. Pooling preserves the exact position of every response.
C. Pooling changes the frequencies of the filters.
D. Pooling guarantees invariance to every rotation and scale.

<!-- answer:start -->
exact
A
<!-- answer:end -->
<!-- explanation:start -->
A response at a single pixel is too localized to describe a texture; pooling aggregate statistics (mean, energy, variance) of each filter's response over a region Ω trades away exact positional information in exchange for describing *which patterns occur and how strongly*. That discarded detail is exactly why small rearrangements leave the summary nearly unchanged. (B) is the opposite of what pooling does, and (D) is too strong: an orientation-selective bank still produces different vectors when the texture rotates.
<!-- explanation:end -->

## q5 [hard]

A student claims that a CNN's "learning" is confined to the classifier at the end, and that the convolutional kernels are still hand-designed basis functions like Gabor filters. Evaluate the claim, naming which components are learned and which are fixed by the designer.

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
The claim is wrong. In a CNN the **convolutional kernel weights themselves are learned** from training data so as to lower the prediction error — a convolutional layer can be understood as a *learned filter bank*, with each kernel producing one feature map. What is fixed by the designer is the **architecture**: the number of layers, kernel sizes, strides, and the choice of nonlinearity and pooling rule. So ReLU (a = max(0, r)) and standard max pooling are fixed nonlinear/aggregation operations, not learned; some networks do use learned strided convolutions in place of pooling. Crucially, the kernels and the classifier are trained **jointly end-to-end**, rather than hand-designing features and then training a classifier on top as in the classical pipeline (filter bank → pooling → separately trained classifier such as nearest-neighbour or nearest-mean).

Interesting nuance: the *content* of learned early-layer kernels often resembles Gabor filters — oriented light–dark transitions at various frequencies — because those are broadly useful for natural images. But that is an empirical outcome of learning, not a hand-imposed basis.
<!-- explanation:end -->

## q6 [medium]

Why is a nonlinearity such as ReLU necessary between convolutional layers?

A. It normalises the feature maps to a fixed range
B. Without it, stacked convolutions collapse into a single equivalent linear filter
C. It performs the downsampling that pooling would otherwise do
D. It increases the number of learnable parameters

<!-- answer:start -->
exact
B
<!-- answer:end -->
<!-- explanation:start -->
Convolution is associative: two stacked convolutions give r₂ = k₂ * (k₁ * x) = (k₂ * k₁) * x, i.e. a *single* effective kernel k_eff. Depth would buy nothing. Inserting ReLU breaks this: r₂ = k₂ * ReLU(k₁ * x) cannot be collapsed into one linear operation, so successive layers can combine responses conditionally and represent genuinely more complex patterns. ReLU itself has no learnable parameters and does not downsample.
<!-- explanation:end -->

## q7 [hard]

Suppose you remove every ReLU from a deep convolutional network, keeping all the convolution and pooling layers and their learned weights. Describe quantitatively what the network's representational capacity becomes, and explain why this makes depth pointless.

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
Because convolution is linear and associative, any chain of convolution layers collapses into one equivalent convolution. For input x and kernels k₁, k₂, …, kₙ (ignoring pooling for the moment), the composition equals (kₙ * … * k₁) * x — one effective kernel k_eff operating on the input. The entire "deep" stack is therefore equivalent to a **single linear filter**, and its capacity is no greater than that shallow network.

Max pooling is itself a fixed nonlinear (piecewise-linear-ish) operation, so a network with pooling but no ReLU is at least not strictly a single convolution; however, pooling only performs fixed spatial aggregation and cannot compensate for the loss of learned nonlinear feature combination. The practical consequence: the network can still learn linear filters, but it cannot build hierarchical composition — e.g. edges into parts into objects — because each new layer merely re-filters with no ability to threshold/rectify. Its expressive power drops dramatically, and the benefit of depth vanishes. This is exactly the derivation from the lecture: without ReLU, k₁ and k₂ associate; with ReLU, they cannot.
<!-- explanation:end -->

## q8 [hard]

Consider the comparison between classical filter-bank pipelines and CNNs along four axes: where kernels come from, depth, how spatial aggregation is used, and when the classifier is trained. Explain how each axis differs, and why the CNN's joint training matters.

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
1. **Kernels.** Filter banks are *manually specified* — the type and parameters (orientation, frequency, scale) are chosen by hand, possibly from a parametric family such as Gabor filters. CNN kernels are *learned from data* by lowering prediction error. This is the fundamental shift.

2. **Depth.** A classical filter bank is shallow — usually one bank applied once, with no learned nonlinearities. A CNN is deep, stacking many convolutional blocks interleaved with nonlinearity and downsampling. Stacking increases the **receptive field** — the set of input pixels an output depends on — so deeper layers aggregate evidence over larger image regions and build hierarchical features.

3. **Spatial aggregation.** The classical pipeline pools a whole region into *one* feature vector, deliberately discarding spatial layout to obtain a position-tolerant texture descriptor for material classification. A CNN applies convolution locally *with stride*, reducing resolution while approximately preserving the spatial arrangement, which is what allows dense prediction tasks (e.g. labelling every pixel) rather than a single per-image label.

4. **Training.** Classical pipelines train the classifier *after* fixing the features, and the classifier may simply be nearest-neighbour or nearest-class-mean over texture vectors. A CNN learns kernels and classifier *jointly* through one end-to-end objective.

Why joint training matters: because the kernels are optimised for the same objective as the classifier, the network can shape its features specifically to make the downstream decision easy, rather than relying on whether a hand-designed bank happens to capture the discriminating structure. It also removes the burden of manually selecting orientations, frequencies and scales, and lets early layers adapt to the statistics of the actual dataset. The empirical echo of the hand-designed approach is that early layers often converge to Gabor-like oriented filters anyway.
<!-- explanation:end -->

## q9 [medium]

A filter bank has K = 10 kernels with different orientations and frequencies. You want both to determine "at what scale" a structure exists and to describe the scene using many scales. Which of the following correctly pairs the two uses of multi-scale responses?

A. Scale selection keeps all scales; a multiscale representation picks a local maximum across scales.
B. Scale selection picks a local maximum of response across scales; a multiscale representation concatenates responses from several scales into one feature vector.
C. Both methods average responses across scales into a single scalar.
D. Scale selection requires a fixed σ; a multiscale representation requires varying the image size only.

<!-- answer:start -->
exact
B
<!-- answer:end -->
<!-- explanation:start -->
Scale selection measures a response across a range of scales and takes its local maximum: for a scale-normalised detector the response peaks when the filter scale matches the characteristic size of the local structure. A multiscale representation instead keeps the responses from several scales, stacking them into a longer feature vector (r with components from σ₁, σ₂, …) so that structures of different sizes are all captured — useful when the scene genuinely contains multiple scales. These are complementary strategies, not the same operation.
<!-- explanation:end -->
