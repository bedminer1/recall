# Search — MCQ

Reply with the option letter only.

## q1 [easy]

Which set gives the five components of a search problem?

A. Initial state, actions, transition model, goal test, path cost  
B. Sensors, actuators, utility, heuristic, queue  
C. Start node, visited set, stack, depth limit, runtime  
D. Agent, environment, loss, model, data

<!-- answer:start -->
exact
A
<!-- answer:end -->
<!-- explanation:start -->
A is correct. B and D mix other frameworks; C lists implementation choices rather than the abstract problem definition.
<!-- explanation:end -->

## q2 [medium]

Which matching is correct?

A. BFS—stack; DFS—FIFO; UCS—heuristic only  
B. BFS—FIFO; DFS—stack; UCS—priority by g(n)  
C. BFS—priority by g(n); DFS—FIFO; UCS—stack  
D. BFS—stack; DFS—priority by f(n); UCS—FIFO

<!-- answer:start -->
exact
B
<!-- answer:end -->
<!-- explanation:start -->
B is correct: BFS is layer-first, DFS is LIFO, and UCS expands the lowest accumulated cost. The other options mismatch these priorities.
<!-- explanation:end -->

## q3 [hard]

Why can BFS be cost-suboptimal when step costs differ?

A. It minimizes depth, not total path cost.  
B. It expands the most expensive edge first.  
C. It cannot find finite-depth goals.  
D. Its heuristic may overestimate.

<!-- answer:start -->
exact
A
<!-- answer:end -->
<!-- explanation:start -->
A is correct: fewest edges need not mean cheapest. B is not BFS's rule, C contradicts completeness, and D wrongly assumes BFS uses a heuristic.
<!-- explanation:end -->

## q4 [medium]

What is the main effect of visited memory?

A. It prevents repeated-state expansion and cycles but uses memory.  
B. It makes every heuristic admissible.  
C. It removes the frontier.  
D. It makes DFS cost-optimal.

<!-- answer:start -->
exact
A
<!-- answer:end -->
<!-- explanation:start -->
A is correct. Visited memory cannot change a heuristic, replace the frontier, or generally make DFS optimal, so B–D are false.
<!-- explanation:end -->

## q5 [medium]

What priority does A* use?

A. h(n) − g(n)  
B. g(n) + h(n)  
C. g(n) only  
D. depth plus branching factor

<!-- answer:start -->
exact
B
<!-- answer:end -->
<!-- explanation:start -->
B is correct: g is cost so far and h estimates cost remaining. C is UCS; A and D are not A* formulas.
<!-- explanation:end -->

## q6 [medium]

Which definitions are correct?

A. Admissible: h≥h*; consistent: h(n)≥c+h(n')  
B. Admissible: h≤h*; consistent: h(n)≤c+h(n')  
C. Admissible means h=0; consistent means h is constant.  
D. They mean exactly the same inequality.

<!-- answer:start -->
exact
B
<!-- answer:end -->
<!-- explanation:start -->
B gives non-overestimation and the triangle inequality. A reverses both, C is too restrictive, and D ignores the neighbor relation in consistency.
<!-- explanation:end -->

## q7 [hard]

Both h1 and h2 are admissible, with h2(n)≥h1(n) everywhere. Which is true?

A. h1 dominates because it is smaller.  
B. h2 dominates and is normally more informative.  
C. Neither works with A*.  
D. h2 must be inconsistent.

<!-- answer:start -->
exact
B
<!-- answer:end -->
<!-- explanation:start -->
B is correct: the pointwise larger admissible estimate is closer to true cost. A reverses dominance; C and D do not follow.
<!-- explanation:end -->

## q8 [hard]

Why does a relaxed problem yield an admissible heuristic?

A. Removing constraints cannot make its optimum costlier than the original optimum.  
B. It always has the identical solution path.  
C. Relaxation makes every edge free.  
D. Its optimum always overestimates.

<!-- answer:start -->
exact
A
<!-- answer:end -->
<!-- explanation:start -->
A makes the relaxed cost a lower bound. B and C are unnecessary, while D is the opposite of admissibility.
<!-- explanation:end -->

## q9 [medium]

Which comparison of DLS and IDS is correct?

A. DLS raises its limit repeatedly; IDS uses one limit.  
B. IDS runs DLS with limits 0,1,2,… and keeps DFS-like space.  
C. IDS is incomplete whenever BFS is complete.  
D. DLS is always optimal.

<!-- answer:start -->
exact
B
<!-- answer:end -->
<!-- explanation:start -->
B is correct. A reverses them, C contradicts IDS's BFS-like completeness conditions, and D ignores missed or nonoptimal goals.
<!-- explanation:end -->

## q10 [hard]

Which conditions suffice for A* optimality in the lecture?

A. Tree search: consistency; graph search: admissibility only  
B. Tree search: admissibility; graph search with visited memory: consistency  
C. Both require only dominance.  
D. Neither can be optimal.

<!-- answer:start -->
exact
B
<!-- answer:end -->
<!-- explanation:start -->
B is correct. A reverses the conditions; C and D discard the stated guarantees.
<!-- explanation:end -->
