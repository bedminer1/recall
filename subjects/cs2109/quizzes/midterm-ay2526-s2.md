# AY2025/26 Semester 2 Midterm

Keep `CS2109S+AY2025-26+Sem+2+-+Midterm+-+Solution.pdf` open for the shared context, tables, and figures. Answer choices are reproduced below where present.

## q1A [easy]

**Question 1A [1 mark]**

Is it true that the search formulation results in a state space where a state is

reachable through more than one sequence of actions? Note: we do not care about the
search algorithms in this question since we are asking about the state space, not the
search tree.

   a. Yes
   b. No

<!-- answer:start -->
exact
A
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** a

Yes. Since toggle operations commute (order doesn't matter) and are self-inverse, the
same state can be reached by dieerent sequences of actions.

Example: Starting from state S, Toggle(A) → Toggle(B) and Toggle(B) → Toggle(A) both reach
the same state, but via dieerent action sequences. More generally, any permutation of a set
of toggles leads to the same state.
<!-- explanation:end -->

## q1B [easy]

**Question 1B [1 mark]**

Does the search formulation result in a unique optimal path if there is one?

   a. Yes
   b. No

<!-- answer:start -->
exact
B
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** b

1B. b

No. As shown in 1A, since toggle operations commute, any permutation of the optimal set
of toggles is also an optimal path with the same cost. So there can be multiple optimal
paths of the same length.
<!-- explanation:end -->

## q1C [easy]

**Question 1C [1 mark]**

For an arbitrary ﬁxed graph and initial state, is a solution guaranteed to exist?

   a. Yes
   b. No

<!-- answer:start -->
exact
B
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** b

1C. b

Consider N = 2 servers connected to each other, initial state = [1, 0], goal state = [1,1].

From any state, you can only ever reach one other state: [1,0] ↔ [0,1]. The states [0,0] and
[1,1] are completely unreachable.

Since the goal [1,1] is unreachable, no solution exists.
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
**Supplied answer:** c

1D. c

None of the above.

Without visited memory, a search can revisit states inﬁnitely. Since Toggle is self-inverse,
an algorithm can cycle (e.g., Toggle(A), Toggle(A), Toggle(A)... forever).

   •    If no solution exists, BFS/UCS keep expanding forever and never terminate.

   •   DFS can follow a cycle forever (e.g., keep toggling the same server) and never
       terminate.
<!-- explanation:end -->

## q1E [medium]

**Question 1E [2 marks]**

Which of the following search (without visited memory) algorithm(s) can we

employ such that the search always ﬁnds an answer (valid solution) if a solution exists?
Select all that apply.

   a. Breadth-First Search (BFS)
   b. Depth-First Search (DFS)
   c. Uniform-Cost Search (UCS) with constant cost C > 0
   d. Depth-Limited Search (DLS) with DFS and max-depth N
   e. None of the above

<!-- answer:start -->
exact
ACD
A C D
A,C,D
A, C, D
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** a, c, d

1E. a, c, d

Key properties of the problem:

   •   State space is ﬁnite with 2^N states, so with visited memory all algorithms
       terminate and are solution-aware

   •   All toggle actions have uniform cost, so path cost = number of toggles

   •   Any solution (if one exists) can be reduced to at most N toggles, since toggling the
       same server twice cancels out — so the optimal solution always lies at depth ≤ N

BFS: BFS explores the search tree level by level. It will ﬁnd a solution at depth d ≤ N.

DFS: Without visited memory, DFS can get trapped in an inﬁnite cycle.

UCS (constant cost C > 0): Equivalent to BFS.

DLS with DFS and max-depth N: This limits the search to depth N. Since optimal solutions
require at most N toggles, a depth limit of N is sueicient to ﬁnd a solution if one exists. The
depth bound prevents inﬁnite cycling, guaranteeing termination.
<!-- explanation:end -->

## q1F [medium]

**Question 1F [2 marks]**

Suppose that we use search with visited memory. Which of the following

search algorithm(s) is/are the best for the problem? Select all that apply.

Best means the algorithm(s) should be complete, optimal, eeicient (in terms of big O
worst-case space and time complexity), and aware if there is no solution.

   a. Depth-First Search (DFS)
   b. Uniform-Cost Search (UCS) with constant cost C > 0
   c. Depth-Limited Search (DLS) with BFS and max-depth N
   d. Iterative Deepening Search (IDS) with DFS
   e. None of the above

<!-- answer:start -->
exact
C
BC
B C
B,C
B, C
BCD
B C D
B,C,D
B, C, D
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** c or b,c or c,d, or b,c,d – see explanation below

1F. c or b,c or c,d, or b,c,d – see explanation below

DFS: Terminates and is solution-aware with visited memory, but not optimal — it ﬁnds any
path to the goal rather than the shortest.

UCS (constant cost C > 0): With uniform costs, UCS behaves identically to BFS, expanding
nodes in order of depth. It is complete, optimal (guaranteed to ﬁnd the minimum number of
toggles), and runs in exponential time and space. If we also compare the time complexity
of a priority queue, we ﬁnd that UCS has an extra log(2^N) factor.

DLS with BFS and max-depth N: Since any solution exists at depth ≤ N, BFS within depth
N is guaranteed to ﬁnd the shallowest — and therefore optimal — solution. Time and space
complexity is also exponential.

IDS with DFS: Complete and optimal in general, and normally attractive for its polynomial
space usage. However, with visited memory the visited set already requires exponential
space, nullifying its space advantage. It also repeats work across iterations, making it
strictly worse than UCS/BFS in both time and space. However, in terms of big O complexity,
it’s the same: O(b^d).

Answer: DLS with BFS and maximum depth N. If we disregard the priority queue time
complexity, we can also consider UCS (constant cost C > 0), as we typically do in this
course. We we will accept both answers. We will also accept choosing or not choosing IDS
because we have never explained the big O time complexity of IDS.

Correct:

   •   B. UCS (optional)
   •   C. DLS
   •   D. IDS (optional)

Part 2: Informed Search
<!-- explanation:end -->

## q2A [medium]

**Question 2A [2 marks]**

Consider a single robot 𝑖 at position (𝑥! , 𝑦! ). For this question, there are no other robots.

Consider the problem of moving the robot to its goal location. Select all the admissible
heuristics for the number of moves to the goal location that apply.

       a) ℎ1!
       b) ℎ2!
       c) ℎ3!
       d) ℎ4!
       e) None of the above.

