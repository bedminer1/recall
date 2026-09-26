# Linear Regression — MCQ

Reply with the option letter only.

## q1 [easy]

Which is the d-dimensional linear model with a bias convention?

A. h(x)=w^T x with x0=1  
B. h(x)=x^T x with w0=0  
C. h(x)=log(x) with no weights  
D. h(x)=argmax x

<!-- answer:start -->
exact
A
<!-- answer:end -->
<!-- explanation:start -->
A is correct: w0 multiplies constant x0=1 and acts as the intercept. B–D are not the stated linear model.
<!-- explanation:end -->

## q2 [medium]

For h(x)=2+3x, what is h(4)?

A. 9  
B. 11  
C. 14  
D. 20

<!-- answer:start -->
exact
C
<!-- answer:end -->
<!-- explanation:start -->
C is correct: 2+3(4)=14. The other values result from incorrect substitution or arithmetic.
<!-- explanation:end -->

## q3 [medium]

Which expression is mean squared error over m examples?

A. (1/m) Σ(h(xᶦ)−yᶦ)²  
B. Σ|w| only  
C. maxᵢ h(xᶦ)  
D. (1/m) Σ(h(xᶦ)+yᶦ)

<!-- answer:start -->
exact
A
<!-- answer:end -->
<!-- explanation:start -->
A averages squared residuals. B is weight magnitude, C is a maximum prediction, and D adds rather than measures prediction error.
<!-- explanation:end -->

## q4 [hard]

Which normal-equation statement is correct?

A. w=(X^TX)⁻¹X^Ty, with matrix inversion costing about O(d³).  
B. w=X+y, with cost O(1).  
C. It requires choosing a learning rate.  
D. It is always faster than gradient descent for huge d.

<!-- answer:start -->
exact
A
<!-- answer:end -->
<!-- explanation:start -->
A is correct when the inverse exists. B is not the solution, C confuses it with gradient descent, and D ignores cubic feature-dimension cost.
<!-- explanation:end -->

## q5 [medium]

Why does gradient descent update w←w−α∇J(w)?

A. The gradient points toward steepest increase, so subtracting it lowers loss.  
B. The gradient always equals zero.  
C. Adding the gradient lowers every convex function.  
D. α is the model bias.

<!-- answer:start -->
exact
A
<!-- answer:end -->
<!-- explanation:start -->
A is correct. B holds only at stationary points, C uses the wrong direction, and D confuses learning rate with intercept.
<!-- explanation:end -->

## q6 [hard]

What usually happens with a learning rate that is far too large?

A. Convergence is guaranteed in one step.  
B. Updates can overshoot, oscillate, or diverge.  
C. The normal equation becomes exact.  
D. Features become normalized automatically.

<!-- answer:start -->
exact
B
<!-- answer:end -->
<!-- explanation:start -->
B is correct. A is not guaranteed, and C–D describe unrelated operations.
<!-- explanation:end -->

## q7 [hard]

Why can feature scaling speed gradient descent?

A. It makes differently scaled directions better conditioned and reduces zig-zagging.  
B. It removes all training examples.  
C. It makes the loss non-convex.  
D. It guarantees every feature has weight 1.

<!-- answer:start -->
exact
A
<!-- answer:end -->
<!-- explanation:start -->
A is correct. Scaling improves optimization geometry; it does not discard data, destroy convexity, or fix the learned weights.
<!-- explanation:end -->

## q8 [medium]

Which comparison is correct?

A. Batch uses all examples, mini-batch a subset, stochastic one example per update.  
B. Stochastic uses all examples and batch uses one.  
C. All three always produce identical updates.  
D. Mini-batch uses no data.

<!-- answer:start -->
exact
A
<!-- answer:end -->
<!-- explanation:start -->
A is correct. B reverses batch and stochastic, C ignores gradient noise, and D contradicts the definition.
<!-- explanation:end -->

## q9 [hard]

Why can polynomial features model a curve while still using linear regression?

A. The prediction is nonlinear in the weights.  
B. It remains a weighted sum, linear in the learned weights, of transformed features.  
C. Polynomial features remove all weights.  
D. The method becomes a decision tree.

<!-- answer:start -->
exact
B
<!-- answer:end -->
<!-- explanation:start -->
B is correct. The curve is nonlinear in original inputs but linear in parameters. A reverses that fact, and C–D describe different models.
<!-- explanation:end -->

## q10 [hard]

Why can gradient descent reach a global minimum for linear regression with MSE under the lecture's assumptions?

A. The objective is convex in the weights.  
B. Every starting weight is already optimal.  
C. MSE has no gradient.  
D. Feature transformation always makes it discrete.

<!-- answer:start -->
exact
A
<!-- answer:end -->
<!-- explanation:start -->
A is correct: local minima of a convex objective are global, with convergence requiring a suitable learning rate. B–D are false.
<!-- explanation:end -->
