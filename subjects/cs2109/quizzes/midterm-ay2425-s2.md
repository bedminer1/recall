# AY2024/25 Semester 2 Midterm

Keep `CS2109S+AY2024-25+Sem+2+-+Midterm+-+Solution.pdf` open for the shared context, tables, and figures. Answer choices are reproduced below where present.

## q1A [easy]

**Question 1A [1 mark]**

Is it true that the search formulation results in a state space where a state

can be visited multiple times? Note: we do not care about the search algorithms in
this question since we are asking about the state space, not the search tree.

   a. Yes
   b. No

<!-- answer:start -->
exact
A
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** a

The state space consists of all integers (…, -2, -1, 0, 1, 2, …). Because you can use “Add
1” and “Subtract 1” moves, you can return to the same integer through different paths
(for instance, you can move up from 5 to 6, and then subtract 1 to get back to 5,
revisiting that state).
<!-- explanation:end -->

## q1B [easy]

**Question 1B [1 mark]**

Does the search formulation result in many goal states?

   a. Yes
   b. No
                                                         CS2109S Midterm Assessment

<!-- answer:start -->
exact
B
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** b

1B. b

The goal test is “Is the current integer equal to M?” That is a single condition referring to
a single value, M. Therefore, there is only one goal state in this problem.
<!-- explanation:end -->

## q1C [easy]

**Question 1C [1 mark]**

Is there always a solution when using the search formulation?

   a. Yes
   b. No

<!-- answer:start -->
exact
A
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** a

1C. a

Because we have both “Add 1” and “Subtract 1,” we can move from any integer N to
any other integer M simply by repeated increments or decrements. Although we may
use the multiply/divide operations to shorten the path, those are not strictly necessary
to reach M; hence a solution always exists.
<!-- explanation:end -->

## q1D [easy]

**Question 1D [1 mark]**

Suppose that you use search without visited memory and queue-based

search (e.g., BFS, UCS). Is the search tree finite?

   c. Yes
   d. No

<!-- answer:start -->
exact
A
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** a

1D. a

Since a solution exists for any N and M, BFS/UCS will find the solution since it explores
the path from smallest depth/cost to largest depth/cost.
<!-- explanation:end -->

## q1E [easy]

**Question 1E [1 mark]**

Suppose that you use search without visited memory and Depth-First

Search (DFS). Does the search always terminate?

   e. Yes
   f. No

<!-- answer:start -->
exact
B
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** b

1E. b

In an unbounded and cyclic state space (no visited memory), a naive DFS could keep
following paths (e.g., keep subtracting or adding 1, then returning) that lead to cycles or
unbounded growth. There is no guarantee it will terminate.
<!-- explanation:end -->

## q1F [medium]

**Question 1F [2 marks]**

Which of the following search (without visited memory) algorithm(s) can

we employ such that the search always finds an answer (valid solution) if a solution
exists?

   a. Breadth-First Search (BFS)
   b. Depth-First Search (DFS)
   c. Uniform-Cost Search (UCS)
   d. Depth-Limited Search (DLS) with DFS and max-depth M
   e. None of the above

<!-- answer:start -->
exact
AC
A C
A,C
A, C
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** a, c

1F. a, c

Uniform-Cost Search (UCS) and Breadth-First Search (BFS)

BFS explores layer by layer (allowing for repeated expansion of new states without
skipping any reachable depth), so if a solution is reachable in finite steps, BFS will
eventually find it.

UCS explores paths in increasing order of path cost. Since all action costs are finite and
nonnegative, any state reachable via a finite-cost path will eventually be expanded by
UCS.
                                                         CS2109S Midterm Assessment

DFS, without visited memory, can get stuck going down an infinite path and may not
return to discover a valid solution.

Depth-Limited Search (DLS) uses an incorrect depth. If M is always positive (not true!),
the correct depth should be M-N to account for a negative initial state N. However,
since M can be negative, DLS with negative max-depth is undefined.
<!-- explanation:end -->

## q1G [medium]

**Question 1G [2 marks]**

Suppose that we use search with visited memory. Which of the

