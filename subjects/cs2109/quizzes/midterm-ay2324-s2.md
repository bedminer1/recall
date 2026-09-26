# AY2023/24 Semester 2 Midterm

Keep `CS2109S+AY2023-24+Sem+2+-+Midterm+-+Solution.pdf` open for the shared context, tables, and figures. Answer choices are reproduced below where present.

## q1 [hard]

**Question 1 [4.0 marks]**

[Question Group: Search Formulation]
Spaceship Transport
Captain Picard is leading a mission to transport n groups of people from different home planets [g1,...,gn]
using m spaceships [s1,...,sm]. Here, gi denotes the number of people in the i-th group, and sj denotes the
capacity of the j-th spaceship. Each group of people cannot be broken apart and must be in the same
ship. Picard needs to assign each group to a ship and ensure that all groups are assigned to spaceships.
At one time, Picard can only allow one group to enter or exit one spaceship. Since the spaceships will
depart tomorrow, Picard must come up with an efficient plan, i.e., a plan with the smallest number of
actions.
–––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––
––––––––––––––––––––––––––––––––––––––
Suppose that you use the following state representation:
•       [z1,...,zm +1] where zi is the set of groups that are assigned to the spaceship i for 1 ≤ i ≤ m. zm +1 is
the set of unassigned groups.

Consider the following invariants, initial states, goal tests, and actions:
Invariants:
•         I1: The number of people in each spaceship must not exceed its capacity.
I1: ∀i ∈ [1,...,m]∑j ∈ z_i gj ≤ si , note: z_i =zi

•         I2: The union of all groups in all spaceships and unassigned groups must be the same as {1, ..., n}.
I2: ∪i ∈ [1,...,m+1]zi ={1, ..., n}, where ∪ is union operator, e.g., {1,2}∪{2,3}={1,2,3}

•        I3: Total number of people in all spaceships must be the same as the total capacity of all spaceships
I3: ∑x ∈ w gx =∑i ∈ [1,...,m]si, where w =⊕i ∈ [1,...,m]zi and ⊕ is concatenation operator, e.g.,
[1,2]⊕[2,3]=[1,2,2,3]

Initial States:
•         S1: All spaceships are empty and all groups are unassigned
S1: ∀i ∈ [1,...,m]zi =[], zm +1 =[1,...,n]

•       S2: Randomly assign each group to a spaceship or let it unassigned
S2: randomly assign [1,...,n] to z1, …, zm +1 without replacement

Goal test:
•       G1: The union of all groups in all spaceships must be the same as {1, ..., n}.
G1: ∪i ∈ [1,...,m]zi ={1, ..., n}

•       G2: No unassigned groups
G2: zm +1 =[]

Actions:
•       A1: Move each x ∈ zm +1 to each zi where i ≤ m

•       A2: Move each x ∈ zi to zm +1 for each i ≤ m

•         A3: Swap each x ∈ zi with each y ∈ zj for any (random) i, j

Which of the above invariant, initial state, goal test, and action tuple(s) is/are reasonable?

        A. I1&I2 −S1 −G1 −A1&A2&A3
    B. I1&I2 −S1 −G1&G2 −A1
        C. I1&I2 −S2 −G1&G2 −A3
        D. I1&I3 −S1 −G1&G2 −A1&A2&A3
        E. I1&I3 −S2 −G1 −A1&A2
        F. I1&I3 −S2 −G1&G2 −A1&A2&A3
        G. I1&I2&I3 −S1 −G2 −A2&A3
        H. I1&I2&I3 −S1 −G1&G2 −A1&A2
         I. I1&I2&I3 −S2 −G1&G2 −A1&A2&A3
        J. None of the above

<!-- answer:start -->
exact
B
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** B

∙     Any answers that contain A3 is incorrect because swapping action is not possible based on the
          description i.e., “Picard can only allow one group to enter or exit one spaceship”.
    ∙     Any answers that contain I3 is incorrect because it is not necessary that the number of people in
          all spaceships equals the combined capacity of all spaceships.
<!-- explanation:end -->

## q2 [easy]

**Question 2 [1.0 marks]**

[Question Group: Uninformed Search]
Spaceship Transport
Captain Picard is leading a mission to transport n groups of people from different home planets [g1,...,gn]
using m spaceships [s1,...,sm]. Here, gi denotes the number of people in the i-th group, and sj denotes the
capacity of the j-th spaceship. Each group of people cannot be broken apart and must be in the same
ship. Picard needs to assign each group to a ship and ensure that all groups are assigned to spaceships.
At one time, Picard can only allow one group to enter or exit one spaceship. Since the spaceships will
depart tomorrow, Picard must come up with an efficient plan, i.e., a plan with the smallest number of
actions.
Suppose that we formulate the above problem as a search problem as follows.
State representation: [x1,...,xn] where xi ∈ [0,...,m] represents the assignment of group i to a spaceship
among the m spaceships, 0 means not assigned.
Invariant:
•         xi ∈ [0,...,m]

•         Total number of people in each spaceship is less than the capacity

Initial state: [0,...,0], all zero (unassigned)
Goal test: [x1,...,xn] where xi ∈ [1,...,m] (all assigned)
Actions: Set each xi to 0, 1, ..., or m. i.e., n*(m+1) actions
–––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––
––––––––––––––––––––––––––––––––––––––
Is it true that the search formulation results in a state space where a state can be visited multiple
times? Note: we do not care about the search algorithms in this question since we are asking about state
space, not search tree/graph.

   A. True
    B. False

<!-- answer:start -->
exact
A
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** A

Based on the action definition, xi can be set to any value in [0, …, m]. Thus, we can revisit any state
multiple times.
<!-- explanation:end -->

## q3 [easy]

**Question 3 [1.0 marks]**

[Question Group: Uninformed Search]

Is it true that the search formulation results in a state space with many goal states?

   A. True
    B. False

<!-- answer:start -->
exact
AACDEF
A A C D E F
A,A,C,D,E,F
A, A, C, D, E, F
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** A,A,C,D,E,F

It is possible that the solution is not unique. For instance, suppose that we have 2 spaceships with the
same capacity of 10 and two group of people (10 people each). Then, the goal states are [1,2] and [2,1].
<!-- explanation:end -->

## q5 [medium]

**Question 5 [2.0 marks]**

