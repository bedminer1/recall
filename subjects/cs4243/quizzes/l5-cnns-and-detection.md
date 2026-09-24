# CS4243 L5 — CNNs & Object Detection

Built from the Lecture 5 review and in-lecture questions plus the midterm-style parameter arithmetic. Covers convolution parameter counts, softmax/logits, equivariance, and the detector family.

## q1 [easy]

A classifier produces logits z = (2.0, 1.0, −1.0). Suppose the same constant 100 is added to every logit. What changes?

A. The predicted class changes.
B. The softmax probabilities change, but the predicted class does not.
C. Neither the softmax probabilities nor the predicted class changes.
D. The logits become probabilities.

<!-- answer:start -->
exact
C
<!-- answer:end -->
<!-- explanation:start -->
Softmax depends only on *differences* between logits. Adding a constant c to all logits factors out: p′ₖ = e^(zₖ+c) / Σⱼ e^(zⱼ+c) = e^c e^(zₖ) / (e^c Σⱼ e^(zⱼ)) = pₖ. So the probability distribution is literally unchanged, and therefore the argmax is unchanged too. This is why logits are *relative* class scores rather than absolute probabilities. (D) is wrong: logits are unrestricted real scores; softmax is what converts them to probabilities.
<!-- explanation:end -->

## q2 [easy]

An object is translated to a different location in the image. What behaviour would we ideally expect from a CNN classifier?

A. Every intermediate feature map should remain identical.
B. Intermediate feature maps should translate, while the final class prediction remains approximately unchanged.
C. Intermediate feature maps should remain unchanged, while the class prediction translates.
D. Both the feature maps and class prediction must change.

<!-- answer:start -->
exact
B
<!-- answer:end -->
<!-- explanation:start -->
Convolution with shared weights is approximately **translation equivariant**: shifting the input shifts the response maps by the same amount. Spatial aggregation (pooling, global aggregation) and the classification head then collapse that spatial information into a single label, producing an approximately **translation invariant** prediction. Both halves are needed: equivariance in the features, invariance in the final decision. In practice invariance is imperfect, since stride, padding and image boundaries break exact equivariance.
<!-- explanation:end -->

## q3 [medium]

Object detection commonly uses an IoU threshold during both non-maximum suppression and evaluation. What is the important difference?

A. NMS compares predictions with ground truth; evaluation compares predictions with each other.
B. Both thresholds compare predictions with ground truth.
C. NMS compares predictions with each other; evaluation compares predictions with ground truth.
D. Both thresholds perform exactly the same operation.

<!-- answer:start -->
exact
C
<!-- answer:end -->
<!-- explanation:start -->
During **NMS**, the overlap is measured between two *predicted* boxes — nearby candidates usually describe the same object, so the strong prediction is kept and sufficiently overlapping alternatives are suppressed. During **evaluation**, the overlap is measured between a *prediction* and a *ground-truth* box: a detection counts as a true positive only if IoU ≥ t (typically t = 0.5). Same quantity, different operands. (A) is the reversed version of the correct answer.
<!-- explanation:end -->

## q4 [medium]

Which description of the detector lineage is correct?

A. R-CNN processes the image once, while Fast R-CNN processes every proposal separately.
B. Fast R-CNN shares convolutional computation but still depends on externally generated proposals; Faster R-CNN learns the proposals with a region proposal network.
C. Faster R-CNN is a one-stage detector because its proposals are learned.
D. YOLO first generates region proposals and then classifies each proposal separately.

<!-- answer:start -->
exact
B
<!-- answer:end -->
<!-- explanation:start -->
The progression is: **R-CNN** runs a CNN separately on each of ~2000 proposed regions (very slow); **Fast R-CNN** computes one shared image-level feature map in a single pass but still relies on *external* proposals; **Faster R-CNN** replaces those with a learned **region proposal network (RPN)** inside the network; **YOLO** abandons proposals altogether and makes dense class and box predictions in a **one-stage** architecture. Faster R-CNN is still two-stage — learning the proposals does not make it one-stage.
<!-- explanation:end -->

## q5 [medium]

A fully connected classification head sits on a feature map of spatial size 2×2 with 10 input channels. If you double **both** spatial dimensions to 4×4 (keeping the 10 channels and the same number of output classes), what happens to the number of weights in that fully connected layer?

A. It is unchanged.
B. It doubles.
C. It quadruples.
D. It increases by a factor of ten.

