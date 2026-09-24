# CS4243 L6 — Image Segmentation

Clustering-based segmentation (k-means, SLIC superpixels), dense prediction with encoder–decoders, and segmentation metrics. Note: mean-shift is **not** part of this year's syllabus and is deliberately excluded.

## q1 [easy]

What distinguishes **semantic** segmentation from **instance** segmentation?

A. Semantic segmentation labels each pixel with a class; instance segmentation gives each object its own separate mask.
B. Semantic segmentation predicts bounding boxes; instance segmentation predicts masks.
C. Semantic segmentation is learned; instance segmentation is hand-designed.
D. Semantic segmentation works on video only.

<!-- answer:start -->
exact
A
<!-- answer:end -->
<!-- explanation:start -->
Semantic segmentation assigns a shared class label to every pixel, so all pixels of the "dog" class carry the same label with no notion of *which* dog. Instance segmentation additionally separates individual objects, predicting a distinct mask per detected instance. This is why a semantic model can label every dog pixel correctly yet still be unable to count the dogs — touching dogs merge into one region and an occluded dog may split into disconnected ones.
<!-- explanation:end -->

## q2 [medium]

A ground-truth object occupies 100 pixels. Prediction A selects 50 pixels, all inside the object. Prediction B selects 200 pixels, including all 100 object pixels. Which statement about their foreground IoU is correct?

A. Both score 0.50, although A misses foreground and B includes extra background.
B. A scores 0.50 and B scores 1.00, because B finds the entire object.
C. A scores 1.00 and B scores 0.50, because every pixel selected by A is correct.
D. Both score 0.67, because IoU counts the overlap twice.

<!-- answer:start -->
exact
A
<!-- answer:end -->
<!-- explanation:start -->
IoU = |A ∩ B| / |A ∪ B|. For A: intersection = 50, union = 50 selected + 100 true = 100 (since A ⊂ object), so IoU = 50/100 = 0.50. For B: intersection = 100, union = 200, so IoU = 100/200 = 0.50. Identical scores despite opposite errors — A has perfect precision but misses foreground, B has perfect recall but adds false positives. This is the key lesson that equal IoU can hide different failure modes, so masks should be inspected as well as scores. (Option D confuses IoU with Dice: counting the overlap twice is the Dice formula.)
<!-- explanation:end -->

## q3 [medium]

In SLIC, what does the compactness parameter m control?

A. The number of superpixels requested.
B. The relative weight of spatial proximity versus colour similarity.
C. The number of iterations before convergence.
D. The colour space used for clustering.

<!-- answer:start -->
exact
B
<!-- answer:end -->
<!-- explanation:start -->
SLIC's composite distance is D = d_c + (m/s)·d_s, where d_c is colour (L2) distance and d_s is spatial distance, with s the initial grid spacing. The parameter m therefore sets the trade-off: **larger m** weights spatial proximity more, giving more regular, compact superpixels that may cut across colour boundaries; **smaller m** weights colour more, so boundaries follow appearance more closely at the cost of regular shape. The number of superpixels is a separate hyperparameter, not controlled by m.
<!-- explanation:end -->

## q4 [medium]

A 512 × 384 image is to be segmented into 100 superpixels with SLIC. What is the approximate initial grid spacing s, and roughly how large is the search window used for each cluster centre? Give s rounded to the nearest integer and the window size.

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
Total pixels n_tp = 512 × 384 = 196,608. The grid spacing is s = √(n_tp / n_sp) = √(196,608 / 100) = √1966.08 ≈ 44. So the centres are initialised on a grid with roughly 44-pixel spacing: 512/44 ≈ 12 and 384/44 ≈ 9, giving a 12 × 9 grid of centres (consistent with the requested 100 superpixels). Each centre searches only within a **2s × 2s = 88 × 88** region rather than the whole image. This localised search is exactly what fixes k-means' global distance evaluation and makes SLIC fast.
<!-- explanation:end -->

## q5 [medium]

Why does SLIC initialise its cluster centres on a regular grid and then nudge each one to the lowest-gradient position in its neighbourhood, instead of initialising randomly as k-means does?

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
Two distinct reasons. **Grid initialisation** enforces a spatially even distribution of superpixels: unlike k-means in colour space (where clusters need not be spatially coherent), SLIC wants roughly equal-sized compact regions tiling the image, so a grid is a far better starting point than random centres. **The lowest-gradient nudge** avoids seeding a centre on an edge or a noise point, since those have extreme values and make convergence slower. Initialising on a value common to the surrounding pixels (approximated by the smallest local gradient in the neighbourhood) means the centre already resembles its eventual cluster mean, so fewer iterations are needed.