<!-- answer:start -->
exact
ABC
A B C
A,B,C
A, B, C
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** a, b, and c.

The true cost of the single-robot problem is the Manhattan distance |𝑥! − 𝑛 + 𝑖 − 1| + |𝑦! − 𝑛|.
We have that ℎ1! ≤ ℎ2! ≤ ℎ3! , and heuristics 1, 2, and 3 are admissible. Heuristic ℎ4! is not
admissible, since if the robot is currently at point (𝑛 − 𝑖, 𝑛 − 1) it takes 2 steps to reach the goal, but
the heuristic will give (1 + 1)! = 4.
<!-- explanation:end -->

## q2B [medium]

**Question 2B [2 marks]**

Consider a single robot 𝑖 at position (𝑥! , 𝑦! ). There are 𝑛 − 1 other robots and

their positions are ﬁxed. The action only moves robot 𝑖. Select all the admissible heuristics
for the number of moves to the goal location that apply.

       a) ℎ1!
       b) ℎ2!
       c) ℎ3!
       d) ℎ4!
       e) None of the above.

<!-- answer:start -->
exact
A
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** a.

2B. a.

Another robot may be at the target position. In that case the true cost is inﬁnity and all heuristics
are admissible. However, admissibility must hold for all states. Assume from now that none of the
n-1 robots is at the target position. Assume the main robot is at (𝑛 − 𝑖 + 1, 𝑛 − 2), so it is in the goal
left to right direction, but not yet at the goal in the top to bottom direction. The heuristic estimates
are ℎ1" = 1, ℎ2" = ℎ3" = ℎ4" = 2. However, the true cost is 1 since the robot can jump over the
other robot with cost 1 to reach the goal. Only ℎ1" is admissible.
<!-- explanation:end -->

## q2C [medium]

**Question 2C [2 marks]**

Consider the same setting as Question 2B. Select all the consistent

heuristics for the number of moves to the goal location that apply.

       a) ℎ𝐶1!
       b) ℎ𝐶2!
       c) ℎ𝐶3!
       d) ℎ𝐶4!
       e) ℎ𝐶5!
       f) None of the above.

<!-- answer:start -->
exact
AB
A B
A,B
A, B
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** ab.

2C. ab.

We can rule out the inadmissible heuristics ℎ𝐶3! , ℎ𝐶4! , ℎ𝐶5! .

The main robot can jump over another robot at cost 1 to get closer to the goal by two positions.
Considering the y direction only: ℎ𝐶" (𝑥" , 𝑦" ) ≤ 1 + ℎ" (𝑥" , 𝑦" + 2) ↔ ℎ𝐶" (𝑥" , 𝑦" ) − ℎ" (𝑥" , 𝑦" + 2) ≤ 1 .
This inequality is satisﬁed by ℎ𝐶1! , ℎ𝐶2! . Recall that only a jump over a single robot is possible.
<!-- explanation:end -->

## q2D [medium]

**Question 2D [2 marks]**

Consider the problem of shueling 𝑛 robots as described in the Context.

Consider an admissible heuristic ℎ3! for the single-robot setting as in Question 2A. Select
all the admissible heuristics for the 𝑛-robots problem that apply.

          a) Sum: ∑%!&# ℎ! .
          b) Max: max {ℎ# , … , ℎ% }.
          c) Min: min {ℎ# , … , ℎ% }.
          d) None of the above.

<!-- answer:start -->
exact
C
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** c.

2D. c.

Despite that any robot i could help another robot to achieve the goal faster, that robot i must at
least go to its goal position which is h3_i away. Hence, the Min of the admissible heuristic is
admissible.

A counterexample for the Max is given as follows, see picture. Here the max is 3 as robot 4 is 3 steps
away from the Goal G4. However, the true cost is 2: First, robot 4 jumps over robot 3 at cost 1. Then
Robot 3 and 4 move to their respective goals at cost 1. Hence the total cost is 2.

Recall again that the context states that at one timestep every robot can move. Hence in a single
time step both robots move at cost 1.

 🤖5     G4     G3     🤖2     🤖1

               🤖3

               🤖4