following search algorithm(s) is/are the best for the problem?

Best means the algorithm(s) should be complete, optimal, efficient (in terms of big O
worst-case space and time complexity), and aware if there is no solution.

   a. Depth-First Search (DFS)
   b. Uniform-Cost Search (UCS)
   c. Depth-Limited Search (DLS) with DFS and max-depth M
   d. Iterative Deepening Search (IDS) with DFS
   e. None of the above
                                                            CS2109S Midterm Assessment

<!-- answer:start -->
exact
B
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** b

1G. b

Uniform-Cost Search (UCS)

Depth-First Search (DFS) and Depth-Limited Search (DLS) fail (same reason as 1F).

IDS may return a solution with higher cost which resides in the same/shallower depth.

Example: Transform N=5 into M=8. IDS may return Path 1.

Path 1: 5 –(x2)➝ 10 –(-1)➝ 9 –(-1)➝ 8, cost: 2+1+1=4, depth: 3 (suboptimal)

Path 2: 5 –(+1)➝ 6 –(+1)➝ 7 –(+1)➝ 8, cost: 1+1+1=3, depth: 3 (optimal)

UCS is the only algorithm listed that guarantees completeness, optimality, and
practical efficiency with visited memory.
                                                           CS2109S Midterm Assessment
<!-- explanation:end -->

## q2A [medium]

**Question 2A [2 marks]**

Is ℎ&!" an admissible heuristic to use for the A* algorithm? Hint: consider

a relaxed version of the problem.

           a. Yes.
           b. No.
                                                           CS2109S Midterm Assessment

<!-- answer:start -->
exact
A
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** a (Main assumption), b (Alternative assumption)

2A. a (Main assumption), b (Alternative assumption)

First, consider the relaxed maze without the walls inside the grid. For this relaxed
problem, the true cost from a state (𝑥, 𝑦) to reach the goal state (0,0) is given as
follows.
Main assumption: 𝑥 describes the left/right direction, 𝑦 describes the up/down
direction. In a 2D graphic typically the 𝑥 direction is the horizontal direction, and the 𝑦
direction is the vertical direction. However, we allow for the other interpretation as well
(with the limitation mentioned in Part C).
The cost of going from 𝑥 to 0 is 2𝑥, because each left/right step costs 2 and 𝑥 steps are
taken. An up/down step from 𝑦 costs 4𝑦 and 𝑦 steps are taken to go to 0, hence the cost
                           )+(+0.)
to the goal is 4 ∑+,-. 𝑗 =   '
                                   = 2𝑦 ' + 2𝑦. The total cost for the relaxed maze is
ℎ$&2#3 = 2𝑥 + 2𝑦 ' + 2𝑦.
Since we considered the relaxed maze, we know that ℎ$&2#3 (𝑥, 𝑦) ≤ ℎ∗ (𝑥, 𝑦), i.e., it must
be smaller or equal than the true cost. Since the friend’s estimate ℎ&!" (𝑥, 𝑦) = 𝑥 + 𝑦 ' ≤
ℎ$&2#3 (𝑥, 𝑦) for all (𝑥, 𝑦), we have that ℎ&!" (𝑥, 𝑦) ≤ ℎ∗ (𝑥, 𝑦) for all (𝑥, 𝑦). Hence, ℎ&!" is
admissible.

Alternative assumption: y describes the left/right direction, 𝑥 describes the up/down
direction.

The cost of going from 𝑥 to 0 is 4𝑦 𝑥, because each up/down step costs 4𝑦 and 𝑥 steps
are taken. A left/right step from 𝑦 costs 2 and 𝑦 steps are taken to go to 0, hence the
cost to the goal is 2y. The total cost for the relaxed maze is ℎ$&2#3 (𝑥, 𝑦) = 4𝑥𝑦 + 2𝑦.
Since we considered the relaxed maze, we know that ℎ$&2#3 (𝑥, 𝑦) ≤ ℎ∗ (𝑥, 𝑦), i.e., it must
be smaller or equal than the true cost. Since the friend’s estimate ℎ&!" (𝑥, 𝑦) = 𝑥 + 𝑦 ' ≥
ℎ$&2#3 (𝑥, 𝑦) for many (𝑥, 𝑦), for example (0,3). Hence, ℎ&!" is…
<!-- explanation:end -->

