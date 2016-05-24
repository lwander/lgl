# lgl

Small graph library written in [Rust](https://rust-lang.org).

## Motivation

I read that writing a graph library in Rust is difficult do Rust's builtin
ownership and borrowing, since each vertex can be referenced by an arbitrary 
number of other vertices. So I figured I'd give it a shot to learn the
language. Plus, this will hopefully serve as a useful utility for compiler
writing in Rust later on.

## Usage


```
let d1 = 1;
let d2 = 2;

let mut g = DirectedGraph::new();

let v1 = g.add_vertex(&d1);
let v2 = g.add_vertex(&d2);

g.add_edge(&v1, &v2);

assert!(g.is_edge(&v1, &v2));
assert!(!g.is_edge(&v2, &v1));
```

## Testing

```
$ cargo test
```