Part 3: Local Search
<!-- explanation:end -->

## q3A [hard]

**Question 3A [4 marks]**

Given the problem, which of the following local search formulation(s) is/are

reasonable for hill climbing algorithm taught in lecture? Select all that apply.
Notes:

     •   In this context, we consider the formulation reasonable if hill-climbing with an
         inﬁnitely large number of random restarts can return the optimal sequence of
         actions for the problem if the solution exists.
     •   At every restart, the initial state and the output of the successor function may dieer
         from the previous run if there is randomness involved.
     •   Hill climbing picks a successor based on the highest evaluation function’s value

a)

     •   State: A binary vector of length 𝑁, representing the current On/Off status of each
         server.
     •   Initial State: The given initial configuration of the network.
     •   Goal Test: Is the vector [1,1, … ,1] all Online?
     •   Evaluation Function: Number of servers currently Online.
     •   Successors: Generate all states reachable by toggling exactly one switch from the
         current state.

b)

     •   State: A binary vector of length 𝑁, where the 𝑖-th bit represents whether switch 𝑖 is
         included in the solution set.
     •   Initial State: A random binary vector of length 𝑁.
     •   Goal Test: Does applying this set of switches result in all servers being Online?
     •   Evaluation Function: (−𝐴 × (Number of Offline servers resulting from this set) −
         𝐵 × (Number of 1s in the state vector)), where 𝐴 and 𝐵 are parameters, 𝐴 ≫ 𝐵 > 0.
     •   Successors: Generate all vectors that differ from the current vector by exactly one
         bit (add or remove one switch).

c)

     •   State: A list of integers representing the sequence of switches pressed so far
         (e.g., [3, 1, 5]).
     •   Initial State: An empty list [].
     •   Goal Test: Does applying this sequence of switches result in all servers being
         Online?
     •   Evaluation Function: Number of servers currently Offline.
     •   Successors: Append any integer 𝑘 ∈ [1, 𝑁] to the end of the current list.

d)

     •   State: A binary vector of length 𝑁, where the 𝑖-th bit represents whether switch 𝑖 is
         included in the solution set.
     •   Initial State: The given initial configuration of the network.
     •   Goal Test: Does applying this set of switches result in all servers being Online?
     •   Evaluation Function: 0 if all servers are Online; otherwise −∞.
     •   Successors: Generate all vectors that differ from the current vector by exactly one
         bit.

e) None of the above

<!-- answer:start -->
exact
B
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** b

a)
Only tracks the current server state, not which switches were pressed. When the algorithm
terminates, it just reports the ﬁnal server state.

b)

The state encodes which switches are included in the solution set (each server toggled at
most once). The evaluation function penalizes both oeline servers (heavily, via A) and the
number of switches used (lightly, via B, with A ≫ B > 0).

     •   The dominant term (-A × oeline servers) drives the search toward valid solutions

     •   The secondary term (-B × switches used) breaks ties in favor of fewer toggles,
         directly encoding optimality

     •   Since the initial state is random and successors ﬂip one bit, with inﬁnite restarts the
         search can reach any binary vector of length N, including the optimal solution set

     •   The state space is ﬁnite (2^N), so the global optimum is reachable

Formulation is reasonable.

c)

The evaluation is the number of o4line servers but hill climbing chooses highest value, so it
is optimizing in the wrong direction.

d)

The evaluation is 0 only at goal and −∞ otherwise, so hill climbing will directly terminate in
one step if the goal is >1 step away since all states are equally bad, i.e., value = −∞. Since
the initial state is ﬁxed based on the problem, inﬁnite restarts will not help solve problems
whose goal is >1 step away.

Part 4: Adversarial Search
Minimax
<!-- explanation:end -->

## q4A [easy]

**Question 4A [1 mark]**

How many terminal nodes are there in the game tree generated by the

algorithm speciﬁed in the context?

<!-- answer:start -->
exact
3
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** 3

4A. 3
<!-- explanation:end -->

## q4B [easy]

**Question 4B [1 mark]**

How many nodes in the full game tree generated by Minimax without cutoe

exceed the cutoe depth speciﬁed in the context? Note: please write “inﬁnity” if the depth is
inﬁnite.

<!-- answer:start -->
exact
9
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** 9

4B. 9
<!-- explanation:end -->

## q4C [easy]

**Question 4C [1 mark]**

What is the game tree's maximum depth for Minimax without cutoe? Note:

please write “inﬁnity” if the depth is inﬁnite.

<!-- answer:start -->
exact
5
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** 5

4C. 5
<!-- explanation:end -->

## q4D [easy]

**Question 4D [1 mark]**

Suppose Commander A Boost the ﬂag to position +1 on their ﬁrst turn. Which

commander can win the game if they both play optimally in the subsequent turns until the
terminal state?

   a. A
   b. B
   c. Game is draw
   d. Game continues indeﬁnitely
   e. None of the above

<!-- answer:start -->
exact
B
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** b

4D. b

Notice that the question asks for the outcome when the game is played until termination
(not cutoe).

“… if they both play optimally in the subsequent turns until the terminal state”

As seen on the full minimax game tree above, Commander B wins.
<!-- explanation:end -->

