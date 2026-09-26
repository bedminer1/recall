# AY2025/26 Semester 1 Midterm

Keep `CS2109S+AY2025-26+Sem+1+-+Midterm+-+Solution_v2.pdf` open for the shared context, tables, and figures. Answer choices are reproduced below where present.

## q1A [easy]

**Question 1A [1 mark]**

Is it true that the search formulation results in a state space where a state is

reachable through more than one sequence of actions? Note: we do not care about the
search algorithms in this question since we are asking about the state space, not the search
tree.
    A. Yes
    B. No

<!-- answer:start -->
exact
A
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** A. Yes

1A. A. Yes

The same state (same floor and same passenger statuses) can be reached via different
action sequences due to cycles (e.g., moving up then down vs. staying and opening at
different times), so multiple distinct paths can lead to the same state.
<!-- explanation:end -->

## q1B [easy]

**Question 1B [1 mark]**

Does the search formulation result in more than one goal states?

   A. Yes
   B. No

<!-- answer:start -->
exact
A
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** A. Yes

1B. A. Yes

A goal state requires all passengers delivered, but the elevator’s final floor is irrelevant.
Therefore, there are multiple goal states—one for each possible final floor (1–20).
<!-- explanation:end -->

## q1C [easy]

**Question 1C [1 mark]**

Is there always a solution when using the search formulation?

   A. Yes
   B. No

<!-- answer:start -->
exact
B
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** B. No

1C. B. No

In this problem, the certified capacity of the elevator can be zero, indicating that it cannot
carry any passengers, for instance, due to elevator maintenance.

If capacity C = 0 and there is at least one passenger (N > 0), no passenger can ever enter,
so no solution exists. Hence, not always solvable under the formulation.
<!-- explanation:end -->

## q1D [medium]

**Question 1D [2 marks]**

Suppose that you use search (without visited memory). Which search

algorithm(s) always terminate? Select all that apply.
    a. Queue-based search (i.e., BFS, UCS)
    b. Depth-First Search (DFS)
    c. None of the above

<!-- answer:start -->
exact
C
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** C. None of the above

1D. C. None of the above

Without visited memory, all these algorithms can revisit states indefinitely due to reversible
actions which cause cycles. BFS/UCS can keep generating longer paths to the same states
if there is no solution; DFS can get stuck down an infinite cycle. Therefore, none of the given
algorithms “always terminate” in general.
<!-- explanation:end -->

## q1E [medium]

**Question 1E [2 marks]**

Which of the following search (without visited memory) algorithm(s) can we

employ such that the search always finds an answer (valid solution) if a solution exists?
Select all that apply.
   a. Breadth-First Search (BFS)
   b. Depth-First Search (DFS)
   c. Uniform-Cost Search (UCS)
   d. Depth-Limited Search (DLS) with DFS and max-depth (40+C)*N/C

   e. None of the above

<!-- answer:start -->
exact
A
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** A. Breadth-First Search (BFS); c. Uniform-Cost Search (UCS)

1E. A. Breadth-First Search (BFS); c. Uniform-Cost Search (UCS)

The question addresses scenarios where a solution exists, indicating cases where C > 0.

Without visited memory,

   •   BFS is complete on finite branching with unit step cost and will find a solution if one
       exists because the number of sequences up to the solution depth is finite.
   •   UCS with positive action costs (here all 1) will also find a solution if one exists, as it
       explores increasing path cost and will eventually expand the optimal path.
   •   DFS is not complete without visited (can loop).
                                            !
   •   The proposed DLS bound (40 + 𝐶) ⋅ " is not sufficient in simple, allowed cases. Let
       the capacity be C=100, and N=1, then (40+100)/100= 1.4, which is not enough to
       deliver the 1 passenger.
<!-- explanation:end -->

## q1F [medium]

**Question 1F [2 marks]**

Suppose that we use search with visited memory. Which of the following

search algorithm(s) is/are the best for the problem? Select all that apply.
Best means the algorithm(s) should be complete, optimal, efficient (in terms of big O worst-
case space and time complexity), and aware if there is no solution.
    a. Depth-First Search (DFS)
    b. Uniform-Cost Search (UCS)
    c. Depth-Limited Search (DLS) with DFS and max-depth (20+C)*N/C+1
    d. Iterative Deepening Search (IDS) with DFS
    e. None of the above

<!-- answer:start -->
exact
D
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** D. Iterative Deepening Search (IDS) with DFS

1F. D. Iterative Deepening Search (IDS) with DFS

