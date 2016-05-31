use std::collections::{HashMap, HashSet};
use std::hash::Hash;

/// A directed graph. Each edge has a source and target vertex.
///
/// # Warning
///
/// This implementation is not thread-safe
pub struct DirectedGraph<'a, T: 'a + Eq + Hash> {
    // Map of vertex IDs -> neighbor vertex ids.
    graph: HashMap<i32, HashSet<i32>>,
    // Vertex -> ID mapping.
    v2i: HashMap<&'a T, i32>,
    // ID -> vertex mapping.
    i2v: HashMap<i32, &'a T>,
    // Monotonically increasing count for labeling vertices.
    id_count: i32,
}

impl<'a, T: 'a + Eq + Hash> DirectedGraph<'a, T> {
    /// Constructs an empty directed graph.
    pub fn new() -> DirectedGraph<'a, T> {
        DirectedGraph {
            graph: HashMap::new(),
            v2i: HashMap::new(),
            i2v: HashMap::new(),
            id_count: 0
        }
    }

    /// Adds a vertex labeled with the input vertex. 
    ///
    /// Returns `true` i.f.f. this vertex was not previously in the graph.
    pub fn add_vertex(&mut self, v: &'a T) -> () {
        self.get_vertex_id(v);
        ()
    }

    fn new_id(&mut self) -> i32 {
        self.id_count += 1;
        self.id_count
    }

    /// Get the id of the associated vertex. If the vertex is not in the graph,
    /// add it, and then return its id.
    fn get_vertex_id(&mut self, v: &'a T) -> i32 {
        let exists = self.v2i.contains_key(v);
        let id;

        if !exists {
            id = self.new_id();
            self.v2i.insert(v, id);
            self.i2v.insert(id, v);
        } else {
            id = match self.v2i.get(v) {
                Some(id) => *id,
                None => panic!("Vertex disappeared during lookup")
            }
        }

        id
    }

    /// Given two vertices, `u` and `v`, creates an edge `u -> v`.
    ///
    /// These vertices do not need to be in the graph before calling this 
    /// method for this work.
    pub fn add_edge(&mut self, v: &'a T, u: &'a T) -> () {
        let vid = self.get_vertex_id(v);
        let uid = self.get_vertex_id(u);

        let neighbors = self.graph.entry(vid).or_insert(HashSet::new());
        neighbors.insert(uid);
        ()
    }

    /// Returns `true` i.f.f. there is an edge `u -> v`.
    pub fn is_edge(&mut self, v: &'a T, u: &'a T) -> bool {
        let vid = self.get_vertex_id(v);
        let uid = self.get_vertex_id(u);

        let neighbors = self.graph.entry(vid).or_insert(HashSet::new());
        neighbors.contains(&uid)
    }

    /// Get the set of neighbors for a given vertex.
    pub fn neighbors(&mut self, v: &'a T) -> HashSet<&'a T> {
        if !self.v2i.contains_key(v) {
            panic!("Attempt made to get neighbors of a missing vertex")
        } else {
            let id = self.get_vertex_id(v);
            let neighbors = self.graph.entry(id).or_insert(HashSet::new());
            let i2v = &self.i2v;

            neighbors.iter().map( |i| {
                match i2v.get(i) {
                    Some(vp) => *vp,
                    None => panic!("Corrupted graph, missing neighbor")
                }
            }).collect()
        }
    }
}
