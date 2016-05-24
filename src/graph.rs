use std::collections::{HashMap, HashSet};
use vertex::Vertex;

/// A directed graph. Each edge has a source and target vertex.
pub struct DirectedGraph<'a, T: 'a> {
    graph: HashMap<i32, HashSet<i32>>,
    lookup: HashMap<i32, &'a T>,
    id_count: i32,
}

impl<'a, T: 'a> DirectedGraph<'a, T> {
    /// Constructs an empty directed graph.
    pub fn new() -> DirectedGraph<'a, T> {
        DirectedGraph {
            graph: HashMap::new(),
            lookup: HashMap::new(),
            id_count: 0
        }
    }

    /// Adds a vertex labeled with the input data, and returns a Vertex
    /// that can be used to query against the graph later.
    pub fn add_vertex(&mut self, data: &'a T) -> Vertex<'a, T> {
        self.id_count += 1;
        let v = Vertex::new(data, self.id_count);
        self.lookup.insert(self.id_count, data);
        v
    }

    /// Given two vertices, `u` and `v`, creates an edge `u -> v`.
    pub fn add_edge(&mut self, v: &Vertex<'a, T>, u: &Vertex<'a, T>) -> () {
        let neighbors = self.graph.entry(v.id).or_insert(HashSet::new());
        neighbors.insert(u.id);
        ()
    }

    /// Returns `true` i.f.f. there is an edge `u -> v`.
    pub fn is_edge(&mut self, v: &Vertex<'a, T>, u: &Vertex<'a, T>) -> bool {
        let neighbors = self.graph.entry(v.id).or_insert(HashSet::new());
        neighbors.contains(&u.id)
    }
}
