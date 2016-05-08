use vertex::generic::*;

#[test]
fn basic() {
    let v = Vertex::new(1);
    assert_eq!(1, v.data);
}