With visited memory:

   •   UCS is complete, optimal for positive costs, and will detect no-solution by exhausting
       reachable states. However, the space complexity is exponential.
   •   IDS with DFS is complete and optimal for unit step costs, and significantly more
       space-efficient (polynomial) while still detecting no-solution by exhausting depths
       across the finite state space.
   •   DFS alone is not optimal;
   •   DLS bound is not sufficient. See 1E for explanation. Also, when C=0, the bound is
       infinite using our definition of divide by 0.
<!-- explanation:end -->

## q2A [medium]

**Question 2A [2 marks]**

In addition to the setting described in the Context, the drone must perform a set of tasks.

These tasks are presented in a queue 𝑄 = [(𝑠𝑡𝑎𝑟𝑡_𝑖, 𝑒𝑛𝑑_𝑖)], which contains distinct 3D
coordinates representing the start and end points of each task. Once a task is complete by
reaching the end point, the task is removed from the queue.

The goal state is to complete all the tasks in sequence and return to the initial position.
Select admissible heuristics to use for the A* algorithm. Here, 𝐿 is the size of the current task
queue. In addition, the Manhattan distance is 𝑑'( (𝑎, 𝑏) = |𝑥% − 𝑥) | + |𝑦% − 𝑦) | + |𝑧% − 𝑧) |.
Select all that apply.

    A. 𝑑* ((𝑥, 𝑦, 𝑧), (𝑥#$%&$ , 𝑦#$%&$ , 1)).
    B. 𝐿
    C. The sum ∑,+-* 𝐷+ where 𝐷+ is 𝑑'( (𝑠𝑡𝑎𝑟𝑡_𝑖, 𝑒𝑛𝑑_𝑖) of the i-th element of the current
       queue.
    D. The sum ∑,+-. 𝐷+ where 𝐷+ is 𝑑'( (𝑠𝑡𝑎𝑟𝑡_𝑖, 𝑒𝑛𝑑_𝑖) of the i-th element of the current
       queue.
    E. None of the above.

<!-- answer:start -->
exact
ABD
A B D
A,B,D
A, B, D
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** ABD

Any obstacles only increase the true cost, because the drone must navigate around them.
For all questions consider the relaxed problem as well without any obstacles.

A is a solution because the goal state is returning to the initial state. At any current point
(𝑥, 𝑦, 𝑧), the drone must at least return to the initial state, hence the Manhattan distance from
the current state to the initial state is an admissible heuristic.

B: First, notice that the question says, “Once a task is complete by reaching the end point,
the task is removed from the queue.”
The drone must solve all the tasks, which consists of distinct 3D start and end coordinates.
Hence the cost of solving a single task is at least 1. Even if the drone is inside the
first/current task, to get to the end point and removing the task from the queue, takes at least
cost 1. Hence, L is an admissible heuristic.

C is not an admissible heuristic. Note that the queue element is only popped when the
endpoint is reached, as the question says. The drone could be inside task 1 and already
have accomplished most of that task. The task L could have an end point that is very close
(say, one move away) from the initial state. Hence, there are states where the heuristic
overcounts the cost to achieve the tasks and to return to the initial state, and the heuristic is
not admissible.

D is admissible as it removes the first task from estimating the cost (see discussion for
Option C).
<!-- explanation:end -->

## q2B [medium]

**Question 2B [2 marks]**

For this question, let the task queue be empty. In addition, the costs for each valid action at

any valid position (𝑥, 𝑦, 𝑧) are as follows:

•   Move left/right/forward/backward = 1.
•   Move up/down = 2.

Let the goal state be defined by reaching (𝑥* , 𝑦* , 𝑧* ) from the initial state. Select all heuristics
to use for the A* algorithm with visited memory such that it returns the optimal solution?
Select all that apply.

    A. ℎL(𝑥, 𝑦, 𝑧)M = 𝑥 + 𝑦 + 2𝑧.
    B. ℎ((𝑥, 𝑦, 𝑧)) = 𝑥 + 𝑦 + 𝑧 . .
    C. ℎ((𝑥, 𝑦, 𝑧)) = |𝑥 − 𝑥* | + |𝑦 − 𝑦* | + 2|𝑧 − 𝑧* |.
    D. ℎ((𝑥, 𝑦, 𝑧)) = |𝑥 − 𝑥* | + |𝑦 − 𝑦* | + |𝑧 − 𝑧* |.
    E. ℎ((𝑥, 𝑦, 𝑧)) = |𝑥 − 𝑥* | + |𝑦 − 𝑦* | + |𝑧 − 𝑧* |. .
    F. None of the above.

<!-- answer:start -->
exact
CD
C D
C,D
C, D
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** CD

2B. CD

A and B will overcount in cases when the current position is close to the goal state, hence
they are not admissible and hence not consistent, i.e., we cannot expect them to produce an
optimal solution in A* with visited memory.

For C and D, we check consistency by noticing that an up/down move changes |𝑧 − 𝑧* | by at
most 1, either making it smaller or larger. The same holds for the other coordinates. For 𝑧,
the move comes with a cost of 2, hence we always have that

                              ℎ((𝑥, 𝑦, 𝑧)) ≤ 2 + ℎL(𝑥, 𝑦, 𝑧 ± 1)M,
and a similar argument holds for the other moves.

For E, for 𝑧 = 1 and 𝑧* = 4, the term |𝑧 − 𝑧* |. = 9, while the cost of moving 3 steps is 6.
Hence, the heuristic is not admissible and cannot be consistent.
<!-- explanation:end -->

## q2C [medium]

**Question 2C [2 marks]**

For this question, let the task queue be empty. Consider the variable cost of

valid actions and the goal described in Question 2B. Select the best heuristics to use for the
A* algorithm with visited memory such that it returns the optimal solution? Select one that
applies.

    A. ℎL(𝑥, 𝑦, 𝑧)M = 𝑥 + 𝑦 + 2𝑧.
    B. ℎ((𝑥, 𝑦, 𝑧)) = 𝑥 + 𝑦 + 𝑧 . .
    C. ℎ((𝑥, 𝑦, 𝑧)) = |𝑥 − 𝑥* | + |𝑦 − 𝑦* | + 2|𝑧 − 𝑧* |.
    D. ℎ((𝑥, 𝑦, 𝑧)) = |𝑥 − 𝑥* | + |𝑦 − 𝑦* | + |𝑧 − 𝑧* |.
    E. ℎ((𝑥, 𝑦, 𝑧)) = |𝑥 − 𝑥* | + |𝑦 − 𝑦* | + |𝑧 − 𝑧* |. .
    F. None of the above.

<!-- answer:start -->
exact
C
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** C

2C. C

Choose the dominant heuristics among the consistent heuristics, which is C. It is always true
that |𝑥 − 𝑥* | + |𝑦 − 𝑦* | + |𝑧 − 𝑧* | ≤ |𝑥 − 𝑥* | + |𝑦 − 𝑦* | + 2|𝑧 − 𝑧* |.
<!-- explanation:end -->

## q2D [medium]

**Question 2D [2 marks]**

For this question, let the task queue be empty. Consider the variable cost of

valid actions as follows:

• Move left/right/forward/backward = 0.
• Move up/down = 2𝑧.
Let the goal state be defined by reaching (𝑥* , 𝑦* , 1). The initial state is any valid state. Select
all heuristics to use for the A* algorithm without visited memory such that it returns the
optimal solution? Select all that apply.

   A. ℎ((𝑥, 𝑦, 𝑧)) = 𝑥 + 𝑦 + 4𝑧 .
   B. ℎ((𝑥, 𝑦, 𝑧)) = 𝑥 + 𝑦 + 2𝑧 . − 2
   C. ℎ((𝑥, 𝑦, 𝑧)) = 𝑥 + 𝑦 + 𝑧 . + 𝑧 − 2
   D. ℎ((𝑥, 𝑦, 𝑧)) = 4𝑧 . − 4
   E. ℎ((𝑥, 𝑦, 𝑧)) = 2𝑧 . − 2
   F. ℎ((𝑥, 𝑦, 𝑧)) = 𝑧 . + 𝑧 − 2.
   G. ℎ((𝑥, 𝑦, 𝑧)) = 𝑧 . − 1.
   H. None of the above.

<!-- answer:start -->
exact
H
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** H -- UPDATED

2D. H -- UPDATED

Updated: We made a mistake in the previous version of the solution script. Recall that in
this question, 𝑥, 𝑦 moves are without cost and A* is without visited memory. Consider an
example where the optimal solution requires a path that moves from 𝑧 to 𝑧 + 1, i.e., the
drone must go up first before being able to land at 𝑧 = 1. The up move will never be first in
the priority queue, if there are any 𝑥, 𝑦 moves available. The algorithm may cycle between
the 𝑥, 𝑦 coordinates indefinitely. It will never perform the necessary move to 𝑧 + 1. Hence, for
none of the heuristics will A* return the optimal solution, and selecting H gives full marks.

Local Search
<!-- explanation:end -->

## q3A [hard]

**Question 3A [4 marks]**

Given the Elevator Problem, which of the following local search formulation(s)

is/are reasonable? Select all that apply.
In this context, we consider the formulation reasonable if hill-climbing with an infinitely large
number of random restarts can return the optimal sequence of actions for the problem.
Note that, at every restart, the initial state and the output of the successor function may differ
from the previous run if there is randomness involved.

A.
State: (current floor, total cost so far).
Initial State: (floor 1, cost = 0).
Goal Test: cost ≥ 40.
Evaluation Function: negative of the path cost.
Successors: move up or open.

B.
State: (current floor, how many floors have been visited).
Initial State: (floor 1, visited = 1).
Goal Test: visited = 20.
Evaluation Function: number of “visited” floors.
Successors: next floor = current floor ± 1.

C.
State: (current floor, state of each passenger: waiting, in elevator, or delivered).
Initial State: elevator on floor 1, all passengers waiting.
Goal Test: all passengers are delivered; floor of elevator does not matter.
Evaluation Function: number of passengers delivered.
Successors: all states you get by either moving the elevator one floor up/down (if within 1–
20) or opening the elevator (allowing boarding/alighting).

D.
State: (current floor, number of passengers in elevator).
Initial State: (floor 1, zero passengers in elevator).

Goal Test: elevator at floor 20.
Evaluation Function: number of visited floors.
Successors: move 1–3 floors up in a single step.

E.
None of the above

<!-- answer:start -->
exact
E
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** E. None of the above

All the state representations do not include the information needed to extract the optimal
solution (i.e., the optimal sequence of actions).

In local search, we start from a random state rather than the actual initial state, and the
optimization steps explore neighbors of the current state without explicitly modeling the
problem’s actions. Therefore, the steps taken during optimization to reach an optimal final
state are not the same as the sequence of actions from the actual initial state to the optimal
goal state. Moreover, local search returns only the final state, not the path leading to it. Even
if the path is returned, it also does not correspond the optimal sequence of actions.

Updated:
In the lecture slides, we have clearly stated that in local search, a state is a candidate
solution and that hill climbing returns a state, not a path to a state. We have also given an
example where we apply local search to a path finding problem. Notice that we had to
modify the state formulation to encode the path so that it can work with local search. If the
local search solution is a sequence of actions and that somehow this encodes the optimal
solution, then there is no need for us to modify the formulation for path finding.

In the AY2024/2025 Sem 2 midterm, Option D was accepted as correct even though it did
not explicitly include the path. This was based on an additional (but unstated) assumption in
the solution following post-midterm queries: we allow an assumption that the MRT graph is
mostly chains with no cycles. Under this assumption, the set of visited states uniquely
determines the path, which is why we allowed Option D as correct. We will update the past
paper and solution to make this assumption explicit and avoid future confusion. However, it
is important to…
<!-- explanation:end -->

## q4A [easy]

**Question 4A [1 mark]**

How many terminal nodes are there in the game tree generated by the

algorithm specified in the context?

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** 5 (Tree) OR 4 (Graph) -- UPDATED

4A. 5 (Tree) OR 4 (Graph) -- UPDATED

Terminal nodes reached within the cutoff: 20 (A wins), 17 (draw) from 13, 17 (draw) from
12+5, 19 (draw) from 15, and 19 (draw) from 14+5.

Updated:

In the lecture slides, we have clearly defined what terminal states and terminal nodes are:

   •    “End State (Terminal State): A state where the game ends, with no further moves
        possible.”
   •    “The outcome of the game (such as monetary payoff, win, loss, or draw) is displayed
        beneath each terminal node (end state).”

We have also explicitly stated that the cutoff is performed on mid-game states. If the state in
node “16 (-1)” is a terminal state, there is no need to apply the cutoff to that state and use
evaluation function which uses heuristic instead of utility function. The heuristic value
coinciding with the actual utility value has nothing to do with the definition of terminal nodes.

Regarding the definition of a valid move, we have made it clear above in the original
solution.

Regarding the game tree, we have explicitly stated in the context that "The game tree is tree
(a child can only have one parent).". However, we will be lenient and accept a graph
representation with four terminal nodes as an answer.
<!-- explanation:end -->

## q4B [easy]

**Question 4B [1 mark]**

How many nodes in the full game tree generated by Minimax without cutoff

exceed the cutoff depth specified in the context?

<!-- answer:start -->
exact
2
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** 2

4B. 2

Without cutoff, only the branch through 16 continues: 16 (depth 4, non-terminal) → 18 (depth
5) → 20 (depth 6, terminal). So there are 2 nodes beyond depth 4.
<!-- explanation:end -->

## q4C [easy]

**Question 4C [1 mark]**

What is the game tree's maximum depth if we don’t perform cutoff? Note:

please write “infinity” if the depth is infinite.

<!-- answer:start -->
exact
6
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** 6

4C. 6

The deepest terminal path without cutoff is 5 → 10 → 12 → 14 → 16 → 18 → 20, which
reaches depth 6.
<!-- explanation:end -->

## q4D [easy]

**Question 4D [1 mark]**

If Player A moves to number 9 on their first turn, which player can win the game

if they both play optimally in the subsequent turns?
     A. A
     B. B
     C. Game is draw
     D. Game continues indefinitely
     E. None of the above

<!-- answer:start -->
exact
C
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** C. Game is draw

4D. C. Game is draw

If A goes to 9, B will optimally choose +4 → 13 (since x2 → 18 lets A win immediately),
leading to 13 → 17 where B has no moves, so draw.
<!-- explanation:end -->

## q4E [easy]

**Question 4E [1 mark]**

Which sequences (from the first turn) guarantee a Player A victory if both

players play optimally in the subsequent turns? Select all that is/are true.
    A. A: x2 → B: +5
    B. A: x2 → B: +2

   C. A: x2 → B: +2 → A: +2
   D. A: +4 → B: x2
   E. None of the above

<!-- answer:start -->
exact
D
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** D. A: +4 → B: x2

4E. D. A: +4 → B: x2

   •    A: x2 → B: +5: B would avoid +5 (to 15) as it gives A a safe draw; optimal is +2 → 12
        leading to draw if A plays optimally.
   •    A: x2 → B: +2: Leads to 12 where A can choose +5 → 17 for a draw.

   •   A: x2 → B: +2 → A: +2: This line actually lets B eventually win (ends in 20 for B), so
       not an A win.
   •   A: +4 → B: x2: This leads to Player A victory.

Updated:

The question asks for the sequence of moves presented, which would lead to player A’s
victory if both players play optimally after that.

A different interpretation for the phrase “in the subsequent turns” was suggested in the
forum. Let us inspect the question statement: “Which sequences (from the first turn)
guarantee a Player A victory if both players play optimally in the subsequent turns?”

The parenthesis allows for two interpretations:

   1. “sequences that start from the first turn and that guarantee [...] in subsequent […]”
   2. “sequences that guarantee from the first turn that […] in subsequent […]”

Interpretation 2. is wrong, or less likely to be correct from a test-takers point of view, in
several aspects. First, it is undefined what is the starting point of the given sequences.
Second, in the question, “first turn” is written inside the parenthesis, hence it is semantically
not correct for “subsequent” to refer to the first turn. Third, it does not make sense to
introduce “sequences” if the question only cares about the first move. Fourth, if one arrived
at the correct game tree, then one will notice that there is a sequence of depth 3 that leads
to a Player A victory. It should then become immediately clear that Option D is a sequence
leading towards that victory sequence.

Overall, the question is a test if the correct game tree was found,…
<!-- explanation:end -->

## q4F [easy]

**Question 4F [1 mark]**

Which of the following action(s) should the first player take on their first turn?

Assume all players play optimally in the subsequent turns. Select all that is/are true.
    A. +4
    B. x2
    C. All actions result in losing the game
    D. All actions result in a draw
    E. Game continues indefinitely

<!-- answer:start -->
exact
D
ABD
A B D
A,B,D
A, B, D
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** D. All actions result in a draw or A,B,D -- UPDATED

4F. D. All actions result in a draw or A,B,D -- UPDATED

From 5, both A’s options (+4 → 9 or x2 → 10) lead to positions where, with optimal play, the
result is a draw.

Updated:

We will allow two answers:

   •   D. All actions result in a draw. Thus, the agent does not have any action that it should
       take.
   •   A,B,D. The agent is indifferent to both actions (A and B), thus can select both, and at
       the same time, it’s aware that both actions result in a draw.

Note that the question assumes that the opponent plays optimally in the subsequent turns;
thus, it doesn’t matter whether the action leads to a higher or lower probability of winning.

Alpha-Beta Pruning
<!-- explanation:end -->

## q4G [hard]

**Question 4G [4 marks]**

Suppose we traverse this tree using (depth-first) alpha-beta pruning from right

to left. Select all the link(s) that would be pruned by alpha-beta pruning algorithm. Select
only the links that are directly pruned and not those that are indirectly pruned because they
are in a subtree of a pruned link.

Which of the following link(s) is/are pruned? Select all that is/are true.
   A. A
   B. B
   C. C
   D. D
   E. E
   F. F
   G. G
   H. H
   I. I
   J. J
   K. K
   L. L

     M. M
     N. N
     O. None of the above

<!-- answer:start -->
exact
CK
C K
C,K
C, K
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** C and K
<!-- explanation:end -->

## q5A [easy]

**Question 5A [1 mark]**

What is the entropy of the Restaurant Decision (yes/no) in the table, rounded to

two decimal places?

<!-- answer:start -->
exact
0.92
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** 0.92

7      7      .         .
 Explanation: − 8 log . i8j − 8 log . (8) = 0.918.
<!-- explanation:end -->

## q5B [easy]

**Question 5B [1 mark]**

What is the information gain of selecting “Price” as the root node of the

Restaurant Decision (yes/no) in the table, rounded to two decimal places?

<!-- answer:start -->
exact
0.46
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** 0.46

5B. 0.46
                                                                              9
 The conditional entropy when splitting according to price is H(Y|Price)= 8 𝐻(1,0) +
 9      . *
     𝐻 i9 , 9j =0.4575. Hence the information gain is 0.918 − 0.457 = 0.461.
 8
<!-- explanation:end -->

## q5C [easy]

**Question 5C [1 mark]**

What is the information gain of selecting “Review” as the root node of the

Restaurant Decision (yes/no) in the table, rounded to two decimal places?

<!-- answer:start -->
exact
0.04
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** 0.04

5C. 0.04
                                                                                      .     * *
 The conditional entropy when splitting according to Review is H(Y|Review)= 8 𝐻 i. , .j +
 7      * 9
     𝐻 i7 , 7j =0.8742. Hence the information gain is 0.918 − 0.874 = 0.044.
 8
<!-- explanation:end -->

## q5D [easy]

**Question 5D [1 mark]**

What is the information gain of selecting “Distance” as the root node of the

Restaurant Decision (yes/no) in the table, rounded to two decimal places?

<!-- answer:start -->
exact
0.00
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** 0.00

5D. 0.00
                                                                                  9       * .
 The conditional entropy when splitting according to Distance is H(Y|Dist)= 8 𝐻 i9 , 9j +
 9      . *
 8
     𝐻 i9 , 9j =0.915. Hence the information gain is 0.918 − 0.915 = 0.003.
<!-- explanation:end -->

## q5E [medium]

**Question 5E [2 marks]**

Build the decision tree given the context information. What is/are the last

attribute(s) in the longest path(s) of the decision tree? Select all that applies.

    A. Price.
    B. Review.
    C. Distance.
    D. None of the above.

<!-- answer:start -->
exact
B
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** B

5E. B

 The decision tree is as follows:

                                        Price

                              100-200           20-100
                                                YES (3)

                       Distance

              Long                Short
                                  No (1)

          Review

Average            Very good
NO(1)              YES (1)
<!-- explanation:end -->

## q5F [easy]

**Question 5F [1 mark]**

According to the decision tree built in Question 5E, consider the Restaurant

Decision for the following restaurants.

Select the restaurant(s) with a Yes decision. Select all that apply.

    A. Price: 100-200, Review: Average, and Distance: Short.
    B. Price: 20-100, Review: Average, and Distance: Long.
    C. None of the above.

<!-- answer:start -->
exact
B
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** B

5F. B

       A. Price: 100-200, Review: Average, and Distance: Short. This example leads to No.
       B. Price: 20-100, Review: Average, and Distance: Long. This example leads to Yes.

Part 6: Linear regression and logistic regression.
Context for Questions 6A and 6B
                                                                    𝑤;     2
Consider the linear regression model ℎ: (𝑥) = 𝑤; + 𝑤* 𝑥* where 𝑤 = l𝑤 m = l m. Consider
                                                                     *     1
                                   *                          .
the MSE loss function 𝐽'<= (𝑤) = .3 ∑3+-*Lℎ: L𝑥 (+) M − 𝑦 (+)M .
Consider the following training data.

 Training               𝑥*                𝑦
 point
      𝑥 (*)       0                0
      𝑥 (.)       2                1
      𝑥 (9)       3                2

Context for Questions 6C and 6D
Consider the following customer data. The customers are described by the following features
and a target variable.

         𝑥*                  𝑥.                  𝑥9                 𝑦
 0.3                  0.9                 1000                     𝑦 (*)
 0.2                  0.6                 3400                     𝑦 (.)
 0.7                  2.1                 500                      𝑦 (9)
 0.4                  1.2                 1900                     𝑦 (7)

The values 𝑦 (*) , … , 𝑦 (7) are real numbers.
<!-- explanation:end -->

## q6A [medium]

**Question 6A [2 marks]**

Using the given data, evaluate the hypothesis and the loss function. Fill in the

following blanks:

ℎ: L𝑥 (*) M [𝐵𝐿𝐴𝑁𝐾1]
ℎ: L𝑥 (.) M [𝐵𝐿𝐴𝑁𝐾2]
ℎ: L𝑥 (9) M [𝐵𝐿𝐴𝑁𝐾3]
𝐽'<= (𝑤) [𝐵𝐿𝐴𝑁𝐾4]

                                                     𝑤;

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** ℎ: L𝑥 (*) M = 2 + 0

ℎ: L𝑥 (*) M = 2 + 0

ℎ: L𝑥 (.) M = 2 + 1 ∗ 2 = 4
ℎ: L𝑥 (9) M = 2 + 1 ∗ 3 = 5
             *               **
𝐽'<= (𝑤) = [2. + 3. + 3. ] =
           8                    9
<!-- explanation:end -->

## q6B [medium]

**Question 6B [2 marks]**

Perform a single update to the vector l𝑤 m using the stochastic gradient

                                                          *
descent (SGD) update rule with the learning rate 𝛾 = 1/3. The SGD algorithm uses a
random number generator that generates a random integer from 1 to n. Assume that for this
                                                                                   𝑤 ′
update, the random generator outputs 2. Fill into the blank the resulting weights t ; v as
                                                                                   𝑤* ′
          𝐵𝐿𝐴𝑁𝐾1
follows: l       m
          𝐵𝐿𝐴𝑁𝐾2

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** ℎ: L𝑥 (.) M − 𝑦 (.) = 4 − 1 = 3.

6B.

ℎ: L𝑥 (.) M − 𝑦 (.) = 4 − 1 = 3.
 𝑤 ′       𝑤;        𝑔;     2    *                    1     2    *      1     1
t ; v = l𝑤 m − 𝛾 l𝑔 m = l m − 9 LℎL𝑥 (.) M − 𝑦 (.) M l m = l m − 9 (3) l m = l m.
 𝑤* ′        *         *    1                         2     1           2     −1
<!-- explanation:end -->

## q6C [medium]

**Question 6C [2 marks]**

Consider the data given in the Context for this question. You consider two sets

of features, 𝑥 = [𝑥* , 𝑥. , 𝑥9 ]@ and 𝑥′ = [𝑥* , 𝑥9 ]@ . You have asked two friends to train linear
regression models using gradient descent. They both use appropriate learning rates and
number of steps until no more change of weights are observed. Friend A returns with results
ℎ A and ℎ AB , corresponding to the use of 𝑥 and 𝑥’, respectively. Friend B returns with results

ℎC and ℎCB , corresponding to the use of 𝑥 and 𝑥’, respectively. Select correct statements.
Select all that apply.

      A. 𝐽'<= (ℎ A ) = 𝐽'<= (ℎC ) with respect to the given data in the Context.
                  !
      B. 𝐽'<= (ℎ A ) = 𝐽'<= (ℎC ′) with respect to the given data in the Context.
      C. The weights of ℎ A are equal to the weights of ℎC .
      D. The weights of ℎ AB are equal to the weights of ℎCB .
      E. None of the above.

<!-- answer:start -->
exact
ABD
A B D
A,B,D
A, B, D
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** ABD.

6C. ABD.

The set of features 𝑥 = [𝑥* , 𝑥. , 𝑥9 ]@ contains the linearly dependent feature 𝑥. = 3 ∗ 𝑥* . The
loss function 𝐽'<= will be convex in the corresponding weights 𝑤 when features 𝑥 are used
and strictly convex in the corresponding weights 𝑤′ when features 𝑥′ are used. For a convex
function, a local minimum is a global minimum, i.e., there can be multiple equivalent minima.
For a strictly convex function, there is a single global minimum. Training with features 𝑥 can
lead to different weights 𝑤A or 𝑤C , depending on the initialization of gradient descent. Both
will minimize the loss function, achieving the same value 𝐽'<= (ℎ A ) = 𝐽'<= (ℎC ).

Note that they both use appropriate learning rates and number of steps until no more change
of weights are observed, hence the theorem conditions mentioned in lecture are satisfied.
<!-- explanation:end -->

## q6D [medium]

**Question 6D [2 marks]**

Suggest changes to improve the learning stability on the data set in the

Context using all the features. Select all that apply.

      A. Subtract the feature standard deviation from 𝑥9 and divide by the feature mean.
      B. Subtract the feature mean from 𝑥9 and divide by the standard deviation.
      C. Min-max scaling of feature 𝑥9
      D. Fix the gradient descent learning rate to a single value.
      E. Use the normal equation to obtain the weights.
      F. None of the above.

<!-- answer:start -->
exact
BC
B C
B,C
B, C
BCE
B C E
B,C,E
B, C, E
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** BC or BCE -- UPDATED

6D. BC or BCE -- UPDATED

For learning stability, we notice that 𝑥9 has a greatly different scale than the other features.
Option A is non-sensical, while B is the standardization of feature 𝑥9 , and C the appropriate
min-max scaling of feature 𝑥9 . Regarding D, a variable learning rate may help, say different
learning rates for the different features.

Updated:
Option E does not work when we use the matrix inverse since we have the linearly
dependent feature which makes the 𝑋 @ 𝑋 matrix in the normal equation not invertible.
However, using the pseudoinverse, which was hinted in tutorial, can make the normal
equation work. Hence, we allow both selections for full marks.
<!-- explanation:end -->

## q6E [medium]

**Question 6E [2 marks]**

We would like to construct a machine learning model to predict the chance of

a thunderstorm 𝑦 in the next hour. Let there be three attributes 𝑥 = [𝑥* , 𝑥. , 𝑥9 ]@ used for
prediction. In addition, it was observed that when 𝑥* = 𝑥. = 𝑥9 = 0, the chance of a
thunderstorm was greater than 50%. Which of the following model(s) is/are correct? Hint:
consider conceptual correctness as well. Select all that apply.

      A. 𝑝(𝑦|𝑥) = 𝜎(𝑤! 𝑥! + 𝑤" 𝑥" + 𝑤# 𝑥# ).
      B. 𝑝(𝑥|𝑦) = 𝜎(𝑤! 𝑥! + 𝑤" 𝑥" + 𝑤# 𝑥# ).
      C. 𝑝(𝑦|𝑥) = 𝑤$ + 𝑤! 𝑥! + 𝑤" 𝑥" + 𝑤# 𝑥# .
      D. 𝑝(𝑥|𝑦) = 𝑤$ + 𝑤! 𝑥! + 𝑤" 𝑥" + 𝑤# 𝑥# .
      E. 𝑝(𝑦|𝑥) = 𝜎(𝑤$ + 𝑤! 𝑥! + 𝑤" 𝑥" + 𝑤# 𝑥# ).
      F. 𝑝(𝑥|𝑦) = 𝜎(𝑤$ + 𝑤! 𝑥! + 𝑤" 𝑥" + 𝑤# 𝑥# ).
      G. 𝑝(𝑦|𝑥) = 𝑤! 𝑥! + 𝑤" 𝑥" + 𝑤# 𝑥# .
      H. 𝑝(𝑥|𝑦) = 𝑤! 𝑥! + 𝑤" 𝑥" + 𝑤# 𝑥# .
      I.   None of the above.

<!-- answer:start -->
exact
E
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** E

6E. E

From the question it is evident that logistic regression is required, since “chance of
thunderstorm” is equivalent to “probability of thunderstorm”. In addition, it was observed that
at 𝑥* = 𝑥. = 𝑥9 = 0 the chance of a thunderstorm was greater than 50%. Hence, we require
an offset, since 𝜎(0) = 0.5. Lastly, notice that logistic regression expresses the conditional
probability of a thunderstorm given that an observation 𝑥 is made. The correct mathematical
expression for this conditional probability is 𝑝(𝑦|𝑥) and not 𝑝(𝑥|𝑦).
<!-- explanation:end -->

## q6F [medium]

**Question 6F [2 marks]**

1 0              5

Let 𝑋 = l     m and 𝑌 = l m. Find the best weights using the normal equation. Fill in the
         0 2              6
blanks with the results w = [Blank1, Blank2]^T.

Hint: Invert a diagonal matrix by inverting the diagonal entries, i.e., the i-th diagonal entry
di becomes 1/di.

<!-- answer:start -->
exact
5,3
5, 3
5 3
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** 5, 3

6F. 5, 3

                         1 0                                                           1  0
First, compute 𝑋 @ 𝑋 = l      m. Then invert, by inverting the diagonals, (𝑋 @ 𝑋)D* = t      v.
                         0 4                                                           0 1/4
                1 0 5        5                            1    0    5   5
Also, 𝑋 @ 𝑌 = l    m l m = l m. Thus, (𝑋 @ 𝑋)D* 𝑋 @ 𝑌 = t        v l m=l m.
                0 2 6       12                            0 1/4 12 3
<!-- explanation:end -->
