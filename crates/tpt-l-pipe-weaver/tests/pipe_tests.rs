#[cfg(test)]
mod tests {
    use tpt_l_pipe_weaver::graph::Graph;
    use tpt_l_pipe_weaver::*;

    #[test]
    fn build_and_connect() {
        let mut g = Graph::new();
        let src = g.add_node("source", 0, 1);
        let sink = g.add_node("sink", 1, 0);
        g.connect(src, 0, sink, 0, 16).unwrap();
        assert_eq!(g.node_count(), 2);
        assert_eq!(g.connections().len(), 1);
    }

    #[test]
    fn rejects_cycle() {
        let mut g = Graph::new();
        let a = g.add_node("a", 1, 1);
        let b = g.add_node("b", 1, 1);
        let c = g.add_node("c", 1, 1);
        g.connect(a, 0, b, 0, 16).unwrap();
        g.connect(b, 0, c, 0, 16).unwrap();
        // c -> a would close a cycle.
        assert_eq!(g.connect(c, 0, a, 0, 16), Err(Error::Cycle));
    }

    #[test]
    fn ring_buffer_transfer() {
        let conn = Connection::new(0, 0, 1, 0, 4);
        assert!(conn.push(1.0).is_ok());
        assert!(conn.push(2.0).is_ok());
        assert_eq!(conn.pop().unwrap(), 1.0);
        assert_eq!(conn.pop().unwrap(), 2.0);
        assert_eq!(conn.pop(), Err(Error::Empty));
    }
}