## q2B [medium]

**Question 2B [2 marks]**

Is ℎ&!" a consistent heuristic to use for the A* algorithm?

          a. Yes.
          b. No.

<!-- answer:start -->
exact
A
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** a (Main assumption), b (Alternative assumption)

2B. a (Main assumption), b (Alternative assumption)

Main assumption: The possible actions are move left, right, up, down, and we are given
the costs for them. For evaluating consistency, we must check the triangle inequality
that says that a consistent heuristic underestimates the cost of making the
corresponding move.
                                                            CS2109S Midterm Assessment

For a move left/right, we have ℎ&!" (𝑥, 𝑦) − ℎ&!" (𝑥 + 1, 𝑦) = 𝑥 + 𝑦 ' − (𝑥 + 1) − 𝑦 ' =
−1 ≤ 2, and ℎ&!" (𝑥, 𝑦) − ℎ&!" (𝑥 − 1, 𝑦) = 𝑥 + 𝑦 ' − (𝑥 − 1) − 𝑦 ' = 1 ≤ 2.

For a move up/down, we have ℎ&!" (𝑥, 𝑦) − ℎ&!" (𝑥, 𝑦 + 1) = 𝑥 + 𝑦 ' − 𝑥 − (𝑦 + 1)' =
−2𝑦 − 1 ≤ 4𝑦, and ℎ&!" (𝑥, 𝑦) − ℎ&!" (𝑥, 𝑦 − 1) = 𝑥 + 𝑦 ' − 𝑥 − (𝑦 − 1)' = 2𝑦 − 1 ≤ 4𝑦.

Hence, the heuristic is consistent.

Other assumption: Since ℎ&!" is not admissible, it cannot be consistent.
<!-- explanation:end -->

## q2C [medium]

**Question 2C [2 marks]**

Which heuristic is preferred for use with A* without visited memory?

          a. ℎ&!" .
          b. ℎ&!"' ((𝑥, 𝑦)) = 4𝑥 + 2𝑦 ' .
          c. ℎ&!"( ((𝑥, 𝑦)) = 2𝑥 + 2𝑦.
          d. ℎ&!") ((𝑥, 𝑦)) = 2𝑥 + 2𝑦 ' .
          e. ℎ&!"* ((𝑥, 𝑦)) = 2𝑥 + 4𝑦 ' .
                                                              CS2109S Midterm Assessment

<!-- answer:start -->
exact
D
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** d (Main assumption)

2C. d (Main assumption)

A preferred heuristic is admissible (theorem for optimality of A* without visited
memory) and dominates other admissible heuristics.

Main assumption:

The closest estimate of the true cost we obtain is ℎ$&2#3 = 2𝑥 + 2𝑦 ' + 2𝑦. Hence, when
a heuristic is bigger than ℎ$&2#3 , we cannot guarantee admissibility. Heuristic ℎ&!"' is not
admissible, since ℎ&!"' (𝑥, 0) = 4𝑥 ≥ 2𝑥 = ℎ$&2#3 (𝑥, 0). Heuristic ℎ&!"* is not admissible,
since ℎ&!"* (0,2) = 4 ∗ 2' = 16 > ℎ$&2#3 (0,2) = 8 + 4 = 12.

The remaining heuristics are admissible, and we find that for all 𝑥 and 𝑦 it holds that 𝑥 +
𝑦 ' ≤ 2𝑥 + 2𝑦 ' and 2𝑥 + 2𝑦 ≤ 2𝑥 + 2𝑦 ' , hence ℎ&!") = 2𝑥 + 2𝑦 ' is preferred.

Other assumption:

The closest estimate of the true cost we obtain is ℎ$&2#3 = 4𝑥𝑦 + 2𝑦. Hence, when a
heuristic is bigger than ℎ$&2#3 , we cannot guarantee admissibility. For the given
heuristics, we see that none of them is admissible. Check the point (𝑥, 0), where
ℎ$&2#3 (𝑥, 0) = 0, and hence smaller than all the given heuristics for 𝑥 > 0. At this
moment, you should have noticed that your assumption was most likely incorrect and
checked the main assumption. However, we have decided to be kind and give 4 marks
for the combined choice of No for Part A and No for Part B. If that choice is made,
automatically no marks for Part C are received. Given that we do not know your
assumption, you will receive the max of this marking and the marking arising from the
main assumption.
                                                           CS2109S Midterm Assessment
<!-- explanation:end -->

## q3A [hard]

**Question 3A [4 marks]**

You are tasked with finding the longest path starting at Jurong East MRT Station in the
Singapore MRT network, such that the path visits the most stations exactly once.

Which of the following local search formulation is/are reasonable?

In this context, we consider the formulation reasonable if hill-climbing with an infinitely
large number of random restarts can reach the global optimum.

   a. State: Current station
      Initial State: Jurong East
      Goal Test: All stations are visited
      Evaluation Function: Total distance traveled
      Successor Function: Move to any adjacent station
   b. State: List of visited stations
      Initial State: [Jurong East]
      Goal Test: Path includes 10+ stations
      Evaluation Function: Number of unique stations visited
      Successor Function: Travel to a random unvisited station
   c. State: Current path (sequence of stations)
      Initial State: [Jurong East]
      Goal Test: No unvisited adjacent stations remain
      Evaluation Function: Length of the path (number of stations)
      Successor Function: Extend the path to an adjacent unvisited station
   d. State: Current station and visited set
      Initial State: (Jurong East, {Jurong East})
      Goal Test: All stations are in the visited set
      Evaluation Function: Total stations in the visited set
      Successor Function: Move to adjacent stations not in the visited set
   e. None of the above.
                                                          CS2109S Midterm Assessment

<!-- answer:start -->
exact
CD
C D
C,D
C, D
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** c, d (main assumption) or c (main assumption, alt) or e (alternative assumption)

We will accept two assumptions: stochastic (main assumption) and deterministic
(alternative assumption) handling of ties.

Main Assumption

Under the stochastic assumption, the correct answer is C and D. Since there is a
possible different interpretation of “adjacent station”, we will also accept C only
(without D).

Formulation C:

   •   State: Current path (sequence of stations).
       This explicitly tracks the path and ensures adjacency (via successor function).
   •   Evaluation function: Path length (number of stations).
   •   Successor function: Extends the path to adjacent unvisited stations.
   •   Hill-climbing greedily maximizes path length. With infinite restarts, it can explore
       all possible paths and find the longest valid one.

Formulation D:

   •   State: Current station + visited set.
       Tracks visited stations (prevents revisits) and enforces adjacency (via successor
       function).
   •   Evaluation function: Number of visited stations.
   •   Successor function: Moves to adjacent unvisited stations.
   •   The visited set ensures no cycles, and the evaluation function directly aligns with
       the goal. Infinite restarts allow finding the maximum possible path. The visited
       set implicitly defines the path’s stations (even if the order isn’t tracked).

Formulation A and B:
   • Do not properly track visited stations or enforce adjacency, leading to invalid
      paths.
   • Formulation 2’s successor function allows non-adjacent moves (unrealistic for
      MRT).