## q4E [easy]

**Question 4E [1 mark]**

Let the following be sequences of play from the ﬁrst turn (i.e., beginning of the

game). Which sequence(s) give(s) the highest value to Commander A according to the
output of the algorithm speciﬁed in the context? Select all that is/are true.

   a. A: Push → B: Boost → A: Invert → B: Boost
   b. A: Boost → B: Push → A: Boost → B: Invert
   c. A: Push → B: Boost → A: Invert → B: Push
   d. A: Invert → B: Push → A: Boost → B: Push
   e. None of the above

<!-- answer:start -->
exact
CD
C D
C,D
C, D
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** c, d

4E. c, d

Based on the minimax game tree with cutoe 4 above:

a) A: Push → B: Boost → A: Invert → B: Boost: 0

b) A: Boost → B: Push → A: Boost → B: Invert: -10

c) A: Push → B: Boost → A: Invert → B: Push: 1

d) A: Invert → B: Push → A: Boost → B: Push: 1

c and d give the highest value of 1.
<!-- explanation:end -->

## q4F [easy]

**Question 4F [1 mark]**

Which of the following action(s) can the Commander A take on their ﬁrst turn

(i.e., beginning of the game) to gain the maximum value based on the output of the

algorithm speciﬁed in the context? Assume all players play optimally in the subsequent
turns according to the algorithm speciﬁed in the context. Select all that is/are true.
Note: If there are multiple actions that Commander A can take, select all the actions. If all
actions have the same utility, do not select any action and select option d.

   a. Push
   b. Boost
   c. Invert
   d. All actions result in the same utility
   e. None of the above

Solution
During the exam, we clariﬁed two things:

   •   Bound shrink occurs simultaneously with the move. For instance, if the current
       bound is 4 and A move to position 3: the bound shrinks to 3, hence player A wins.
   •   Bound shrink occurs from both sides, e.g., +-6 to +-5.

Minimax with cutoe depth 4:

Full Minimax:

4A. 3

4B. 9

4C. 5

4D. b

Notice that the question asks for the outcome when the game is played until termination
(not cutoe).

“… if they both play optimally in the subsequent turns until the terminal state”

As seen on the full minimax game tree above, Commander B wins.

4E. c, d

Based on the minimax game tree with cutoe 4 above:

a) A: Push → B: Boost → A: Invert → B: Boost: 0

b) A: Boost → B: Push → A: Boost → B: Invert: -10

c) A: Push → B: Boost → A: Invert → B: Push: 1

d) A: Invert → B: Push → A: Boost → B: Push: 1

c and d give the highest value of 1.

4F. a

Based on the minimax game tree with cutoe 4 above:

Push → 0 (maximum value)

Boost → −10

Invert → −10

Alpha-Beta Pruning
Context
Consider the following game tree for a two-player game where alpha-beta pruning is
applied.

Note: The symbol ▲ represents the max player’s turn, while ▼ indicates the min player’s
turn.

<!-- answer:start -->
exact
A
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** a

4F. a

Based on the minimax game tree with cutoe 4 above:

Push → 0 (maximum value)

Boost → −10

Invert → −10

Alpha-Beta Pruning
<!-- explanation:end -->

## q4G [hard]

**Question 4G [4 marks]**

Suppose we traverse this tree using (depth-ﬁrst) alpha-beta pruning from

right to left. Select all the link(s) that would be pruned by alpha-beta pruning algorithm.
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
    o. O
    p. P
    q. Q
    r. R
    s. S
    t. None of the above

<!-- answer:start -->
exact
FGIJR
F G I J R
F,G,I,J,R
F, G, I, J, R
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** f, g, i, j, r

4G. f, g, i, j, r

Part 5: Decision trees
You work at a real-estate company and have been tasked with deciding on investing in an
apartment. Leveraging your expertise, you create a decision tree based on previous
investing data given in Table 1.

       Pool              Quality          Location         Decision
    1  Yes               Mid              A                Yes
    2  Yes               High             B                No
    3  No                Mid              C                Yes
    4  Yes               High             B                Yes
    5  No                High             A                No
    6  Yes               Mid              D                Yes
    7  No                Low              C                No
    8  Yes               Mid              B                Yes
    9  No                High             C                No
    10 Yes               High             D                No
    11 No                Low              B                No
    12 Yes               High             C                Yes

Consider the decision tree learning algorithm using information gain.

In the case of ties, the preference is Location > Quality > Pool. The default value at the
initial DTL call is “No”. The majority decision function outputs “Yes” in case of a tie.

Based on this description, answer questions 5A-5F.

Notes:

•    If a decision tree does not check an attribute, it is a zero-depth decision tree.
•    The entropy for a given probability distribution 𝑝! , for 𝑖 = 1, . . . , 𝑛 is given by:
                                                      %

                                   𝐸𝑛𝑡𝑟𝑜𝑝𝑦 = − V 𝑝! log " (𝑝! ).
                                                     !&#

•    The conditional entropy of a random variable 𝑌 given 𝑋…
<!-- explanation:end -->

## q5A [easy]

**Question 5A [1 mark]**

What is the entropy of the Decision (yes/no) given the data in the table,

rounded to two decimal places?

<!-- answer:start -->
exact
1
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** 1

