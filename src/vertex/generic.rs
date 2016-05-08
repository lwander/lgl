pub struct Vertex<'a, T: 'a> {
    pub data: &'a T,
    id: i32
}

impl<'a, T: 'a> Vertex<'a, T> {
    pub fn new(d: &'a T) -> Vertex<'a, T> {
        Vertex {
            data: d,
            id: 0
        }
    }
}