<!-- answer:start -->
exact
C
<!-- answer:end -->
<!-- explanation:start -->
A fully connected layer has one weight per input–output connection, and its input is the *flattened* feature map. The flattened dimension is 2·2·10 = 40, which doubles both ways to 4·4·10 = 160 — four times as large. Since the weight matrix is (outputs × flattened inputs), the weight count scales with the flattened input dimension, which quadruples when both spatial dimensions double. This is precisely why fully connected heads are parameter-hungry while convolutions are not.
<!-- explanation:end -->

## q6 [medium]

Why does doubling the spatial size of the input **feature map** not change the number of parameters in a convolutional layer, even though there are now four times as many output activations?

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
Because of **parameter sharing**. The layer's parameters are just the kernel weights (plus one bias per filter): for a bank of c_o filters each of size k_h × k_w over c_i input channels, the parameter count is c_o · c_i · k_h · k_w + c_o — it depends on kernel size and channel counts only, never on the image's height and width. The *same* filter is slid across every spatial location, so increasing the image size adds more positions at which the same weights are reused, not more weights.

What does grow is the *computation* and the *size of the activation maps*: the number of output locations increases (roughly quadruples when both dimensions double), so the cost O(c_o · c_i · k_h · k_w · o_h · o_w) grows. This is the key efficiency win of convolution over a fully connected layer, and it rests on the stationarity assumption that the same local pattern detector is useful everywhere in the image.
<!-- explanation:end -->

## q7 [easy]

How many learnable parameters does a fully connected network with 3 inputs, 4 hidden units, and 2 outputs have (including biases)? Give the number only.

<!-- answer:start -->
numeric 0
26
<!-- answer:end -->
<!-- explanation:start -->
Weights: between input and hidden there are 3 × 4 = 12; between hidden and output there are 4 × 2 = 8. Total weights = 20. Biases: one per unit in the hidden and output layers = 4 + 2 = 6. Learnable parameters = 20 + 6 = 26. (The number of *neurons* is 4 + 2 = 6, which is a common distractor — neurons are not parameters.)
<!-- explanation:end -->

## q8 [medium]

Why is a multi-layer perceptron (MLP) applied directly to raw image pixels a poor choice for vision?

A. It cannot represent non-linear functions.
B. Flattening the image destroys spatial neighbourhood structure, and the weight count grows with image size.
C. It cannot be trained by gradient descent.
D. It has fewer parameters than a convolutional network and therefore underfits.

<!-- answer:start -->
exact
B
<!-- answer:end -->
<!-- explanation:start -->
Applying an MLP to an image requires flattening x ∈ ℝ^(H×W×C) into a vector, which discards the fact that nearby pixels are correlated — the very structure that makes vision tractable. Worse, every hidden unit connects to every pixel, so the weight count explodes: a 224×224×3 input with 1000 hidden units needs 224·224·3·1000 = 150,528,000 weights. Convolution instead exploits locality and parameter sharing. MLPs are perfectly capable of non-linearity and gradient training, and they do not underfit for lack of parameters.
<!-- explanation:end -->

## q9 [hard]

Starting from a fully connected layer on a 200×200 image with 30,000 hidden units, explain how the parameter count drops to roughly 10,000, naming the two ideas responsible and the order in which they reduce the count.

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
**Step 0 — fully connected.** Each of the 30,000 hidden units connects to every pixel: (200 × 200) × 30,000 + 30,000 ≈ **1.2 billion** parameters. The problem is that fully connected units ignore the image property that nearby pixels are more correlated than distant ones.

**Step 1 — locality (local receptive fields).** Let each hidden unit look at only a 10×10 neighbourhood instead of the whole image: (10 × 10) × 30,000 = **3 million** parameters. This is a 400× reduction and encodes the prior that useful structure is local. Composition of layers later expands this local view to a global one, since the receptive field grows with depth.

**Step 2 — parameter sharing (weight reuse).** Rather than giving each of the 30,000 units its own 10×10 weights, use the *same* 10×10 kernel at every spatial location. A single kernel has only **100 parameters** (10×10). This is convolution with a learned kernel.

**Step 3 — multiple filters.** One filter produces one feature map, which is too few; use 100 filters: (10 × 10) × 100 = **10,000 parameters**. Each filter acts as a local perceptron producing its own response map.