5A. 1

                       #          #    #       #
Explanation: − $! log ! 8$!9 − $! log ! ($!) = 1.
<!-- explanation:end -->

## q5B [easy]

**Question 5B [1 mark]**

What is the information gain of selecting “Location” as the root node of the

Decision (yes/no) in the table, rounded to two decimal places?

<!-- answer:start -->
exact
0
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** 0

5B. 0

                                                                                   !         $ $            %         ! !
The conditional entropy when splitting according to Location is H(Y|L)= $! 𝐻 8! , !9 + $! 𝐻 8% , %9 +
%       ! !        !       $ $
     𝐻 8% , %9 + $! 𝐻 8! , !9 = 1. Hence the information gain is 0.
$!
<!-- explanation:end -->

## q5C [easy]

**Question 5C [1 mark]**

What is the information gain of selecting “Quality” as the root node of the

Decision (yes/no) in the table, rounded to two decimal places?

<!-- answer:start -->
exact
0.54
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** 0.54

5C. 0.54

                                                                               #         ! %            %           & %
The conditional entropy when splitting according to Quality is H(Y|Q)= $! 𝐻 8# , #9 + $! 𝐻 8% , %9 +
!       & !        $
$!
     𝐻 8! , !9 = ! ∗ 0.918 = 0.459. Hence the information gain is 1 − 0.459 = 0.541.
<!-- explanation:end -->

## q5D [easy]

**Question 5D [1 mark]**

What is the information gain of selecting “Pool” as the root node of the

Decision (yes/no) in the table, rounded to two decimal places?

<!-- answer:start -->
exact
0.20
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** 0.20

5D. 0.20
                                                                          '            ! (         (            % $
The conditional entropy when splitting according to Pool is H(Y|P)=            𝐻8 , 9+                  𝐻8 , 9 =
                                                                          $!           ' '         $!           ( (
0.80. Hence the information gain is 1 − 0.80 = 0.20.

5E. c or d.

Select attribute Quality. For value = High we obtain six samples. The other values are decisive.

       Pool                      Quality           Location    Decision
    2  Yes                       High              B           No
    4  Yes                       High              B           Yes
    5  No                        High              A           No
    9  No                        High              C           No
    10 Yes                       High              D           No
    12 Yes                       High              C           Yes

                                                         ! %
The base entropy is H(Y| Quality=High) = 𝐻 8# , #9 = 0.918.

                                                                                                                $
The conditional entropy when splitting according to Location is H(Y| Quality=High,L)= # 𝐻(1,0) +
!      $ $     !       $ $       $         %
    𝐻 8 , 9 + 𝐻 8 , 9 + 𝐻(0,1) = = 0.66. Hence, the information gain is 0.258.
#      ! !     #       ! !       #         #

                                                                                       %    ! !
     The conditional entropy when splitting according to Pool is H(Y|Quality=High,P)= # 𝐻 8% , %9 +
     !
     #
         𝐻(0,1) = 0.66. Hence, the information gain is 0.258.

     The decision tree is as…
<!-- explanation:end -->

## q5E [medium]

**Question 5E [2 marks]**

Build the decision tree given the context information up to maximum depth 2.

Select the attributes that are not tested anywhere in that decision tree. Select all that
apply.

    a) Location.
    b) Quality.
    c) Pool.
    d) All attributes are tested.

<!-- answer:start -->
exact
BC
B C
B,C
B, C
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** bc.

a) Pool: Yes. Quality: High. Location: A. -> No
   b) Pool: No. Quality: High. Location: B. -> Yes
   c) Pool: Yes. Quality: Mid. Location: C. -> Yes
   d) Pool: No. Quality: Low. Location A. -> No
<!-- explanation:end -->

## q5F [medium]

**Question 5F [2 marks]**

According to the decision tree built in Question 5E, consider the Decision for

the following examples.

Select the example(s) with a Yes decision. Select all that apply.

    a) Pool: Yes. Quality: High. Location: A.
    b) Pool: No. Quality: High. Location: B.
    c) Pool: Yes. Quality: Mid. Location: C.
    d) Pool: No. Quality: Low. Location A.
    e) None of the above.

Solution

5A. 1

                       #          #    #       #
Explanation: − $! log ! 8$!9 − $! log ! ($!) = 1.

5B. 0

                                                                                   !         $ $            %         ! !
The conditional entropy when splitting according to Location is H(Y|L)= $! 𝐻 8! , !9 + $! 𝐻 8% , %9 +
%       ! !        !       $ $
     𝐻 8% , %9 + $! 𝐻 8! , !9 = 1. Hence the information gain is 0.
$!

5C. 0.54

                                                                               #         ! %            %           & %
The conditional entropy when splitting according to Quality is H(Y|Q)= $! 𝐻 8# , #9 + $! 𝐻 8% , %9 +
!       & !        $
$!
     𝐻 8! , !9 = ! ∗ 0.918 = 0.459. Hence the information gain is 1 − 0.459 = 0.541.

5D. 0.20
                                                                          '            ! (         (            % $
