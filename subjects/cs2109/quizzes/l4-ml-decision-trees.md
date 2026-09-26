# Supervised Learning and Decision Trees — MCQ

Reply with the option letter only.

## q1 [easy]

What distinguishes supervised learning?

A. It learns from labeled input-target examples.  
B. It never uses data.  
C. It can only cluster unlabeled examples.  
D. It requires a game tree.

<!-- answer:start -->
exact
A
<!-- answer:end -->
<!-- explanation:start -->
A is correct. C describes unsupervised clustering; B and D are unrelated restrictions.
<!-- explanation:end -->

## q2 [medium]

In house-price prediction, which identification is correct?

A. Features and prices are data; the model predicts price; MSE can be the loss; learning adjusts parameters.  
B. Price is an actuator and MSE is a sensor.  
C. The data are the learning algorithm.  
D. The model is the collection of correct labels only.

<!-- answer:start -->
exact
A
<!-- answer:end -->
<!-- explanation:start -->
A correctly separates data, model, loss, and learning algorithm. B mixes agent terms; C and D collapse distinct ML components.
<!-- explanation:end -->

## q3 [easy]

Predicting a continuous house price is what kind of task?

A. Classification  
B. Regression  
C. Clustering  
D. Adversarial search

<!-- answer:start -->
exact
B
<!-- answer:end -->
<!-- explanation:start -->
B is correct because the target is continuous. Classification predicts categories, clustering is unsupervised grouping, and adversarial search is unrelated.
<!-- explanation:end -->

## q4 [medium]

Which statement best distinguishes loss from performance measure?

A. Loss guides optimization; a performance measure evaluates task quality, though one formula may serve both roles.  
B. Loss applies only to classification.  
C. Performance measures are always differentiable.  
D. They can never use the same formula.

<!-- answer:start -->
exact
A
<!-- answer:end -->
<!-- explanation:start -->
A is correct. B and C impose false restrictions; D ignores examples such as MSE serving both roles.
<!-- explanation:end -->

## q5 [easy]

What do decision-tree internal nodes, branches, and leaves represent?

A. Predictions, datasets, and losses  
B. Feature tests, test outcomes, and predictions  
C. Gradients, weights, and learning rates  
D. States, actions, and path costs

<!-- answer:start -->
exact
B
<!-- answer:end -->
<!-- explanation:start -->
B is correct. A, C, and D borrow terminology from other ML or search components.
<!-- explanation:end -->

## q6 [hard]

A binary node has 8 positive and 8 negative examples. Its entropy is:

A. 0 bits  
B. 0.5 bits  
C. 1 bit  
D. 16 bits

<!-- answer:start -->
exact
C
<!-- answer:end -->
<!-- explanation:start -->
C is correct: −2(0.5 log2 0.5)=1. A corresponds to a pure node; B and D do not follow from the entropy formula.
<!-- explanation:end -->

## q7 [hard]

How does greedy decision-tree learning select a split?

A. It minimizes H(Y) before seeing any attribute.  
B. It chooses the attribute with largest H(Y|X).  
C. It chooses the attribute with maximum information gain H(Y)−H(Y|X).  
D. It always chooses the first attribute.

<!-- answer:start -->
exact
C
<!-- answer:end -->
<!-- explanation:start -->
C is correct because it maximizes uncertainty reduction. B prefers remaining uncertainty, A cannot distinguish attributes, and D ignores the criterion.
<!-- explanation:end -->

## q8 [hard]

Why can max depth or minimum samples per leaf improve a decision tree?

A. They prevent overly specific branches that fit training noise.  
B. They guarantee zero training error.  
C. They convert regression into classification.  
D. They maximize tree size.

<!-- answer:start -->
exact
A
<!-- answer:end -->
<!-- explanation:start -->
A is correct: pre-pruning controls complexity and overfitting. B often conflicts with pruning, while C and D misstate its purpose.
<!-- explanation:end -->
