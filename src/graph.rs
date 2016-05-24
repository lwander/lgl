use vertex::Vertex;

pub trait Graph<'a, T> {
    fn new() -> Self;
    fn add_vertex(&mut self, &'a T) -> Vertex<'a, T>;
    fn add_edge(&mut self, &Vertex<'a, T>, &Vertex<'a, T>) -> ();
    fn is_edge(&mut self, &Vertex<'a, T>, &Vertex<'a, T>) -> bool;
}
