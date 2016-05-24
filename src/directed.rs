use std::collections::HashMap;
use std::collections::HashSet;
use vertex::Vertex;
use graph::Graph;

pub struct DirectedGraph<'a, T: 'a> {
    graph: HashMap<i32, HashSet<i32>>,
    lookup: HashMap<i32, &'a T>,
    id_count: i32,
}

impl<'a, T: 'a> Graph<'a, T> for DirectedGraph<'a, T> {
    fn new() -> DirectedGraph<'a, T> {
        DirectedGraph {
            graph: HashMap::new(),
            lookup: HashMap::new(),
            id_count: 0
        }
    }

    fn add_vertex(&mut self, data: &'a T) -> Vertex<'a, T> {
        self.id_count += 1;
        let v = Vertex::new(data, self.id_count);
        self.lookup.insert(self.id_count, data);
        v
    }

    fn add_edge(&mut self, v: &Vertex<'a, T>, u: &Vertex<'a, T>) -> () {
        let neighbors = self.graph.entry(v.id).or_insert(HashSet::new());
        neighbors.insert(u.id);
        ()
    }

    fn is_edge(&mut self, v: &Vertex<'a, T>, u: &Vertex<'a, T>) -> bool {
        let neighbors = self.graph.entry(v.id).or_insert(HashSet::new());
        neighbors.contains(&u.id)
    }
}