Ambiguity in the “adjacent station” definition for formulation D
It could be interpreted as allowing moves to any adjacent station from any station in the
visited set, which would make D incorrect. Therefore, we will also accept only C as the
correct answer (without…
<!-- explanation:end -->

## q4A [easy]

**Question 4A [1 mark]**

How many terminal nodes exist in the game tree?

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** 5 (tree) or 3 (graph)

4A. 5 (tree) or 3 (graph)
<!-- explanation:end -->

## q4B [easy]

**Question 4B [1 mark]**

What is the game tree's maximum depth (root node is at depth 0)?

<!-- answer:start -->
exact
5
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** 5

4B. 5
<!-- explanation:end -->

## q4C [easy]

**Question 4C [1 mark]**

If Player A moves to position 3 on their first turn, which player can force a

win?

   a. A
   b. B
   c. Game is draw
   d. Game continues indefinitely
   e. None of the above

<!-- answer:start -->
exact
C
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** c (main assumption) or e (alternative assumption)

4C. c (main assumption) or e (alternative assumption)

If player B plays optimally (main assumption), the game results in a draw, therefore, C is
the correct answer.

If player B plays sub-optimally, B will choose +2, allowing player A to force a win. In
other words, there is an action (picking +2) that will ensure A wins the game.
                                                           CS2109S Midterm Assessment

Under the assumption that players may play sub-optimally (the alternative
assumption), there are two possible outcomes: either player A can force a win if player
B plays sub-optimally, or the game will result in a draw if player B plays optimally. Thus,
the correct answer is e.
<!-- explanation:end -->

## q4D [easy]

**Question 4D [1 mark]**

Which sequences guarantee a Player B victory? Select all that is/are true.

   a. A: +2 → B: +2
   b. A: +2 → B: +5
   c. A: +2 → B: +2 → A: +2
   d. A: +5
   e. None of the above
                                                           CS2109S Midterm Assessment

<!-- answer:start -->
exact
D
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** d (main assumption) or e (alternative assumption)

4D. d (main assumption) or e (alternative assumption)

If we assume that player B plays optimally (main assumption), then only +5 will lead to
Player B winning. Otherwise (alternative assumption), then even with +5, B may take +2
which results in a draw.
<!-- explanation:end -->

## q4E [easy]

**Question 4E [1 mark]**

Which of the following action(s) should the first player take? Select all that

is/are true.

   a. +2
   b. +5
   c. All actions result in losing the game
   d. All actions result in a draw
   e. Game continues indefinitely

Consider the following game tree for a two-player game where alpha-beta pruning is
applied.

Figure: Game tree for Part 4 Adversarial search - Alpha-Beta pruning.

Note: The symbol ▲ represents the max player’s turn, while ▼ indicates the min
player’s turn.

<!-- answer:start -->
exact
A
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** a

4E. a

Between the option of losing and draw, the player will choose draw.
<!-- explanation:end -->

## q4F [hard]

**Question 4F [4 marks]**

Suppose we traverse this tree using (depth-first) alpha-beta pruning from

left to right. Select all the link(s) that would be pruned by alpha-beta pruning algorithm.
Select only the links that are directly pruned and not those that are indirectly pruned
because they are in a subtree of a pruned link.

Which of the following link(s) is/are pruned? Select all that is/are true.

   a. A
   b. B
   c. C
   d. D
   e. E
   f. F
   g. G
   h. H
   i. I
   j. J
   k. K
   l. L
   m. M
   n. N
   o. None of the above
                                                           CS2109S Midterm Assessment

<!-- answer:start -->
exact
F
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** f

4F. f
                                                                    CS2109S Midterm Assessment
<!-- explanation:end -->

## q5A [medium]

**Question 5A [2 marks]**

What is the entropy of the Hiring Decision (yes/no) in the table, rounded to

two decimal places?

<!-- answer:start -->
exact
0.86
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** 0.86

*         *    '         '
Explanation: − 6 log ' O6P − 6 log ' (6) = 0.863.
<!-- explanation:end -->

## q5B [medium]

**Question 5B [2 marks]**

What is the information gain of selecting “Experience” as the root node of

the Hiring Decision (yes/no) in the table, rounded to two decimal places?
                                                         CS2109S Midterm Assessment

<!-- answer:start -->
exact
0.47
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** 0.47

5B. 0.47

The remainder when splitting according to experience is remainder(Exp)=
  .             (                  (    '           '        .      .
− 6 1 log ' (1) − 6 1 log ' (1) − 6 [− ( log ' O(P − ( log ' O(P\ = 0 + 0 + 0.39355. Hence the
information gain is 0.863 − 0.39355 = 0.4695 = 0.47.
<!-- explanation:end -->

## q5C [medium]

**Question 5C [2 marks]**

Given that Experience has the highest information gain, you build the

remaining decision tree given the context information. According to this decision tree,
what is the Hiring Decision for the following candidate?

Candidate Information: Experience: < 1 year / Interview: Good / Potential: Mid

   a) Yes
   b) No
   c) No/Yes

<!-- answer:start -->
exact
B
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** b

5C. b

The decision tree is as follows:

                               Experience

                    >3 years       1-3years         <1 year

           Yes (1/1)               Yes (3/3)                     Potential

                                                          mid                  high

                                               No (1/1)                      No (1/2) Yes (1/2)

Experience <1 year and Potential mid leads to No for the decision.
<!-- explanation:end -->

## q5D [medium]

**Question 5D [2 marks]**

Please prune the decision tree from the previous question ensuring that

each leaf node contains at least 2 training data points. According to your pruned
decision tree, what is the Hiring Decision for the following candidate?

Candidate Information: Experience: <1 year / Interview: Good / Potential: Mid

   a) Yes
   b) No
   c) No/Yes
                                                                        CS2109S Midterm Assessment