The conditional entropy when splitting according to Pool is H(Y|P)=            𝐻8 , 9+                  𝐻8 , 9 =
                                                                          $!           ' '         $!           ( (
0.80. Hence the information gain is 1 − 0.80 = 0.20.

5E. c or d.

Select attribute Quality. For value = High we obtain six samples. The other values are decisive.

       Pool                      Quality           Location    Decision
    2  Yes                       High              B           No
    4  Yes                       High              B           Yes
    5  No                        High              A           No
    9  No                        High              C           No
    10 Yes                       High              D           No
    12 Yes                       High              C           Yes

                                                         ! %
The base entropy is H(Y| Quality=High) = 𝐻 8# , #9 = 0.918.

                                                                                                                $
The conditional entropy when splitting according to Location is H(Y| Quality=High,L)= # 𝐻(1,0) +
!      $ $     !       $ $       $         %
    𝐻 8 , 9 + 𝐻 8 , 9 + 𝐻(0,1) = = 0.66. Hence, the information gain is 0.258.
#      ! !     #       ! !       #         #

                                                                                       %    ! !
     The conditional entropy when splitting according to Pool is H(Y|Quality=High,P)= # 𝐻 8% , %9 +
     !
     #
         𝐻(0,1) = 0.66. Hence, the information gain is 0.258.

     The decision tree is as follows:

                                                    Quality

                          High (6)                    Mid: Yes (4)    Low: No (2)

                                                      ->Yes           ->No

                                  Location

              A: No (1)                B: Yes                           D: No (1)
                                                         C: Yes(1),
                                       (1),No (1)
              ->No                                       No(1)          ->No
                                       ->Yes
                                                         -> Yes
                                       (maj
                                                         (maj
                                       default)
                                                         default)

     By the problem statement, if a decision tree does not check an attribute, it is a zero-depth
     decision tree. This decision tree is depth 2; it is the intended solution decision tree.

     Alternative assumption:

     We allow the interpretation “First attribute is checked at depth zero”. Under that
     interpretation, we continue the location B, C cases.

          Location

                B: Yes               C: Yes(1),
                (1),No (1)           No(1)

              Pool                           Pool

Yes: Yes             No: No            Yes: Yes(1)        No: No(1)
(1), No (1)          rows ->                              ->No
                                       ->Yes
                     default ->
Maj -> Yes           maj ->
                     Yes

5E. bc.

   a) Pool: Yes. Quality: High. Location: A. -> No
   b) Pool: No. Quality: High. Location: B. -> Yes
   c) Pool: Yes. Quality: Mid. Location: C. -> Yes
   d) Pool: No. Quality: Low. Location A. -> No

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
Check the supplied solution for Question 5F in `CS2109S+AY2025-26+Sem+2+-+Midterm+-+Solution.pdf`.
<!-- explanation:end -->

## q6A [medium]

**Question 6A [2 marks]**

Using the given data, construct the hypothesis ℎ*       (𝑥) of polynomial

regression of degree 2. The initial weight value for the additional polynomial feature shall be

𝑤" = 1. Evaluate the hypothesis and the corresponding loss function for the data points.
Fill in the following blanks:
 2345
ℎ*    ;𝑥 (#)= [𝐵𝐿𝐴𝑁𝐾1]
 2345
ℎ*    ;𝑥 (")= [𝐵𝐿𝐴𝑁𝐾2]
 2345
ℎ*    ;𝑥 (1)= [𝐵𝐿𝐴𝑁𝐾3]

𝐽,-. (𝑤) [𝐵𝐿𝐴𝑁𝐾4]

                                                         2345

<!-- answer:start -->
exact
7,3,4,9.33
7, 3, 4, 9.33
7 3 4 9.33
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** 7, 3, 4, 9.33.

6A. 7, 3, 4, 9.33.
                     *+,-
The hypothesis is ℎ) (𝑥) = 𝑤& + 𝑤$ 𝑥$ + 𝑤! 𝑥$! . Since we have only a single feature and degree
2, there is only one possible way to write the model. Update the table as follows:

 Training               𝑥#                𝑥#"          𝑦
 point
     𝑥 (#)        -1                  1          1
     𝑥 (")        1                   1          -1
     𝑥 (1)        2                   4          2

To be able to evaluate the loss function, we need weights values. However, we have “initial
                               𝑤&
weights” for the linear model B𝑤 C and the “initial weight value” 𝑤! for the polynomial feature.
                                $
                                                                  𝑤&       4
Hence, we take these values to evaluate the hypothesis. I.e., D 𝑤$ E = F−2G.
                                                                 𝑤2       1
ℎ) H𝑥 ($) I = 4 − 2 ∗ (−1) + 1 ∗ 1 = 7

ℎ) H𝑥 (!) I = 4 − 2 ∗ 1 + 1 ∗ 1 = 3

ℎ) H𝑥 (0) I = 4 − 2 ∗ 2 + 1 ∗ 4 = 4

                                                                               *+,-
We evaluate the “corresponding” loss function, i.e., the loss function using ℎ)       (𝑥).

            $                                        $
𝐽123 (𝑤) = # [(7 − 1)! + (3 + 1)! + (4 − 2)! ] = # (36 + 16 + 4) = 9.33.

                                           *+,-
That we must use the loss function using ℎ) (𝑥) is clear from the word “corresponding”, and from
the fact that MSE (and other) loss functions are conceptually always dependent of the model under
consideration. Hence the deﬁnition in the context 𝐽123 (𝑤) does not ﬁx the loss function to the
linear regression model.
<!-- explanation:end -->

