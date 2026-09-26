# Intelligent Agents — MCQ

Reply with the option letter only.

## q1 [easy]

What does PEAS stand for?

A. Performance, Execution, Actions, States  
B. Performance measure, Environment, Actuators, Sensors  
C. Percepts, Environment, Agent, Search  
D. Planning, Evaluation, Actions, Success

<!-- answer:start -->
exact
B
<!-- answer:end -->
<!-- explanation:start -->
B is correct: PEAS specifies success, surroundings, outputs, and inputs. A, C, and D substitute terms outside the framework.
<!-- explanation:end -->

## q2 [medium]

Which statement correctly distinguishes an agent function from an agent program?

A. The function is code; the program is a mathematical mapping.  
B. The function maps percept histories to actions; the program implements it on an architecture.  
C. The function defines sensors; the program defines actuators.  
D. They are exactly the same thing.

<!-- answer:start -->
exact
B
<!-- answer:end -->
<!-- explanation:start -->
B is correct. A reverses the definitions, C confuses both with PEAS, and D ignores the abstract-versus-implementation distinction.
<!-- explanation:end -->

## q3 [easy]

Which list contains the four common agent structures?

A. Reflex, goal-based, utility-based, learning  
B. Static, dynamic, deterministic, stochastic  
C. BFS, DFS, UCS, A*  
D. Sensor, actuator, critic, environment

<!-- answer:start -->
exact
A
<!-- answer:end -->
<!-- explanation:start -->
A is correct. B lists environment properties, C lists search algorithms, and D mixes unrelated components.
<!-- explanation:end -->

## q4 [medium]

A robot knows its exact state, has predictable actions, works alone, makes choices that affect later choices, and plans in an unchanged world. Which classification fits?

A. Partially observable, stochastic, multi-agent, episodic, dynamic  
B. Fully observable, deterministic, single-agent, sequential, static  
C. Fully observable, stochastic, single-agent, episodic, static  
D. Partially observable, deterministic, multi-agent, sequential, dynamic

<!-- answer:start -->
exact
B
<!-- answer:end -->
<!-- explanation:start -->
B matches every property. A is the opposite; C conflicts with predictability and dependent choices; D conflicts with exact state knowledge, working alone, and an unchanged world.
<!-- explanation:end -->

## q5 [medium]

What is the correct order for a problem-solving agent?

A. Search → goal formulation → execute → problem formulation  
B. Goal formulation → problem formulation → search → execute  
C. Problem formulation → execute → goal formulation → search  
D. Goal formulation → search → problem formulation → execute

<!-- answer:start -->
exact
B
<!-- answer:end -->
<!-- explanation:start -->
B is correct: decide what is wanted, define the problem, find a solution, then act. The other options search or execute too early.
<!-- explanation:end -->

## q6 [hard]

Why is “reach the destination” insufficient as the full performance measure for an autonomous taxi?

A. It must also evaluate how safely and legally the destination is reached.  
B. Performance measures can contain only distances.  
C. A destination is an actuator.  
D. Performance measures describe sensors.

<!-- answer:start -->
exact
A
<!-- answer:end -->
<!-- explanation:start -->
A is correct: success includes journey quality. B adds a false restriction, C misclassifies a goal as hardware, and D confuses evaluation with perception.
<!-- explanation:end -->

## q7 [medium]

Which learning-agent component proposes exploratory actions that may produce useful experience?

A. Critic  
B. Performance element  
C. Problem generator  
D. Learning element

<!-- answer:start -->
exact
C
<!-- answer:end -->
<!-- explanation:start -->
C is correct. The critic evaluates, the performance element acts, and the learning element improves behavior using feedback.
<!-- explanation:end -->

## q8 [hard]

Which statement best captures exploration versus exploitation?

A. Exploration uses the known best action; exploitation chooses randomly.  
B. Exploration gathers information; exploitation uses current knowledge for reward now.  
C. Exploration occurs only in deterministic environments.  
D. Exploitation guarantees the global optimum immediately.

<!-- answer:start -->
exact
B
<!-- answer:end -->
<!-- explanation:start -->
B is correct. A reverses the terms, C adds a false restriction, and D ignores incomplete knowledge.
<!-- explanation:end -->
