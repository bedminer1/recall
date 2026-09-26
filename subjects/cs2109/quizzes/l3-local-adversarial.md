# Local and Adversarial Search — MCQ

Reply with the option letter only.

## q1 [easy]

When is local search especially attractive?

A. The path itself is required and the state space is tiny.  
B. The state space is huge, only the final configuration matters, and good-enough is acceptable.  
C. Every state must be stored.  
D. Only shortest-path problems are allowed.

<!-- answer:start -->
exact
B
<!-- answer:end -->
<!-- explanation:start -->
B is correct. Local search focuses on configurations with little memory. A requires path search, while C and D falsely restrict it.
<!-- explanation:end -->

## q2 [medium]

What does basic hill climbing do?

A. Selects the best neighbor repeatedly and stops when none improves the objective.  
B. Explores every state breadth-first.  
C. Always accepts a worse neighbor.  
D. Builds a minimax tree.

<!-- answer:start -->
exact
A
<!-- answer:end -->
<!-- explanation:start -->
A is correct. B is BFS, C is unlike basic hill climbing, and D is adversarial search.
<!-- explanation:end -->

## q3 [hard]

Why can hill climbing stop before a global optimum?

A. It knows the entire landscape and deliberately avoids the optimum.  
B. A local maximum or plateau may offer no improving immediate neighbor.  
C. It always uses an inadmissible heuristic.  
D. It expands nodes in FIFO order.

<!-- answer:start -->
exact
B
<!-- answer:end -->
<!-- explanation:start -->
B is correct: local information can provide no uphill direction. A assumes global knowledge, C misuses A* terminology, and D describes BFS.
<!-- explanation:end -->

## q4 [medium]

Which assumptions define the adversarial games emphasized in the lecture?

A. Single-player, episodic, cooperative  
B. Two-player, turn-taking, zero-sum  
C. Multi-player, simultaneous, noncompetitive  
D. Continuous-action, stochastic, single-agent

<!-- answer:start -->
exact
B
<!-- answer:end -->
<!-- explanation:start -->
B is correct. The other choices change the number of players, move structure, or competitive relationship.
<!-- explanation:end -->

## q5 [medium]

How does minimax back up values?

A. MAX takes a minimum and MIN takes a maximum.  
B. Both take averages.  
C. MAX takes a maximum and MIN takes a minimum.  
D. Both take the first child.

<!-- answer:start -->
exact
C
<!-- answer:end -->
<!-- explanation:start -->
C is correct because each player chooses its preferred outcome. A reverses roles; B and D do not model rational adversarial choices.
<!-- explanation:end -->

## q6 [hard]

A MAX node has terminal children −2, 4, and 1. What value is backed up?

A. −2  
B. 1  
C. 3  
D. 4

<!-- answer:start -->
exact
D
<!-- answer:end -->
<!-- explanation:start -->
D is correct because MAX chooses max(−2,4,1)=4. A is MIN's choice, B is merely one child, and C incorrectly combines values.
<!-- explanation:end -->

## q7 [hard]

Which statement about alpha-beta pruning is correct?

A. Alpha is MAX's best guaranteed value, beta is MIN's, and pruning is possible when alpha≥beta.  
B. Alpha is depth, beta is branching factor, and pruning occurs when equal.  
C. It changes the optimal minimax move.  
D. It works best when the worst moves are examined first.

<!-- answer:start -->
exact
A
<!-- answer:end -->
<!-- explanation:start -->
A is correct. B invents meanings, C violates alpha-beta equivalence to minimax, and D misses that good move ordering tightens bounds sooner.
<!-- explanation:end -->

## q8 [hard]

Why use a cutoff and evaluation function in a large game tree?

A. To turn the game into BFS.  
B. To stop at manageable depth and estimate cutoff-state utility.  
C. To guarantee the estimate is admissible.  
D. To remove the opponent from the model.

<!-- answer:start -->
exact
B
<!-- answer:end -->
<!-- explanation:start -->
B is correct because full terminal search is often infeasible. A and D change the problem, while A*-style admissibility in C is not the applicable guarantee.
<!-- explanation:end -->