So the reduction is 1.2 billion → 3 million (locality) → 10,000 (sharing, then choosing a bank of 100 filters). The stationarity assumption behind sharing is what yields an equivariant representation; the price paid is that the layer can only express translation-equivariant, locally-computed features.
<!-- explanation:end -->

## q10 [medium]

Which statement about what is learned versus fixed in a CNN is correct?

A. Both the convolution kernels and the ReLU operation are learned.
B. The convolution kernels are learned; ReLU and standard max pooling are fixed operations chosen by the designer.
C. The convolution kernels are fixed; only the classifier is learned.
D. Max pooling is learned while ReLU is fixed.

<!-- answer:start -->
exact
B
<!-- answer:end -->
<!-- explanation:start -->
The **kernel weights are learned** from data (a convolutional layer is a learned filter bank). The **non-linearity** — ReLU, a = max(0, r) — and the **pooling rule** (max or average) are fixed operations with no learned parameters; their window size and stride are architectural choices. In a traditional pipeline the feature extractor was hand-designed and only the classifier was learned; deep learning learns the feature extractor and classifier jointly end-to-end. (Note one nuance: *strided convolution* is a learned alternative to fixed pooling, because it learns how to combine values.)
<!-- explanation:end -->

## q11 [hard]

A colleague says: "My CNN is translation invariant because it uses convolution." Identify precisely what is wrong with this claim and describe where invariance actually comes from.

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
The claim conflates **equivariance** with **invariance**. Convolution with shared parameters gives *equivariance*: if the input shifts, the feature maps shift correspondingly. Equivariance means "changes in a predictable, corresponding way" — it is not invariance, which means "output does not change at all." A stack of pure convolutions would still produce a feature map that moves when the object moves, so the representation itself is not invariant.

Invariance comes from operations that *discard* spatial information: pooling (especially global pooling/average over the whole map), global aggregation, and the classification head that reduces a spatial map to one label. Those operations summarise the presence of features without regard to exactly where they occurred. So the correct statement is: convolution gives approximately translation-equivariant features, and pooling/aggregation plus the head give an approximately translation-invariant class prediction.

Even that invariance is imperfect in practice: stride, padding, and image-boundary effects break exact equivariance, and the effective receptive field is concentrated near its centre, so the same object near an image border may be treated differently. Pooling also gives only *local* tolerance to small shifts, not global invariance, unless it is global.
<!-- explanation:end -->

## q12 [hard]

You must evaluate a detector on a crowded street scene. Define IoU, explain how NMS uses it, and explain why NMS can destroy correct detections in this scene. Then state what precision and recall measure and why accuracy is not used.

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
**IoU (intersection over union)** measures box overlap: IoU(A, B) = |A ∩ B| / |A ∪ B| — the area of intersection divided by the area of the union, ranging from 0 (disjoint) to 1 (identical). Using areas rather than, say, centre distance makes it sensitive to both localisation and size mismatch.

**NMS use.** Detectors produce many overlapping candidate boxes for the same object. NMS sorts candidates by score, keeps the highest-scoring box, and suppresses any remaining box whose IoU with a kept box exceeds a threshold. This collapses duplicate detections of one object into a single prediction. (The same IoU quantity, but computed against ground truth rather than against another prediction, is used at evaluation time to decide whether a detection counts as a true positive at threshold t, usually t = 0.5.)

**Why NMS fails in a crowd.** NMS only knows geometry, not identity. In a crowded scene, two *genuinely different* nearby objects — say two people standing shoulder to shoulder — can have predicted boxes whose mutual IoU exceeds the suppression threshold. NMS will then suppress the lower-scoring box, deleting a correct detection and producing a false negative. Methods such as softer NMS, or learned/end-to-end NMS-free heads, are motivated by this failure.

**Precision and recall.** Precision answers "of all the objects I predicted, how many were correct?" = TP / (TP + FP). Recall answers "of all the real objects, how many did I find?" = TP / (TP + FN). A true positive is a detection matched to a ground-truth box with IoU ≥ t (one-to-one matching, so extra boxes on the same object count as false positives); a false negative is an un-detected ground-truth object.

**Why not accuracy.** Accuracy needs true negatives, but in detection the number of possible background boxes is enormous and not uniquely defined — there is no meaningful count of "correctly rejected boxes." Precision and recall (aggregated into AP, and averaged across classes as mAP) are therefore the standard measures. Note also that mAP is only comparable under the same protocol, since it depends on the IoU threshold(s) and class averaging used.
<!-- explanation:end -->
