use graph::*;

#[test]
fn edge_basic() {
    let d1 = 1;
    let d2 = 2;

    let mut g = DirectedGraph::new();

    g.add_vertex(&d1);
    g.add_vertex(&d2);

    g.add_edge(&d1, &d2);

    assert!(g.is_edge(&d1, &d2));
    assert!(!g.is_edge(&d2, &d1));
}