[Question Group: Uninformed Search]

Which of the following tree search algorithm(s) can we employ such that the search always terminates
(including if there is no solution)?

       A. Breadth-first search (BFS)
       B. Depth-first search (DFS)
       C. Uniform-cost search (UCS) with constant cost
  D. Depth-limited search with BFS
   E. Depth-limited search with DFS
       F. Iterative deepening search (IDS)
       G. None of the above

Notes:
   ∙     DFS may get stuck in an infinite loop as explained previously.
   ∙     BFS, UCS, and IDS may not terminate if there is no solution, i.e., they will keep on searching and
         revisiting previously visited states.
   ∙     Depth-limited search will always terminate since the depth is limited and the branching factor is
         finite.
Update:
   ∙     IDS definition in the lecture is ambiguous. It can be interpreted as running DLS from 0 depth up to
         an infinite depth or from 0 depth until a finite depth. To clarify, the usual implementation of IDS
         uses no max depth limit, so it runs DLS from 0 depth to an infinite depth.
   ∙     Given the above, we'll allow both interpretations:
         1. Under the assumption that IDS has a finite depth: IDS and DLSes (D, E and F)
         2. Otherwise: DLSes (D and E)

<!-- answer:start -->
exact
DE
D E
D,E
D, E
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** D,E
<!-- explanation:end -->

## q6 [medium]

**Question 6 [2.0 marks]**

[Question Group: Uninformed Search]

Which of the following tree search algorithm(s) can we employ such that the search always finds an
answer (valid sequence of moves) if a solution exists?

  A. Breadth-first search (BFS)
       B. Depth-first search (DFS)
  C. Uniform-cost search (UCS) with constant cost
       D. Depth-limited search with BFS
       E. Depth-limited search with DFS
   F. Iterative deepening search (IDS)
       G. None of the above

<!-- answer:start -->
exact
ACF
A C F
A,C,F
A, C, F
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** A,C,F

∙     DFS may get stuck in an infinite loop as explained previously.
   ∙     Depth-limited search may not find an answer if the depth is not set correctly.
Update:
         See Q5 explanation about IDS and we'll allow the following interpretations:
         1. Assume correct max depth: DLSs, BFS, UCS, IDS (A, C, D, E and F)
         2. Assume: max depth can be wrong + IDS has finite depth limit: BFS, UCS (A and C)

         3. Otherwise: BFS, UCS, IDS (A, C and F)
   ∙
<!-- explanation:end -->

## q7 [medium]

**Question 7 [2.0 marks]**

[Question Group: Uninformed Search]

Which of the following tree search algorithm(s) is/are the best for the problem? Best means the
algorithm(s) should be complete, optimal, efficient, and aware if there is no solution.

       A. Breadth-first search (BFS)
       B. Depth-first search (DFS)
       C. Uniform-cost search (UCS) with constant cost
       D. Depth-limited search with BFS
       E. Depth-limited search with DFS
       F. Iterative deepening search (IDS)
  G. None of the above

<!-- answer:start -->
exact
G
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** G

None of the algorithms satisfy the definition of “Best” given in the question.
Update:
Using the state representation given in the question, we can actually bound the depth of the search.
Since we didn't explicitly state the assumption regarding whether DLS uses the correct depth or any
depth, we'll allow both interpretations.
   1. Assume: correct max depth: DLS with DFS (E)
   2. Assume: max depth can be wrong: None of the above (G)
<!-- explanation:end -->

## q8 [medium]

**Question 8 [2.0 marks]**

[Question Group: Uninformed Search]

Which of the following graph search algorithm(s) is/are the best for the problem? Best means the
algorithm(s) should be complete, optimal, efficient, and aware if there is no solution.

  A. Breadth-first search (BFS)
    B. Depth-first search (DFS)
  C. Uniform-cost search (UCS) with constant cost
    D. Depth-limited search with BFS
    E. Depth-limited search with DFS
   F. Iterative deepening search (IDS)
    G. None of the above

<!-- answer:start -->
exact
ACF
A C F
A,C,F
A, C, F
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** A,C,F

The three algorithms above satisfy the definition. Efficient here could refer to time efficiency and space
efficiency. Since there is a trade-off between the two, BFS and UCS with constant cost and IDS are
correct, the former two have better time-complexity (no overhead) while the latter has better space
complexity.
Update:
See explanation about DLS in Q7
In the question, we didn't specify explicitly regarding the consideration of space and time efficiency.
Thus, we will allow the following:
   1. Assume: correct max depth: DLS with DFS (E)
   2. Assume: max depth can be wrong + IDS has depth limit: BFS, UCS (A and C)
   3. Assume: max depth can be wrong + IDS no depth limit + efficiency considering big-O notation:
         IDS (F)
   4. Assume: max depth can be wrong + IDS no depth limit + efficiency counting the overhead: IDS,
         BFS, or UCS (A, C and F)
<!-- explanation:end -->

## q9 [hard]

**Question 9 [3.0 marks]**

[Question Group: Local Search]
Spaceship Transport: Variant #1
Captain Picard is leading a mission to transport n groups of people from different home planets [g1,...,gn]
using spaceships. Here, gi denotes the number of people in the i-th group, and each spaceship can hold
k people. Each group of people cannot be broken apart and must be in the same ship. Picard needs to
assign each group to a ship and ensure that all groups are assigned to spaceships. There is no limit to
the number of spaceships. In this case, Picard wants to ensure the use of the minimum number of
spaceships.
Suppose that we formulate the above problem as a local search problem and use the following state
representation: [a1,...,an] where ai ≥ 0 represents the assignment of group i to the ai-th spaceship, ai =0
means group i is not assigned yet.

–––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––
––––––––––––––––––––––––––––––––––––––
Consider the following initial states and successor functions:
Initial states:
•         S1 - [0,…,0]

•         S2 - ai randomly sampled from [1,…,n]

Successor functions:
•         F1 - Select a random group i, then set ai =0, ai =1, ..., or, ai =n (there will be n +1 neighbors)

•      F2 - Select a random group i where ai =0, then set ai =1, ai =2, ..., or, ai =n (there will be n
neighbors)

•         F3 - Swap ai with aj for any i, j

•      F4 - Select a random group i where ai =0, then set ai =1, ai =2, ..., or, ai =n (there will be n
neighbors) +swap ai with aj for any i, j, where ai >0 and aj >0

