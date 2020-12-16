---
marp: true
---
<style>
img[alt~="center"] {
  display: block;
  margin: 0 auto;
}
</style>

<!--
What you need to tell us:
1. what method you use (dont need to explain how it works)
    1. Alpha beta with heuristics
    1.
2. Any unique aspects
    1. written in rust: faster runtime so we can search deeper
    1. genetic algorithm to find weights
3. Any literature which has informed your decision process
4. what have you up to now achieved including evidence for quality (eg performance agaisnt test bots)
5. Any experiments used to inform decisions, or for evaluation
6. quantitative measures of performance (eg depth of seach, number of playouts per move, training time etc)
7. what you expect to have accomplished by submission
8. each member of the group should say something about the content, eg their contribution to the project
-->

# Group 23

## Karl Meakin & Ben Maxwell

---

# Our approach
## Communicating with the engine
We used a parser generator to parse messages recieved from the engine. This was faster to setup and more robust than manually splitting and searching the string.

---

# Selecting moves
Once a message is recieved from the game engine, update the agent's internal model of the state of the board, and consider which of the available moves to select next

We used Alpha-Beta search guided by heuristics:
1. Difference between player's score and opponent's score
2. Difference between number of stones player can capture and number of stones opponent can capture
3. Difference between player's chaining potential (moves that allow the player to have a second turn) and opponent's chaining potential
4. Difference in seeds on each side (avoid starvation)

These heuristics are used to evaluate the board at the depth limit, and to decide the order to search child nodes.

---

# Unique aspects

## Rust
We chose to implement our bot in Rust (a systems programming language with compile time memory safey checks) to guarantee high performance vs Java or Python.

## Weights finding
We used a genetic algorithm library to find weights for our heuristic to maximise win margin when playing against the provided test bots.

---
<!--
We found a tool that allowed us to specify a cost function, and then run a genetic algorithm to (attempt to) find the maximum of that function. We made a function that takes the weights vector for our agent and then plays a game against a [????] test bot, and uses our score as the result.  -->

![center](./ga_res.png)

```
 The best solution found:
 [REDACTED]

 Objective function:
 -58.0
```

---

# Performance

![center](./ga_matches.png)

---

# Performance

![center](./depth_and_score.png)