<!-- answer:start -->
exact
A
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** a

5D. a

Explanation: Everything is pruned because the leaf node Experience >3 years only has 1
training points. Based on the majority at that level, the decision is Yes.
                                                                CS2109S Midterm Assessment
<!-- explanation:end -->

## q6A [medium]

**Question 6A [2 marks]**

Consider the following housing data.

After standardization of both features, which data set will you have?

Hint: The problem can be solved without calculation.

        a) A.
        b) B.
        c) C.
                                                                             𝑤7

<!-- answer:start -->
exact
C
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** c

C is the correct standardized data. Without calculation, we can rule out A, as the two
features are not on the same scale, a key aspect of standardization. The two features
are on even more distinct scales in A. We can rule out B based on the absence of
negative values: Standardization subtracts the mean (and divides by the standard
deviation) and for these data we would expect that some training points are smaller
than the mean hence the standardized data should have examples with negative
feature values.
<!-- explanation:end -->

## q6B [hard]

**Question 6B [4 marks]**

Take the linear regression hypothesis ℎ(𝑥) = 𝑤7 + 𝑤. 𝑥, where ^𝑤 _ =

                                                                                         .
 0.5
^ _. We have the data points 𝑥 (7) = 0, 𝑦 (7) = 0 and 𝑥 (.) = 2, 𝑦 (.) = 1, and the loss
  1
            .                     ' .                      '
function is ' `ℎ`𝑥 (7) a − 𝑦 (7) a + ' `ℎ`𝑥 (.) a − 𝑦 (.) a . Perform a single update to the vector
 𝑤7
^𝑤 _ using the gradient descent update rule with the learning rate 1/4. The resulting
   .
         𝑤7,5&9
weights ^𝑤       _ are given by which of the options?
           .,5&9

       −1/2
   a) c      d
        1/4
         0
   b) c      d
       −1/4
         0
   c) c      d
       −1/2
       1/2
   d) c    d
       1/4
                                                         CS2109S Midterm Assessment

        0
   e) c    d
       1/2
       −1/4
   f) c      d
        1/2
        0
   g) c    d
       1/4
       1/4
   h) c    d
       1/2
   i) None of the above.

<!-- answer:start -->
exact
G
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** g

.                             '   .                '
The loss function is 𝐽(𝑤) = ' `ℎ`𝑥 (7) a − 𝑦 (7) a + ' `ℎ`𝑥 (.) a − 𝑦 (.) a . First, note that
ℎ`𝑥 (7) a − 𝑦 (7) = 0.5 and ℎ`𝑥 (.) a − 𝑦 (.) = 0.5 + 2 − 1 = 1.5. The partial derivatives are
 C                                                                                                    C
C9
   𝐽(𝑤) = `ℎ`𝑥 (7) a − 𝑦 (7) a + `ℎ`𝑥 (.) a − 𝑦 (.) a = 0.5 + 1.5 = 2 and C9 𝐽(𝑤) =
     !                                                                                                    "
         (7)         (7)        (7)            (.)         (.)        (.)