The successor function ensures that the number of people in the ai-th spaceship is less than k
Which of the above initial state and successor function pair(s) is/are reasonable?

    A. S1 −F1
        B. S1 −F2
        C. S1 −F3
    D. S1 −F4
    E. S2 −F1
        F. S2 −F2
    G. S2 −F3
    H. S2 −F4
         I. None of the above

<!-- answer:start -->
exact
ADEGH
A D E G H
A,D,E,G,H
A, D, E, G, H
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** A,D,E,G,H

∙     B is sampling a state through a greedy sequential random sampling
    ∙     C is simply swapping 0s
    ∙     F is random sampling one state
Update:
We didn't define what "+" means, but this one doesn't matter. We didn't properly define what reasonable
actually means. Thus, we will allow two different interpretations of reasonability:
    1. Reasonable: not just random sample a state: S1-F1, S1-F4, S2-F1, S2-F3, S2-F4 (A, D, E, G and
          H)
    2. Reasonable: can explore all states: S1-F1 & S2-F1 (A and E)

Item Weight: 3.0
____________________________________________________________________________
<!-- explanation:end -->

## q10 [hard]

**Question 10 [3.0 marks]**

[Question Group: Local Search]

Let
•            s be the current state,

•            set(x) returns a set of unique items in the list x,

•            len(x) returns the number of items in a set/list x,

•            count(x,i) returns the number of items in x that has the value i,

•            indices(x,i) returns the indices of items in x that has the value i,

•            f1(s) =n − count(s,0),

•            f2(s) =len(set(s)),

•       f3(s,i) =∑j ∈ indices(s,i)gj. i.e., the total number of people in spaceship i if s is the state and i the index
of the spaceship

Suppose that we are doing hill-climbing where we want to maximize an evaluation function. Regardless
of the initial state and the successor function, which of the following evaluation function(s) is/are
reasonable (i.e., maximizing it/them lead(s) us closer to the objective)?

          A. f(s) = f1(s)
          B. f(s) = 1/f2(s)
          C. f(s) = minif3(s,i)
          D. f(s) = f1(s) + ∑igi * f2(s)
          E. f(s) = f1(s) + minif3(s,i)
          F. f(s) = ∑igi/f2(s) + minif3(s,i)
    G. f(s) = f1(s) + ∑if3(s,i)/f2(s)
          H. None of the above

<!-- answer:start -->
exact
G
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** G

∙      Considering only f1 is incorrect because it could lead to a trivial solution where each group is
             assigned to one spaceship.
      ∙      Considering only 1/f2 is incorrect because it could lead to a trivial solution where no group is

        assigned, giving a maximum value.
    ∙   minif3(s,i) is always 0 since i is unbounded (i.e., there is no limit in the number of spaceships).
    ∙   f2 is incorrect because it could lead to maximizing the number of spaceships.
<!-- explanation:end -->

## q11 [medium]

**Question 11 [2 marks]**

[Question Group: Local Search]

Using the above state representation, which of the following uninformed search algorithm(s) can we use
to find the optimal solution assuming that it exists?

  A. Breadth-first search (BFS)
  B. Depth-first search (DFS)
  C. Uniform-cost search (UCS) with constant cost
  D. DLS with BFS
  E. DLS with DFS
  F. Iterative deepening search (IDS)
   G. None of the above
Update:
Here, only the state representation is fixed, and we can use any transition function. Let [a1, a2, ..., an] be
any state.
One possible transition function is: Pick any nonempty subset of indices such that ai = 0 for every i in the
subset and the sum of group sizes in the subset does not exceed k, and change ai in the selected subset
to max(a1, a2, ..., an) + 1.

Let's suppose n = 4, [g1, g2, g3, g4] = [1, 2, 3, 4], and k = 6.
Initial state: [a1, a2, a3, a4] = [0, 0, 0, 0]. The set of indices i such that ai = 0 is {1, 2, 3, 4}.

The non-empty subsets of indices of {1, 2, 3, 4} such that the sum of the corresponding g_i for all i in the
subset is no more than k = 6 are {1}, {2}, {3}, {4}, {1, 2}, {1, 3}, {1, 4}, {2, 3}, {2, 4}, {1, 2, 3}. So we can
select any of the subsets and set a_i in the selected subset as 1 (max(a1, a2, a3, a4) + 1 = 1). Therefore,
the corresponding neighbors are (in the same order): [1, 0, 0, 0], [0, 1, 0, 0], [0, 0, 1, 0], [0, 0, 0, 1], [1, 1,
0, 0], [1, 0, 1, 0], [1, 0, 0, 1], [0, 1, 1, 0], [0, 1, 0, 1], [1, 1, 1, 0]. This is considered as one step, and notice
that exactly one spaceship is used.

Now, let's suppose we have selected subset {1, 3} and are currently at the state [1, 0, 1, 0]. The set of
indices i such that ai = 0 is {2, 4}. The non-empty subsets of indices of {2, 4} such that the sum of the

corresponding g_i for all i in the subset is no more than k = 6 are {2}, {4}, {2, 4}. So we can select any of
the subsets and set a_i in the selected subset as 2 (max(a1, a2, a3, a4) + 1 = 2). Therefore, the
corresponding neighbors are (in the same order): [1, 2, 1, 0], [1, 0, 1, 2], [1, 2, 1, 2]. Notice that exactly
two spaceships are used.

Basically, we consider all possible group assignments starting from spaceship 1, 2, ... until all groups are
assigned. In this case, we can utilize BFS, UCS, or IDS (A, C, and F). The original answer G is not
correct.

Reference:
https://coursemology.org/courses/2714/forums/general-discussion/topics/official-midterm-queries

<!-- answer:start -->
exact
ACF
A C F
A,C,F
A, C, F
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** A,C,F
<!-- explanation:end -->

## q12 [medium]

**Question 12 [2.0 marks]**

