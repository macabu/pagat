# pagat
A library that helps you split the bill; made for learning purposes, not production-ready.

## Installation
```toml
[dependencies]
pagat = "0.0.1"
```

## Concepts
This crate has the following concepts:
- `Person`: someone who participates in the bill splitting;
- `Money`: i32 for money calculations, using 2 decimals for cents (such that 100 = $1.00)
- `Payment`: payment made by someone that can involves up to N amount of people
    - maybe you took a cab with everyone but `D`, so you can record this payment to `B` and `C` only
- `Obligation`: the record that says someone has to pay someone else a certain amount of money
    - this is used as the output of the graph solver

## Usage & Examples
Please [refer to the tests](src/lib.rs#13) in order to see different use cases.

## How it works
It uses directed graphs to represent who needs to pay whom how much.
However, such representation is not well optimized and would require many people to pay others and keep track of the money, which defeats the purpose of the library.  
  
The [Solver](src/solver.rs) optimizes the connections in such a way that you have the minimum amount of payments being made, optimistically we can have N-1 payments where N is the amount of people involved.  
  
Implementation-wise, there are four passes the solver executes.

### First Pass
Reduces doubly connected edges to a single edge connection.
The resulting direction is dictated by subtracting the edges' weights.
In case the result is zero, then both edges are removed.

Consider the following example, where `A` has to pay `B` $10, and `B` has to pay `A` $20:
```
┌─────┐     $10       ┌─────┐
│     ├──────────────►│     │
│ A   │               │ B   │
│     │◄──────────────┤     │
└─────┘     $20       └─────┘

```
After the first pass, we would have only `B` paying `A` $10
```
┌─────┐               ┌─────┐
│     │               │     │
│ A   │               │ B   │
│     │◄──────────────┤     │
└─────┘     $10       └─────┘
```
In the case where `A` and `B` would pay the same, then both connections are removed.

### Second Pass
After the first pass, we are guaranteed to not have doubly connected edges, and we can move on to the second pass.  
By taking and an edge, we check if the target of that edge has an edge to yet another node that our source edge has as well. If we find it, we remove the first edge, add update the weights of the remaining edges (what a mouthful) 
Let's see an example:
```
     ┌───┐       ┌───┐
     │   │ $25   │   │
     │ H ├──────►│ C │
     │   │       │   │
     └─┬─┘       └─┬─┘
       │           │
   $50 │   ┌───┐   │ $50
       │   │   │   │
       └──►│ A │◄──┘
           │   │
           └───┘
```
The steps are the following:
- Remove `H->C`
- Add its weight (`$25`) to `H->A`
- Subtract its weight (`$25`) from `H->C`
  
With that the resulting graph is:
```
   ┌───┐       ┌───┐
   │   │       │   │
   │ H │       │ C │
   │   │       │   │
   └─┬─┘       └─┬─┘
     │           │
 $75 │   ┌───┐   │ $25
     │   │   │   │
     └──►│ A │◄──┘
         │   │
         └───┘
```
The reasoning behind this optimization is simple: `H` doesn't need to pay `C`, if both `H` and `C` pay to the same person `A`.  
In case the subtraction step is negative, we simply invert the direction of the edge.

### Third Pass
If `A` is paying `B` $10, and `B` is paying `C` that same amount, we can reduce it so that `A` pays `C` directly the $10.
More technically speaking, if there's an edge A --[X]--> B and another B --[X]--> C, it can be reduced to A --[X]--> C.

### Fourth Pass
To make things a bit faster, we don't actually remove any connections, but set their weight to `0`.
It is in this step where we do the removal, after all other three steps.

## TODO
- Improve in-code docs for Rust docs
- Add proper examples