`ℎ`𝑥           a−𝑦         a𝑥         + `ℎ`𝑥         a−𝑦         a𝑥         = 0 + 1.5 ∗ 2 = 3. The gradient update step
is

                                       C
 𝑤7,5&9      𝑤7           𝐽(𝑤)                    𝑤7,5&9
                      C9!                                     0.5 . 2    0.5
^𝑤      _ = ^𝑤 _ − 𝛾 o C        p, which becomes ^𝑤      _ = ^ _−)^ _ = ^ _−
  .,5&9       .           𝐽 (𝑤 )                   .,5&9       1    3     1
                      C9                   "
   0.5     0
^      _=c    d.
  0.75    1/4
<!-- explanation:end -->

## q6C [medium]

**Question 6C [2 marks]**

You are given a data set that consists of 4K images, where each pixel of

every image has a grayscale value 𝑔 ∈ [0,1]. You create a feature vector 𝑥, where each
feature corresponds to the greyscale value of a pixel. Additionally, each image has an
associated real-valued target 𝑦. You plan to use a linear regression model ℎ(𝑥), and you
aim to minimize the MSE loss to predict the target value for new images. Given these
choices, select true statements about the regression and learning algorithms?

   a) The problem setting implies that the normal equation will be the preferred
      method over gradient descent.
   b) Since this problem has uneven features, min-max scaling will improve the
      learning.
   c) Stochastic gradient descent will take the most direct path to the optimal
      solution.
   d) Since we transform features before applying linear regression, the model is
      overparameterized.
   e) None of the statements are true.

<!-- answer:start -->
exact
E
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** e

6C. e

The problem setting implies that gradient descent will be the preferred method over the
normal equation, because we have many features (4K implies around 8 million pixels).
In lecture we have discussed that solving a linear system (matrix inversion) comes at a
cost of 𝑑 ( hence is prohibitive.

The features are even [0,1], hence min-max scaling will not change much.

Stochastic gradient descent takes a randomized path to the solution. Gradient descent
takes the path in the greatest decrease of the function hence is the most direct path.

The sentence “Since we transform features before applying linear regression, the
model is overparameterized.” is non-sensical in this problem.
Hence, none of the statements are true.
<!-- explanation:end -->

## q6D [medium]

**Question 6D [2 marks]**

For a linear regression problem, you decide to use an unusual loss

function. The loss function used for each data point is 𝑓(ℎ(𝑥) − 𝑦), where 𝑓(𝑧) is
graphed below for 𝑧 = ℎ(𝑥) − 𝑦. You average the loss over the data set in the same way
as the MSE. Given this choice, what is the best learning algorithm to use? (Hint: Best is
here considered in terms of speed and finding a good minimum.)
                                                           CS2109S Midterm Assessment

   a) Normal equation.
   b) Gradient descent.
   c) Stochastic gradient descent.
   d) Decision tree learning.

<!-- answer:start -->
exact
B
C
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** b or c

The loss function is non-convex. There are line segments between two points that are
below the function. We discussed the property of Stochastic Gradient Descent to be
                                                         CS2109S Midterm Assessment

able to overcome being stuck in local minima/saddle points arising in non-convex
landscapes in class. From these two observations, c) is the main correct answer.

While the function is non-convex, it is in fact quasi-convex. For quasi-convex functions,
a version of gradient descent can be used in certain cases. It is called normalized
gradient descent, see https://arxiv.org/abs/1507.02030 and references therein. Hence,
we also allow the choice of option b).

Note on overshooting the minimum: We have introduced in class the concept of the
learning rate as a hyperparameter and choosing a small learning rate (with the tradeoff
of slow convergence). In addition, tutorial 4 mentions a learning rate scheduler, which
can be used to decrease the learning rate during gradient descent. The question asks
about the best method where “Best is here considered in terms of speed and finding a
good minimum”. A small learning rate during the later stage of gradient descent will
prevent overshooting by large amounts, and will find a good enough minimum.

Option a) cannot be used as we cannot derive a normal equation, due to the presence
of the function 𝑓. Option d) applies to classifiers with a decision tree model only.
<!-- explanation:end -->

## q6E [medium]

**Question 6E [2 marks]**