[Question Group: Heuristics: Light]
Light
The first floor of the DBZ Bank building contains n x n rooms, each equipped with a light. Some of these
lights are currently on. Your task is to turn off all the lights on the first floor. You are denoted as p, and
your current position is represented by coordinates (xp, yp). The position of each room r can be
represented by coordinates (xr, yr), where r ∈ [0,...,n2−1] is the index of the room. If you are in room r
located at coordinates (xr, yr), then xp =xr and yp =yr. Importantly, you must remain within the n x n rooms
at all times. In each step, you are only allowed to take the following actions:
1.Move one room to the left/right/up/down with a cost of 1.
2.Stay in the current room and turn off the light in the room if it is on with a cost of 1.
3.Stay in the current room and turn on the light in the room if it is off with a cost of 1.
Please note that due to the special arrangement on the first floor, turning off the light in room r will
also turn off the light in the directly adjacent room to the left if it is currently on, and it will remain
off if it is currently off. This only applies when the position of the directly adjacent room to the left is within
the n x n grid. Turning on the light in room r will not affect the lights in other rooms.
For instance, consider Figure (1), where your current position is (1, 3), and the positions of rooms with
lights on are {(1, 1), (1, 2), (1, 3)}. You can now choose to move one room to the left/right/up/down, as
shown in Figure (2). Alternatively, you can stay in the current room and turn off the light, as it is currently
on. In this case, both lights in rooms (1, 2) and (1, 3) will be turned off due to the special arrangement on
the first floor. To turn off all the lights, you can then move to room (1, 1) and turn off the light, as illustrated
in Figures (4) and (5) respectively.

After discussing this problem with your cousin, Ben Bitdiddle, he proposed some heuristics to be used
with A* search. Your task is to evaluate each heuristic.
Hint: The Manhattan Distance is defined as MD(a,b) =|xa−xb| +|ya−yb|, where xa and ya represent the row
and column index of a, and the same applies to b.
–––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––
–––––––––––––––––––––––––––––––––––––––––––
hA: The number of rooms with lights on.
Is hA admissible for this Light problem?

         A. Yes, it is admissible.
         B. No, it is not admissible.

<!-- answer:start -->
exact
B
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** B

Consider a situation where your current position is (0, 2), and the rooms with lights on are (0, 1) and (0,
2). In this case, hA = 2, which exceeds the real cost, h* = 1 (turn off the light in room (0, 2)). Thus, hA is
not admissible.
<!-- explanation:end -->

## q13 [medium]

**Question 13 [2.0 marks]**

[Question Group: Heuristics: Light]

hA: The number of rooms with lights on.
Is hA consistent for this Light problem?

         A. Yes, it is consistent.
         B. No, it is not consistent.

<!-- answer:start -->
exact
B
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** B

Consider a situation where your current position is (0, 2), and the rooms with lights on are (0, 1) and (0,
2). In this case, the next state (n’) can be the goal state if we choose to turn off the light in room (0, 2).
hA(n) = 2 > c(n, a, n’) + hA(n’) = 1 + 0. Thus, hA is not consistent.
<!-- explanation:end -->

## q14 [medium]

**Question 14 [2.0 marks]**

[Question Group: Heuristics: Light]

hB: Divide the sum of all Manhattan distances between rooms with light on and your current position by 2.
hB =(∑r ∈ RMD(r,p))/2.
Here, R contains the indices of all the rooms with lights on.
Is hB admissible for this Light problem?

         A. Yes, it is admissible.
         B. No, it is not admissible.

<!-- answer:start -->
exact
B
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** B

Consider a situation where your current position is (3, 3), and the rooms with lights on are {(1, 0), (1, 1),
(2, 1), (2, 2)}. In this case, hB = (5+4+3+2)/2=7, which exceeds the real cost, h* = 6 (1. Move from (3, 3)
to (3, 2); 2. Move from (3, 2) to (2, 2); 3. Turn off the light in room (2, 2); 4. Move from (2, 2) to (2, 1); 5.
Move from (2, 1) to (1, 1); 6. Turn off the light in room (1, 1)). Thus, hB is not admissible.
<!-- explanation:end -->

## q15 [medium]

**Question 15 [2.0 marks]**

[Question Group: Heuristics: Light]

hB: Divide the sum of all Manhattan distances between rooms with light on and your current position by 2.
hB =(∑r ∈ RMD(r,p))/2.
Here, R contains the indices of all the rooms with lights on.
Is hB consistent for this Light problem?

         A. Yes, it is consistent.
         B. No, it is not consistent.

<!-- answer:start -->
exact
B
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** B

Consider a situation where your current position is (3, 3), and the rooms with lights on are {(1, 0), (1, 1),
(2, 1), (2, 2)}. In this case, hB(n) = (5+4+3+2)/2=7. If you choose to move from (3, 3) to (3, 2) with a cost
of 1, then hB(n’) = (4+3+2+1)/2=5. hB(n) = 7 > c(n, a, n’) + hB(n’) = 1 + 5. Thus, hB is not consistent.
<!-- explanation:end -->

## q16 [medium]

**Question 16 [2.0 marks]**

[Question Group: Heuristics: Light]

hC: The maximum Manhattan distance between rooms with lights on and your current position.
hC =maxr ∈ RMD(r,p).
Here, R contains the indices of all the rooms with lights on.
Is hC admissible for this Light problem?

         A. Yes, it is admissible.
         B. No, it is not admissible.

<!-- answer:start -->
exact
AA
A A
A,A
A, A
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** A,A

To turn off all the lights, we must turn off the light in the farthest room, rf. There are two cases for turning
off the light in the farthest room:

In case 1, if the light in the directly adjacent room to the right of rf is off. You can move to room rf and
choose to turn off the light. The actual cost to turn off the light in the farthest room is hC + 1.

In case 2, if the light in the directly adjacent room to the right of rf is also on, you can move to that
adjacent room and choose to turn off its light. The actual cost to turn off the light in the farthest room then
becomes (hC – 1) + 1 = hC.

Thus, the actual cost to turn off the light in the farthest room is at least hC, and hC is admissible.
<!-- explanation:end -->

## q18 [medium]

**Question 18 [2.0 marks]**