## q6B [medium]

**Question 6B [2 marks]**

You would like to learn the hypothesis ℎ*       (𝑥). Perform a single update to

            𝑤+
the vector i𝑤# j, where 𝑤" = 1, using the gradient descent (GD) update rule with the
            𝑤"
                                                                    𝑤+ ′             𝐵𝑙𝑎𝑛𝑘 1
learning rate 𝛾 = 1/2. Fill into the blank the resulting weights m𝑤# ′o as follows: i𝐵𝑙𝑎𝑛𝑘 2j
                                                                    𝑤" ′             𝐵𝑙𝑎𝑛𝑘 3

<!-- answer:start -->
exact
2,-2.33,-2
2, -2.33, -2
2 -2.33 -2
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** 2, -2.33, -2.

6B. 2, -2.33, -2.

The gradient vector is:
 𝑔&
        $    *+,-                          *+,-                        *+,-
D𝑔$ E = 0 OHℎ) H𝑥 ($) I − 𝑦 ($) I𝑥 ($) + Hℎ) H𝑥 (!)I − 𝑦 (!)I𝑥 (!) + Hℎ) H𝑥 (0) I − 𝑦 (0) I𝑥 (0) P =
 𝑔!

$
         1         1         1    $
                                    12       4
  Q(6) D−1E + (4) D1E + (2) D2ER = D 2 E = D2/3E.
0                                 0
         1         1         4      18       6

Using the initial weights as in 6A, we arrive at:

 𝑤& ′     𝑤&        𝑔&       4      4        2
                                 $
Q𝑤$ ′R = D𝑤$ E − 𝛾 D𝑔$ E = D−2E − D2/3E = D−2.33E.
                                 !
 𝑤! ′     𝑤!        𝑔!       1      6       −2
<!-- explanation:end -->

## q6C [medium]

**Question 6C [2 marks]**

The same context as for Questions 6A and 6B applies. Select potential

                         2345                      2345
problems when training ℎ*     (𝑥) and when using ℎ*     (𝑥) with new data. Consider the data
that are available and new data that the model will be used with. Select all that apply.

   a) In the context, there is not a sueicient amount of data points for the described
      polynomial regression.
   b) The new feature is linearly dependent on the original feature.
   c) The MSE is only convex and not strictly convex.
   d) You receive the knowledge about the possible range for          .
   e) The negative gradient of MSE points away from the minimum of the MSE.
   f) None of the above.

<!-- answer:start -->
exact
A
D
AD
A D
A,D
A, D
F
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** a or d or ad or f.

6C: a or d or ad or f.

    b) The new feature is not linearly dependent on the original feature, 𝑥! = 𝑥$ ∗ 𝑥$ .

    c) MSE is strictly convex here. Also, only convex would not be a problem.

    d) May lead to a problem. You receive the knowledge about the possible range for
                   . This can be a problem for training with more data points, because feature 2 is
    on very diXerent scale as feature 1, i.e., may be 100 times of feature 1. Gradient descent may
    converge very slowly. Also, the current training set does not include points in that range, hence
    the prediction may be very wrong for data points with, e.g., feature 1 =100.

    e) Not correct. The negative gradient points to the minimum. Not a problem.

Update: We allow the following options.

    •   a (optional)

    •   d (optional)

    •     f when suitable.

Explanation:

a: Indeed, under a machine learning lens, the 3 data points, while allowing a ﬁt, will probably lead
to overﬁtting. This can be a potential problem when using the model with new data.

d: We decided that the wording can be assumed to be too vague to imply a problem. While indeed a
large range for the feature may pose a problem (due to features with diXerent scales and training
example not seen in that range), Option d is not stated clear enough under the assumption that we
require stronger evidence.

6D. [-1,-2,1] or c[-1,-2,1], where c is a positive number.

Start from: 𝑥! ≥ 1 + 2𝑥$ for non-healthy patients. This equation implies that a patient is not-healthy
if 𝑥! − 2𝑥$ − 1 ≥ 0. The logistic function is 𝜎(𝑧) ≥ 0.5 for 𝑧 ≥ 0. The logistic model 𝜎(𝑥! − 2𝑥$ − 1)
with threshold 0.5 will output 1 for non-healthy patients. Hence, the weights of a possible logistic
model are given by 𝑤& = −1, 𝑤$ = −2, 𝑤! = 1. Any other…
<!-- explanation:end -->

## q6D [medium]

**Question 6D [2 marks]**

Let hospital patients be described by two features 𝑥# and 𝑥" . The hospital has

found a decision boundary for healthy and not-healthy patients given by 𝑥" = 1 + 2𝑥# ,
where the patient is not-healthy if 𝑥" ≥ 1 + 2𝑥# . You want to ﬁnd the logistic regression
                                   𝑤+
model given by three weights i𝑤# j, where 𝑤+ is for the oeset, 𝑤# is for the feature 𝑥# , and 𝑤"
                                   𝑤"
is for the feature 𝑥" . The logistic model with threshold 0.5 should output 1 if the patient is
not-healthy. Fill in the blanks with the weights of the logistic regression model.

