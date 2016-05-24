pub struct Vertex<'a, T: 'a> {
    pub data: &'a T,
    pub id: i32
}

impl<'a, T: 'a> Vertex<'a, T> {
    pub fn new(data: &'a T, id: i32) -> Vertex<'a, T> {
        Vertex {
            data: data,
            id: id
        }
    }
}