Let us be given 4 key attributes of a smartphone. You would like to predict

the failure probability of a phone given a data set of phone failures. What is the best
hypothesis class to choose?
   a) ℎ9 (𝑥) = 𝑤7 𝑥7 + 𝑤. 𝑥. + 𝜎(𝑤' 𝑥' + 𝑤( 𝑥( + 𝑤) 𝑥) )

   b) ℎ9 (𝑥) = 𝑤. 𝑥. + 𝑤' 𝑥' + 𝑤( 𝑥( + 𝑤) 𝑥)

   c) ℎ9 (𝑥) = 𝑤7 + 𝑤. 𝑥. + 𝑤' 𝑥' + 𝑤( 𝑥( + 𝑥)

   d) ℎ9 (𝑥) = 𝑤7 + 𝑤. 𝑥. + 𝑤' 𝑥' + 𝑤( 𝑥( + 𝑤) 𝑥)

   e) ℎ9 (𝑥) = 𝜎(𝑤7 𝑥7 + 𝑤. 𝑥. ) +𝜎(𝑤' 𝑥' + 𝑤( 𝑥( + 𝑤) 𝑥) )

   f) ℎ9 (𝑥) = 𝜎(𝑤7 + 𝑤. 𝑥. + 𝑤' 𝑥' + 𝑤( 𝑥( + 𝑥) )

   g) ℎ9 (𝑥) = 𝜎(𝑤7 𝑥7 + 𝑤. 𝑥. + 𝑤' 𝑥' + 𝑤( 𝑥( + 𝑤) 𝑥) )

   h) ℎ9 (𝑥) = 𝜎(𝑤. 𝑥. + 𝑤' 𝑥' + 𝑤( 𝑥( + 𝑤) 𝑥) )

<!-- answer:start -->
exact
G
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** g

6E. g

From the problem statement, we require a logistic model with 4 features. In addition,
the bias improves a model. Adding the dummy feature 𝑥7 gives the hypothesis ℎ9 (𝑥) =
𝜎(𝑤7 𝑥7 + 𝑤. 𝑥. + 𝑤' 𝑥' + 𝑤( 𝑥( + 𝑤) 𝑥) ).
<!-- explanation:end -->

## q6F [medium]

**Question 6F [2 marks]**

Consider a logistic regression model for multi-class classification with

three classes: Cat, Dog, and Rabbit. We are given the following weight vectors for “One
vs. Rest” classifiers, where the ℎ:/< (𝑥) represents the probability of the class A instead
of B and contains the weight vector 𝑤:/< . The weight vectors for each classifier includes
the bias term as the first element in each weight vector. For example, -1 is the bias for
𝑤=#>>4"/?#" .

                                                   0
                                   𝑤@AB/=#>>4" = k0.4l,
                                                  0.1
                                                          CS2109S Midterm Assessment

                                                 −1
                                 𝑤=#>>4"/?#" = k 0.2 l,
                                                −0.4
                                               2
                                  𝑤?#"/@AB = k−0.5l.
                                              0.3

                                                        3
Let the decision threshold be 0.5. Given the input 𝑥 = ^ _, determine which class the
                                                        2
model predicts.

   a) Dog
   b) Cat
   c) Rabbit
   d) None of the above.
                                                                                         CS2109S Midterm Assessment

<!-- answer:start -->
exact
B
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** b

6F. b

Computing the dot products (using the dummy feature) gives:

         0 D 1                               −1 D 1
𝑎@/= = k0.4l k3l = 0.8 + 0.2 = 1. , 𝑎=/? = k 0.2 l k3l = −1 + 0.6 − 0.8 = −1.2.
        0.1 2                               −0.4 2
             2 D 1
𝑎?#"/@AB = k−0.5l k3l = 2 − 1.5 + 0.6 = 1.1.
            0.3    2

Since the sigmoid is monotonically increasing, these dot products tell us that Dog
obtains 1 vote and Cat obtains 2 votes, hence Cat is the answer.
                  CS2109S Midterm Assessment

−END OF PAPER −

      28
<!-- explanation:end -->
