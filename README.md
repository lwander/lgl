# lgl

Small graph library written in [Rust](https://rust-lang.org).

## Motivation

I read that writing a graph library in Rust is difficult do Rust's builtin
ownership and borrowing, since each vertex can be referenced by an arbitrary 
number of other vertices. So I figured I'd give it a shot to learn the
language. Plus, this will hopefully serve as a useful utility for compiler
writing in Rust later on.

## Usage


```rust
let d1 = 1;
let d2 = 2;

let mut g = DirectedGraph::new();

g.add_vertex(&d1);
g.add_vertex(&d2);

g.add_edge(&d1, &d2);

let neighbors = g.neighbors(&d1);

assert!(g.is_edge(&d1, &d2));
assert!(!g.is_edge(&d2, &d1));
assert!(neighbors.contains(&d2));
```

## Testing

```
$ cargo test
```