Note the honest caveat: lowest gradient is a *local heuristic*, not knowledge of the true segment. It can in principle move a centre off a thin object into its background, but the search neighbourhood is small so the displacement is limited.
<!-- explanation:end -->

## q6 [hard]

You run SLIC twice on the same image with the same requested number of superpixels, but in the second run you substantially increase the compactness parameter m. Describe the change you expect and explain the underlying trade-off.

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
You should expect **more regularly shaped superpixels, which may follow object boundaries less closely** in the high-compactness run. Increasing m raises the weight on spatial proximity (the (m/s)·d_s term) relative to colour similarity d_c in the composite distance. The clustering therefore prefers assignments that keep regions spatially regular and compact, even when the colour evidence would favour a boundary that wanders.

The trade-off is spatial regularity versus boundary adherence: high m gives grid-like superpixels that can cross an object boundary where colour evidence is weak; low m gives superpixels that hug appearance boundaries but with less regular shapes and sizes. Note what does **not** happen: the number of superpixels is held fixed by the request (connectivity cleanup may perturb it slightly, but that is not compactness's role); increasing m does not make them follow colour boundaries more closely (that is the reverse); and superpixels never correspond to complete semantic objects, because SLIC groups by appearance and position, not semantics.
<!-- explanation:end -->

## q7 [hard]

A superpixel produced by SLIC contains only some of the pixels of a thin structure, and one isolated pixel remains assigned to a distant superpixel. Explain why SLIC permits this and what the standard remedy is.

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
SLIC provides **no mechanism to enforce connectivity** of its superpixels. Its assignment step simply gives each pixel to the nearest centre under the composite distance, and because the search is local but the objective is purely distance-based, it is possible for the label field to end up disconnected: a few pixels sharing a colour with a distant region can be assigned there, leaving **isolated pixels** after convergence. This is aggravated when the initial grid is nudged to low-gradient positions, since a centre can land somewhere that splits a thin structure.

The standard remedy is a **post-processing connected-components step**: find connected components of each superpixel's label mask, retain the largest component as the superpixel, and reassign the orphaned small components/isolated pixels to the neighbouring superpixel they touch. This is a cleanup pass, not part of the SLIC iteration itself.

It is also worth noting that the superpixel grid is only an over-segmentation useful for downstream processing — a superpixel is *not* an object or semantic category. If a superpixel straddles foreground and background, and a later classifier must assign one label per superpixel, that region's boundary cannot be repaired by the classifier; smaller superpixels reduce but never eliminate this limitation.
<!-- explanation:end -->

## q8 [medium]

Why can an upsampled segmentation prediction have the original image's dimensions yet still produce coarse, poorly localised boundaries?

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
Output *resolution* is not the same as retained *detail*. An encoder downsamples through pooling/strided convolution, which reduces spatial precision and destroys fine boundary information. If the decoder merely upsamples that coarse feature map (by nearest-neighbour or bilinear interpolation, or by a transposed convolution), it creates more output samples but cannot recreate the boundary detail that was lost — every output pixel is derived from a coarse representation, so the predicted boundary is smooth and imprecise.

This is precisely why **skip connections** are needed: they copy spatially aligned high-resolution encoder features into the decoder, supplying the missing fine-grained evidence (thin structures, exact edge locations) so boundaries can be localized rather than merely interpolated. Features from deeper layers have larger receptive fields but lower resolution, so combining them with shallow features gives both semantic context and spatial precision.
<!-- explanation:end -->

## q9 [hard]

An encoder–decoder segmentation network combines high-resolution encoder features with decoder features through skip connections. You randomly **shuffle the spatial positions** of the encoder features before combining them, preserving their values and tensor dimensions. What is the most likely consequence, and why?

A. Nothing changes, because the same feature values are still available.
B. The output segmentation becomes smaller because spatial information has been removed.
C. Boundary localisation becomes less reliable because encoder and decoder features no longer correspond to the same locations.
D. The network can no longer produce a class prediction for every pixel.

<!-- answer:start -->
exact
C
<!-- answer:end -->
<!-- explanation:start -->
Skip connections supply spatially *aligned* detail: the decoder combines its own coarse, semantically rich features with encoder features from the *same* image location. Shuffling preserves values and dimensions but destroys that correspondence, so a decoder location may receive encoder evidence from an unrelated part of the image — boundary localisation degrades. (A) is wrong because in dense prediction, *where* a feature came from is part of the information; (B) is wrong because tensor dimensions are untouched; (D) is wrong because the network still emits a prediction at every output location, just a less accurate one.
<!-- explanation:end -->

## q10 [medium]

How is a transposed convolution with fractional stride conceptually implemented, and what does a stride of 1/s correspond to?

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
By **inserting zeros between the input elements and then applying an ordinary convolution**. Increasing the stride shrinks the output (stride > 1) and stride 1 preserves it (with "same" padding), so a stride below 1 grows the output — a **fractionally strided convolution**. Concretely, a stride of 1/s is equivalent to inserting **s − 1 zeros** between adjacent samples in each spatial dimension, which expands the grid, after which a normal convolution produces the finer output.

The name "transposed convolution" comes from the implementation (the operation is the transpose of the matrix form of a convolution), which is also why the *same* stride can yield different output sizes in different sources: the final size also depends on kernel size, padding, and any output-padding convention. Stride alone is not sufficient to determine it. In U-Net-style decoders this learned upsampling is often called "upconv"/"deconv".
<!-- explanation:end -->

## q11 [hard]

A model reports Dice = 0.80 on a segmentation task; a different model reports IoU = 0.70. Explain why you cannot conclude the second model is worse, and derive the relationship for a single binary-mask comparison.

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
The two numbers are not on the same scale, so they cannot be compared directly. For a single binary mask, let I = |A ∩ B| be the intersection and let the ground truth have |A| pixels and the prediction |B|. Then

- Dice = 2I / (|A| + |B|)
- IoU = I / |A ∪ B| = I / (|A| + |B| − I)

Writing IoU = I/U with U = |A| + |B| − I, and Dice = 2I/(U + I), we can eliminate I/U to get

**IoU = Dice / (2 − Dice)** and conversely **Dice = 2·IoU / (1 + IoU)**.

Dice and IoU always rank predictions identically (the map is monotone increasing), but **Dice gives numerically larger values than IoU**. Checking the example: Dice 0.80 → IoU = 0.80/(2 − 0.80) = 0.80/1.20 = 0.667 ≈ 0.67. So the first model's Dice 0.80 corresponds to about IoU 0.67 — essentially a tie with the second model's IoU 0.70, not a loss.

For dataset-level numbers you must be even more careful: the conversion is nonlinear, so the mean of per-image Dice is *not* the Dice of the mean IoU. Averages depend on the aggregation protocol (pooling all pixels versus averaging per image or per object), which is why models should only be compared under the same metric, thresholds and aggregation.
<!-- explanation:end -->

## q12 [hard]

A colleague reports 98% pixel-wise accuracy on a road-segmentation dataset and claims the model is nearly solved. Explain why this may be misleading, name a better metric, and explain why the loss function for training usually differs from the metric used for evaluation.

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
**Why accuracy misleads.** Pixel-wise accuracy is dominated by whichever class covers the most pixels. If roads occupy only 2% of the image, a model that predicts "background" everywhere scores 98% accuracy while detecting *nothing* of interest. The metric is inflated by the trivial majority class, so a high value can coexist with complete failure on the foreground. This is exactly why overlap-based measures are preferred: they prevent the background from dominating.

**Better metric.** Report **IoU** (|A ∩ B| / |A ∪ B|) or the **Dice coefficient** (2|A ∩ B| / (|A| + |B|)) over the foreground class, per class and averaged across classes, plus **precision and recall**. Precision/recall expose the two different error types: low precision means many false positives (over-segmenting road), low recall means many false negatives (missing road). Also be aware that aggregation matters — ten missed pixels is most of a tiny object but negligible for a large one — so per-object averaging can differ greatly from pooling all pixels.

**Why the training loss differs.** Training needs a smooth, differentiable signal that can be optimised by gradient descent. Pixel-averaged cross-entropy provides this and softly penalises low confidence in the correct class, but it too is dominated by the many background pixels. Alternative *soft* overlap losses (soft Dice) directly encourage the predicted region to overlap the ground truth and are differentiable when computed on predicted probabilities. Evaluation, by contrast, is done on **thresholded hard masks** with IoU/Dice. The two can legitimately disagree: probabilities can improve for several epochs (lower loss) without crossing the decision threshold, leaving the hard-mask Dice unchanged — that is not necessarily a bug, but persistent disagreement is a reason to inspect the predictions.
<!-- explanation:end -->
