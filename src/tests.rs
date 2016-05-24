use vertex::*;
use graph::*;

#[test]
fn vertex_basic() {
    let id = 0;

    let contents_i32 = 1;
    let v_i32 = Vertex::new(&contents_i32, id);
    assert_eq!(contents_i32, *v_i32.data);

    let contents_str = "str";
    let v_str = Vertex::new(&contents_str, id);
    assert_eq!(contents_str, *v_str.data);
}

#[test]
fn edge_basic() {
    let d1 = 1;
    let d2 = 2;

    let mut g = DirectedGraph::new();

    let v1 = g.add_vertex(&d1);
    let v2 = g.add_vertex(&d2);

    g.add_edge(&v1, &v2);

    assert!(g.is_edge(&v1, &v2));
    assert!(!g.is_edge(&v2, &v1));
}