[Question Group: Heuristics: Light Variant #1]
Light Variant #1
The second floor of the DBZ Bank building contains n x n rooms, each equipped with a light. Some of
these lights are currently on. Your task is to turn off all the lights on the second floor. You are denoted
as p, and your current position is represented by coordinates (xp, yp). The position of each room r can be
represented by coordinates (xr, yr), where r ∈ [0,...,n2−1] is the index of the room. If you are in room r
located at coordinates (xr, yr), then xp =xr and yp =yr. Importantly, you must remain within the n x n rooms
at all times. In each step, you are only allowed to take the following actions:
1.Move one room to the left/right/up/down with a cost of 1.
2.Stay in the current room and turn off the light in the room if it is on with a cost of 1.
3.Stay in the current room and turn on the light in the room if it is off with a cost of 1.
Please note that due to a special arrangement on the second floor, changing the status of the light in
room r will toggle the status of lights in neighboring rooms. Specifically, if you are now in room r,
and you switch the light in room r from off to on or from on to off, the lights in rooms at positions (xr − 1,
yr), (xr, yr − 1), (xr +1, yr), and (xr, yr +1) will be toggled if their positions are within the n x n grid.
As shown in Figure (1), you are currently located at (1, 3), and there are five rooms with lights on: {(0, 3),
(0, 4), (1, 2), (2, 3), (2, 4)}. You are allowed to move one room to the left/right/up/down, as illustrated in
Figure (2). Alternatively, you can choose to stay in the current room and turn on the light, as it is currently
off. If you choose to turn on the light in room (1, 3), then as depicted in Figure (3), the lights in room (1, 4)
will also be turned on, while the lights in rooms (1, 2), (0, 3), and (2, 3) will be turned off. To turn off all the

lights, you can opt to move to (1, 4) (Figure (4)) and turn off the lights in room (1, 4). Consequently, the
lights in rooms (1, 3), (0, 4), and (2, 4) will also be turned off, as shown in Figure (5).

After discussing this problem with your cousin, Ben Bitdiddle, he proposed some heuristics to be used
with A* search. Your task is to evaluate each heuristic.
Hint: The Manhattan Distance is defined as MD(a,b) =|xa−xb| +|ya−yb|, where xa and ya represent the row
and column index of a, and the same applies to b.
–––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––
–––––––––––––––––––––––––––––––––––––––––––
hD: The minimum Manhattan distance between rooms with lights on and your current position.
hD =minr ∈ RMD(r,p).
Here, R contains the indices of all the rooms with lights on.
Is hD admissible for this Light Variant #1 problem?

         A. Yes, it is admissible.
         B. No, it is not admissible.

<!-- answer:start -->
exact
A
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** A

To turn off all the lights, we need to turn off the light in the farthest room, rf. To do so, we need at least
move to one of its neighboring rooms and change the status of the light in that room. Then the cost is

(maxr ∈ RMD(r,p) - 1) + 1 = maxr ∈ RMD(r,p) >= hD. Thus, hD is admissible.
<!-- explanation:end -->

## q19 [medium]

**Question 19 [2.0 marks]**

[Question Group: Heuristics: Light Variant #1]

hD: The minimum Manhattan distance between rooms with lights on and your current position.
hD =minr ∈ RMD(r,p).
Here, R contains the indices of all the rooms with lights on.
Is hD consistent for this Light Variant #1 problem?

         A. Yes, it is consistent.
         B. No, it is not consistent.

<!-- answer:start -->
exact
B
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** B

Consider a situation where your current position is (3, 3), and the rooms with lights on is (0, 0). In this
case, hD(n) = 6. If you choose to turn on the light in room (3, 3) with a cost of 1, then hD(n’) = 0. hD(n) = 6
> c(n, a, n’) + hD(n’) = 1 + 0. Thus, hD is not consistent.
<!-- explanation:end -->

## q20 [medium]

**Question 20 [2.0 marks]**

[Question Group: Heuristics: Light Variant #1]

hE: Divide the number of rooms with light on by 5.
Is hE admissible for this Light Variant #1 problem?

         A. Yes, it is admissible.
         B. No, it is not admissible.

<!-- answer:start -->
exact
A
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** A

Referring to Q21, we can prove that hE is consistent. Since consistency implies admissibility, hE is
admissible.
<!-- explanation:end -->

## q21 [medium]

**Question 21 [2.0 marks]**

[Question Group: Heuristics: Light Variant #1]

hE: Divide the number of rooms with light on by 5.
Is hE consistent for this Light Variant #1 problem?

         A. Yes, it is consistent.
         B. No, it is not consistent.

<!-- answer:start -->
exact
A
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** A

Considering the available actions in the Light Variant #1 problem, the maximum number of lights can be
turned off in one step is 5. So hE can be reduced by at most 1 in each step, i.e., hE (n) – hE (n’) ≤ 1 = c(n,
a, n’). Therefore, hE (n) ≤ hE (n’) + c(n, a, n’), and hE is consistent.
<!-- explanation:end -->

## q22 [medium]

**Question 22 [2.0 marks]**

[Question Group: Heuristics: Light Variant #1]

hF: The average of the maximum and minimum Manhattan distances between rooms with light on and
your current position.
hF =(maxr ∈ RMD(r,p)+minr ∈ RMD(r,p))/2.
Here, R contains the indices of all the rooms with lights on.
Is hF admissible for this Light Variant #1 problem?

         A. Yes, it is admissible.
         B. No, it is not admissible.

<!-- answer:start -->
exact
A
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** A

To turn off all the lights, we need to turn off the light in the farthest room, rf. To do so, we need at least
move to one of its neighboring rooms and change the status of the light in that room. Then the cost is

(maxr ∈ RMD(r,p) - 1) + 1 = maxr ∈ RMD(r,p) >= hF. Thus, hF is admissible.
<!-- explanation:end -->

## q23 [medium]

**Question 23 [2.0 marks]**

[Question Group: Heuristics: Light Variant #1]

hF: The average of the maximum and minimum Manhattan distances between rooms with light on and
your current position.
hF =(maxr ∈ RMD(r,p)+minr ∈ RMD(r,p))/2.
Here, R contains the indices of all the rooms with lights on.
Is hF consistent for this Light Variant #1 problem?

         A. Yes, it is consistent.
         B. No, it is not consistent.

<!-- answer:start -->
exact
B
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** B

Consider a situation where your current position is (3, 3), and the rooms with lights on is (0, 0) and (0, 1).
In this case, hF(n) = (6+5)/2=5.5. If you choose to turn on the light in room (3, 3) with a cost of 1, then
hF(n’) = (6+0)/2=3. hF(n) = 5.5 > c(n, a, n’) + hD(n’) = 1 + 3. Thus, hF is not consistent.
<!-- explanation:end -->

## q24 [medium]

**Question 24 [2.0 marks]**

[Question Group: Heuristics: Light Variant #1]

If you are required to use A* graph search for this Light Variant #1 problem, which heuristic(s) should you
pick among hD, hE and hF?
1.hD: The minimum Manhattan distance between rooms with lights on and your current position.
  hD =minr ∈ RMD(r,p). Here, R contains the indices of all the rooms with lights on.
2.hE: Divide the number of rooms with light on by 5.
3.hF: The average of the maximum and minimum Manhattan distances between rooms with light on and
your current position.
  hF =(maxr ∈ RMD(r,p)+minr ∈ RMD(r,p))/2. Here, R contains the indices of all the rooms with lights on.

         A. hD
         B. hE
         C. hF
         D. None of the above

<!-- answer:start -->
exact
B
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** B

If h(n) is consistent, A* using graph search is optimal. hD and hF are not consistent, so we need to select
hE.
<!-- explanation:end -->

## q25 [medium]

**Question 25 [2.0 marks]**

[Question Group: Decision Tree]
As DBZ Bank launches its recruitment drive, you, a machine learning specialist within its ranks, are
tasked with the pivotal role of shortlisting candidates for interviews. Drawing from your expertise, you've
chosen to kickstart the project by constructing a decision tree based on the bank's historical data
presented in table below.

–––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––
–––––––––––––––––––––––––––––––––––––––––––
What is the entropy of the Outcomes (offer/reject) in the table, rounded to 2 decimal places? 0.88

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
Update:
Answers with rounding errors are accepted.
<!-- explanation:end -->

## q26 [medium]

**Question 26 [2.0 marks]**

[Question Group: Decision Tree]

You decide to start by creating a one-level decision tree with only one split, using information gain. What
is root node for your one-level decision tree?

         A. Education
         B. Experience
         C. Reference
         D. Salary Expectation

<!-- answer:start -->
exact
A
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** A
<!-- explanation:end -->

## q27 [easy]

**Question 27 [1.0 marks]**

[Question Group: Decision Tree]

Based on your one-level decision tree created according to information gain. What is the Outcome for
the following applicant?
Applicant Information:
Education: Bachelor
Experience: <1 year
Reference: good
Salary Expectation: low

         A. offer
         B. reject
         C. offer/reject

<!-- answer:start -->
exact
B
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** B

Item Weight: 1.0
____________________________________________________________________________
<!-- explanation:end -->

## q28 [easy]

**Question 28 [1.0 marks]**

[Question Group: Decision Tree]

Based on your one-level decision tree created according to information gain. What is the Outcome for
the following applicant?
Applicant Information:
Education: Master
Experience: >3 years
Reference: bad
Salary Expectation: high

         A. offer
         B. reject
         C. offer/reject

<!-- answer:start -->
exact
A
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** A
<!-- explanation:end -->

## q29 [easy]

**Question 29 [1.0 marks]**

[Question Group: Full Decision Tree]
As DBZ Bank launches its recruitment drive, you, a machine learning specialist within its ranks, are
tasked with the pivotal role of shortlisting candidates for interviews. Drawing from your expertise, you've
chosen to kickstart the project by constructing a decision tree based on the bank's historical data
presented in table below.

Suppose that you pick "Reference" as the root of your decision tree. Use the data in the table and
information gain to create the full decision tree. In case of a tie, the priority order for constructing the tree
is Education >Experience >Reference >Salary Expectation.
–––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––
––––––––––––––––––––––––––––––––––––––––––––––––––

What is the Outcome for the following applicant according to your full decision tree?
Applicant Information:
Education: Bachelor
Experience: <1 year
Reference: good
Salary Expectation: low

         A. offer
         B. reject
         C. offer/reject

<!-- answer:start -->
exact
B
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** B

Update:
Q29 is a duplicate of Q31 and is voided.
<!-- explanation:end -->

## q30 [easy]

**Question 30 [1.0 marks]**

[Question Group: Full Decision Tree]

What is the Outcome for the following applicant according to your full decision tree?
Applicant Information:
Education: Master
Experience: 1-3 years
Reference: good
Salary Expectation: high

       A. offer

         B. reject
         C. offer/reject

<!-- answer:start -->
exact
A
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** A
<!-- explanation:end -->

## q31 [easy]

**Question 31 [1.0 marks]**

[Question Group: Full Decision Tree]

What is the Outcome for the following applicant according to your full decision tree?
Applicant Information:
Education: Bachelor
Experience: <1 year
Reference: good
Salary Expectation: low

         A. offer
         B. reject

         C. offer/reject

<!-- answer:start -->
exact
B
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** B
<!-- explanation:end -->

## q32 [medium]

**Question 32 [2.0 marks]**

[Question Group: Full Decision Tree]

If you aim to prune the full decision tree, ensure that each leaf node contains at least 3 training data
points.
How many leaf nodes in your pruned decision tree? 2 leaf nodes

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
Update:
Simplifying decision tree following the lecture slides: okay. So 1 leaf node is also accepted.
<!-- explanation:end -->

## q33 [easy]

**Question 33 [1.0 marks]**

[Question Group: Full Decision Tree]

If you aim to prune the full decision tree, ensure that each leaf node contains at least 3 training data
points. According to your pruned decision tree, what is the Outcome for the following applicant?
Applicant Information:
Education: Master
Experience: <1 year
Reference: bad
Salary Expectation: low

         A. offer
         B. reject
         C. offer/reject

<!-- answer:start -->
exact
A
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** A
<!-- explanation:end -->

## q34 [easy]

**Question 34 [0.5 marks]**

[Question Group: Decision Tree Performance Measurement]
One of your colleagues also design his own decision tree to help with recruitement, as shown below.

–––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––
––––––––––––––––––––––––––––––––––––––––––––––––––
Using the data from the table, please figure out the number of True Positives (TP) of his decision tree.
Use "offer" as a positive label and "reject" as a negative label.
The number of True Positives (TP) is       6   .

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
Check the supplied solution for Question 34 in `CS2109S+AY2023-24+Sem+2+-+Midterm+-+Solution.pdf`.
<!-- explanation:end -->

## q35 [easy]

**Question 35 [0.5 marks]**

[Question Group: Decision Tree Performance Measurement]

Using the data from the table, please figure out the number of False Positives (FP) of his decision tree.
Use "offer" as a positive label and "reject" as a negative label.
The number of False Positives (FP) is       3          .

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
Check the supplied solution for Question 35 in `CS2109S+AY2023-24+Sem+2+-+Midterm+-+Solution.pdf`.
<!-- explanation:end -->

## q36 [easy]

**Question 36 [0.5 marks]**

[Question Group: Decision Tree Performance Measurement]

Using the data from the table, please figure out the number of True Negatives(TN) of his decision tree.
Use "offer" as a positive label and "reject" as a negative label.
The number of True Negatives(TN) is        0       .

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
Check the supplied solution for Question 36 in `CS2109S+AY2023-24+Sem+2+-+Midterm+-+Solution.pdf`.
<!-- explanation:end -->

## q37 [easy]

**Question 37 [0.5 marks]**

[Question Group: Decision Tree Performance Measurement]

Using the data from the table, please figure out the number of False Negatives (FN) of his decision tree.
Use "offer" as a positive label and "reject" as a negative label.
The number of False Negatives (FN) is          1           .

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
Check the supplied solution for Question 37 in `CS2109S+AY2023-24+Sem+2+-+Midterm+-+Solution.pdf`.
<!-- explanation:end -->

## q38 [easy]

**Question 38 [1.0 marks]**

[Question Group: Decision Tree Performance Measurement]

Using the data from the table, please figure out the Precision of his decision tree, rounded to 2 decimal
places. Use "offer" as a positive label and "reject" as a negative label.
Precision is     0.67   .

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
Check the supplied solution for Question 38 in `CS2109S+AY2023-24+Sem+2+-+Midterm+-+Solution.pdf`.
<!-- explanation:end -->

## q39 [easy]

**Question 39 [1.0 marks]**

[Question Group: Decision Tree Performance Measurement]

Using the data from the table, please figure out the Recall of his decision tree, rounded to 2 decimal
places. Use "offer" as a positive label and "reject" as a negative label.
Recall is     0.86 .

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
Check the supplied solution for Question 39 in `CS2109S+AY2023-24+Sem+2+-+Midterm+-+Solution.pdf`.
<!-- explanation:end -->

## q40 [easy]

**Question 40 [1.0 marks]**

[Question Group: Decision Tree Performance Measurement]
Using the data from the table, please figure out the F1 Score of his decision tree, rounded to 2 decimal
places. Use "offer" as a positive label and "reject" as a negative label.
F1 Score is     0.75    .

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
Check the supplied solution for Question 40 in `CS2109S+AY2023-24+Sem+2+-+Midterm+-+Solution.pdf`.
<!-- explanation:end -->

## q41 [easy]

**Question 41 [0.5 marks]**

[Question Group: Game Tree &Minimax]
Consider a two-player game featuring two piles of sticks: one with 2 sticks and the other with 3 sticks.
Players alternate turns, selecting to remove either 1 or 3 sticks from one or both piles. If sticks are

removed from both piles, the number of sticks taken from each must be equal. The game concludes
when a player takes the last stick or sticks, thus winning the game.
For example, at the beginning, the first player can opt to remove 1 stick from the first or second pile, 1
stick from each pile, or 3 sticks from the second pile. Notably, removing 3 sticks from each pile or the first
pile isn't feasible since there are only 2 sticks in the first pile. If the first player removes 1 stick from each
pile, the first pile now holds 1 stick, while the second pile holds 2 sticks. The second player then has the
choice to remove either 1 stick from the first or second pile, or 1 stick from each pile. The game proceeds
in this fashion until one player has no move left. At this juncture, the opposing player wins the game.
Note: In this game, if the max player wins, the utility is +1. If the max player loses, the utility is -1. A draw
results in a utility of 0.
Suppose we represent the game state with a monotonically increasing sequence of integers. For
instance, if there are two piles of sticks with the first pile containing 2 sticks and the second pile
containing 1 stick, it will be represented as (1,2). Similarly, if there are two piles of sticks with the first pile
containing 1 stick and the second pile containing 2 sticks, it will also be represented as (1,2). Now,
suppose that we employ the Minimax algorithm to solve the game and constructing the complete game
tree.
–––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––
––––––––––––––––––––––––––––––––––––––
Which of the following state(s) is/are the children of state (0, 0)? Select all that is/are true.

     A. (0, 0)
     B. (0, 1)
    C. (0, 2)
    D. (0, 3)
     E. (1, 1)
     F. (1, 2)
    G. (1, 3)
    H. (2, 2)
      I. (2, 3)
   J. None of the above

<!-- answer:start -->
exact
J
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** J

Item Weight: 0.5
____________________________________________________________________________
<!-- explanation:end -->

## q42 [easy]

**Question 42 [0.5 marks]**

[Question Group: Game Tree &Minimax]

Which of the following state(s) is/are the children of state (0, 1)? Select all that is/are true.

   A. (0, 0)
     B. (0, 1)
    C. (0, 2)
    D. (0, 3)
     E. (1, 1)
     F. (1, 2)
    G. (1, 3)
    H. (2, 2)
      I. (2, 3)
     J. None of the above

<!-- answer:start -->
exact
A
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** A
<!-- explanation:end -->

## q43 [easy]

**Question 43 [0.5 marks]**

[Question Group: Game Tree &Minimax]

Which of the following state(s) is/are the children of state (0, 2)? Select all that is/are true.

     A. (0, 0)
   B. (0, 1)
    C. (0, 2)
    D. (0, 3)
     E. (1, 1)
     F. (1, 2)
    G. (1, 3)
    H. (2, 2)
      I. (2, 3)
     J. None of the above

<!-- answer:start -->
exact
B
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** B
<!-- explanation:end -->

## q44 [easy]

**Question 44 [0.5 marks]**

[Question Group: Game Tree &Minimax]

Which of the following state(s) is/are the children of state (0, 3)? Select all that is/are true.

   A. (0, 0)
     B. (0, 1)
   C. (0, 2)
    D. (0, 3)
     E. (1, 1)
     F. (1, 2)
    G. (1, 3)
    H. (2, 2)
      I. (2, 3)
     J. None of the above

<!-- answer:start -->
exact
AC
A C
A,C
A, C
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** A,C
<!-- explanation:end -->

## q45 [easy]

**Question 45 [0.5 marks]**

[Question Group: Game Tree &Minimax]

Which of the following state(s) is/are the children of state (1, 1)? Select all that is/are true.

   A. (0, 0)
   B. (0, 1)
    C. (0, 2)
    D. (0, 3)
     E. (1, 1)
     F. (1, 2)
    G. (1, 3)
    H. (2, 2)
      I. (2, 3)
     J. None of the above

<!-- answer:start -->
exact
AB
A B
A,B
A, B
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** A,B
<!-- explanation:end -->

## q46 [easy]

**Question 46 [0.5 marks]**

[Question Group: Game Tree &Minimax]

Which of the following state(s) is/are the children of state (1, 2)? Select all that is/are true.

     A. (0, 0)
   B. (0, 1)
   C. (0, 2)
    D. (0, 3)
   E. (1, 1)
     F. (1, 2)
    G. (1, 3)
    H. (2, 2)
      I. (2, 3)
     J. None of the above

<!-- answer:start -->
exact
BCE
B C E
B,C,E
B, C, E
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** B,C,E
<!-- explanation:end -->

## q47 [easy]

**Question 47 [0.5 marks]**

[Question Group: Game Tree &Minimax]

Which of the following state(s) is/are the children of state (1, 3)? Select all that is/are true.

     A. (0, 0)
   B. (0, 1)
   C. (0, 2)
   D. (0, 3)
     E. (1, 1)
   F. (1, 2)
    G. (1, 3)
    H. (2, 2)
      I. (2, 3)
     J. None of the above

<!-- answer:start -->
exact
BCDF
B C D F
B,C,D,F
B, C, D, F
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** B,C,D,F
<!-- explanation:end -->

## q48 [easy]

**Question 48 [0.5 marks]**

[Question Group: Game Tree &Minimax]

Which of the following state(s) is/are the children of state (2, 2)? Select all that is/are true.

     A. (0, 0)
     B. (0, 1)
    C. (0, 2)
    D. (0, 3)
   E. (1, 1)
   F. (1, 2)
    G. (1, 3)
    H. (2, 2)
      I. (2, 3)
     J. None of the above

<!-- answer:start -->
exact
EF
E F
E,F
E, F
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** E,F
<!-- explanation:end -->

## q49 [easy]

**Question 49 [0.5 marks]**

[Question Group: Game Tree &Minimax]

Which of the following state(s) is/are the children of state (2, 3)? Select all that is/are true.

     A. (0, 0)
     B. (0, 1)
   C. (0, 2)
    D. (0, 3)
     E. (1, 1)
   F. (1, 2)
  G. (1, 3)
   H. (2, 2)
      I. (2, 3)
     J. None of the above

<!-- answer:start -->
exact
CFGH
C F G H
C,F,G,H
C, F, G, H
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** C,F,G,H
<!-- explanation:end -->

## q50 [easy]

**Question 50 [0.5 marks]**

[Question Group: Game Tree &Minimax]

What is the value of state (1, 2) in depth 1?

   A. -1
    B. 0
    C. 1
    D. None of the above

<!-- answer:start -->
exact
A
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** A
<!-- explanation:end -->

## q51 [easy]

**Question 51 [0.5 marks]**

[Question Group: Game Tree &Minimax]

What is the value of state (2, 3) in depth 0?

    A. -1
    B. 0
  C. 1
    D. None of the above

<!-- answer:start -->
exact
C
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** C
<!-- explanation:end -->

## q52 [easy]

**Question 52 [0.5 marks]**

[Question Group: Game Tree &Minimax]

What is the value of state (0, 0) in depth 3?

    A. -1
    B. 0
  C. 1
    D. None of the above

<!-- answer:start -->
exact
CC
C C
C,C
C, C
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** C,C
<!-- explanation:end -->

## q54 [easy]

**Question 54 [0.5 marks]**

[Question Group: Game Tree &Minimax]

What is the value of state (0, 0) in depth 5?

    A. -1
    B. 0
  C. 1
    D. None of the above

<!-- answer:start -->
exact
C
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** C
<!-- explanation:end -->

## q55 [easy]

**Question 55 [0.5 marks]**

[Question Group: Game Tree &Minimax]

Which player do we expect to win?

   A. First player
    B. Second player
    C. Draw
    D. It depends

<!-- answer:start -->
exact
AM
A M
A,M
A, M
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** A,M

See next question.
<!-- explanation:end -->

## q57 [hard]

**Question 57 [4.0 marks]**

[Question Group: Alpha-beta Pruning]

Suppose we traverse this tree with DFS from left-to-right. Select all the link(s) that would be pruned by
alpha-beta pruning algorithm. Select only the links that are directly pruned and not those that are
indirectly pruned because they are in a subtree of a pruned link.
Which of the following link(s) is/are pruned? Select all that is/are true.

    A. a
    B. b
    C. c
    D. d
    E. e
   F. f
  G. g
    H. h
     I. i
     J. j
    K. k
     L. l
    M. m
    N. n
    O. o
    P. p
    Q. q
    R. r
  S. s
   T. t
    U. u
  V. v
    W. w
    X. None of the above

<!-- answer:start -->
exact
FGSTV
F G S T V
F,G,S,T,V
F, G, S, T, V
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** F,G,S,T,V

Item Weight: 4.0
____________________________________________________________________________
<!-- explanation:end -->

## q58 [hard]

**Question 58 [4.0 marks]**

[Question Group: Alpha-beta Pruning]

Suppose we traverse this tree with DFS from right-to-left. Select all the link(s) that would be pruned by
alpha-beta pruning algorithm. Select only the links that are directly pruned and not those that are
indirectly pruned because they are in a subtree of a pruned link.
Which of the following link(s) is/are pruned? Select all that is/are true.

    A. a
    B. b
    C. c
    D. d
    E. e
    F. f
    G. g
    H. h
     I. i
     J. j
    K. k
     L. l
    M. m
    N. n

    O. o
    P. p
    Q. q
  R. r
  S. s
    T. t
    U. u
    V. v
   W. w
    X. None of the above

<!-- answer:start -->
exact
RS
R S
R,S
R, S
<!-- answer:end -->
<!-- explanation:start -->
**Supplied answer:** R,S
<!-- explanation:end -->
