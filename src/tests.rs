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

    let neighbors = g.neighbors(&d1);

    assert!(neighbors.contains(&d2));
}

#[test]
fn edge_complete() {
    let d = vec![1, 2, 3, 4, 5];

    let mut g = DirectedGraph::new();

    for v in &d {
        g.add_vertex(v);
    }

    for vo in &d {
        for vi in &d {
            if vo != vi {
                g.add_edge(&vi, &vo);
            }
        }
    }

    for vo in &d {
        let neighbors = g.neighbors(&vo);
        for vi in &d {
            if vo != vi {
                assert!(neighbors.contains(&vi));
                assert!(g.is_edge(&vi, &vo));
            } else {
                assert!(!neighbors.contains(&vi));
                assert!(!g.is_edge(&vi, &vo));
            }
        }
    }
}