𝑤+ = [𝐵𝐿𝐴𝑁𝐾1]

𝑤# = [𝐵𝐿𝐴𝑁𝐾2]

𝑤" = [𝐵𝐿𝐴𝑁𝐾3]

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
Check the supplied solution for Question 6D in `CS2109S+AY2025-26+Sem+2+-+Midterm+-+Solution.pdf`.
<!-- explanation:end -->

## q6E [medium]

**Question 6E [2 marks]**

Consider the following customer data. The customers are described by the

following features and a target variable.

        𝑥#              𝑥"                       𝑥1         𝑦
 1000             0.1              0.4                     𝑦 (#)
 3400             0.5              2.0                     𝑦 (")
 500              0.25             1.0                     𝑦 (1)
 1900             0.8              3.2                     𝑦 ($)
            (#)     ($)
The values 𝑦 , … , 𝑦 are either 0 or 1.

A company uses the logistic regression model 𝜎(𝑤# 𝑥# + 𝑤" 𝑥" + 𝑤1 𝑥1 ), without oeset. You
are asked to submit alternative models including a recommendation to retrain the model or
not. Consider the scenario where retraining is expensive, hence a retraining
recommendation should only be made when necessary. Also, the alternative model should
be at least as expressive as the current model.

Note that a more expressive model is a model with higher model complexity.

Select the logistic regression models and the corresponding recommendation. Select all
that apply.

   a) Model: 𝜎((𝑤# + 4𝑤" )𝑥" + 𝑤1 𝑥1 ). Retrain: no.
   b) Model: 𝜎((𝑤# + 4𝑤" )𝑥" + 𝑤1 𝑥1 ). Retrain: yes.
   c) Model: 𝜎(𝑤# 𝑥# + (𝑤" + 4𝑤1 )𝑥" ). Retrain: no.
   d) Model: 𝜎(𝑤# 𝑥# + (𝑤" + 4𝑤1 )𝑥" ). Retrain: yes.
                    |'! 78++|
   e) Model: 𝜎(𝑤#     "9++
                                + 𝑤" 𝑥" ). Retrain: no.
                    |'! 78++|
   f) Model: 𝜎(𝑤#     "9++
                                + 𝑤" 𝑥" ). Retrain: yes.
   g) None of the above.

<!-- answer:start -->
exact
CF
C F
C,F
C, F
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** cf.

6E: cf.

Feature 𝑥0 is 4 times feature 𝑥! . Hence, we can remove it at no loss of model complexity.

    a) Model: 𝜎((𝑤$ + 4𝑤! )𝑥! + 𝑤0 𝑥0 ). Retrain: no.

       False, the model complexity is reduced by not considering feature 1.
    b) Model: 𝜎((𝑤$ + 4𝑤! )𝑥! + 𝑤0 𝑥0 ). Retrain: yes.

          False, the model complexity is reduced by not considering feature 1.

    c) Model: 𝜎(𝑤$ 𝑥$ + (𝑤! + 4𝑤0 )𝑥! ). Retrain: no.

          True, no retraining needed if we combine the weights as given.

    d) Model: 𝜎(𝑤$ 𝑥$ + (𝑤! + 4𝑤0 )𝑥! ). Retrain: yes.

          False, no retraining needed. From the problem statement, retraining should be avoided.

                       |5! 6(&&|
    e) Model: 𝜎(𝑤$       !7&&
                                   + 𝑤! 𝑥! ). Retrain: no.

          False, while the model complexity is retained, we must retrain the model. Also, the feature
          scaling means that we must retrain the model.

                       |5! 6(&&|
   f)    Model: 𝜎(𝑤$     !7&&
                                   + 𝑤! 𝑥! ). Retrain: yes.
         True, while the model complexity is retained, we must retrain the model. Also, the feature
         scaling means that we must retrain the model.
<!-- explanation:end -->

## q6F [medium]

**Question 6F [2 marks]**

You are given a data set that consists of images of size 1024 × 768, where

each pixel of every image has a grayscale value 𝑔 ∈ [0,1]. You have 100 images available.

You create a feature vector 𝑥, where each feature corresponds to the greyscale value of a
pixel. Additionally, each image has an associated binary target 𝑦. You plan to use a logistic
regression model ℎ(𝑥), and you aim to minimize the BCE loss to predict the target value for
new images. Given these choices, select true statements about the problem and the
associated training of the model?

    a) The normal equation will be the preferred method over gradient descent for training
       the model.
    b) The model has signiﬁcantly more parameters than data points.
    c) Min-max scaling is necessary to improve the training of the model.
    d) Gradient descent will be about 0.01 times as fast as stochastic gradient descent per
       iteration.
    e) None of the statements are true.

<!-- answer:start -->
exact
BD
B D
B,D
B, D
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** bd

6F. bd

   a) False, logistic regression cannot be solved via the normal equation.
   b) True, the model has 786432 +1 parameters and only 100 data points are given.
   c) False, while min-max scaling may achieve some improvements, it is not necessary to
      improve the training of the model. All the features already are on a scale of [0,1].
   d) True, gradient descent uses in every iteration all the 100 data points. Hence it will be about
      100 times slower than stochastic gradient descent per iteration, where SGD uses only a
      single data point.
<!-- explanation:end -->
