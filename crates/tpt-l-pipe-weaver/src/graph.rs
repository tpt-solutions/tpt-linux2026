//! The DSP graph: nodes, ports, and connections.

use crate::{Connection, Error};
use std::collections::HashMap;

/// A DSP processing node with a fixed number of input and output ports.
#[derive(Debug, Clone)]
pub struct Node {
    pub id: u32,
    pub name: String,
    pub inputs: u32,
    pub outputs: u32,
}

/// A builder/container for a DSP graph.
#[derive(Debug, Default)]
pub struct Graph {
    nodes: HashMap<u32, Node>,
    connections: Vec<Connection>,
}

impl Graph {
    pub fn new() -> Self {
        Graph::default()
    }

    /// Add a node; returns its id.
    pub fn add_node(&mut self, name: &str, inputs: u32, outputs: u32) -> u32 {
        let id = self.nodes.len() as u32;
        self.nodes.insert(id, Node { id, name: name.to_string(), inputs, outputs });
        id
    }

    /// Connect an output port of `from` to an input port of `to`.
    pub fn connect(
        &mut self,
        from: u32,
        from_port: u32,
        to: u32,
        to_port: u32,
        ring_capacity: usize,
    ) -> Result<(), Error> {
        let f = self.nodes.get(&from).ok_or(Error::NoNode(from))?;
        let t = self.nodes.get(&to).ok_or(Error::NoNode(to))?;
        if from_port >= f.outputs {
            return Err(Error::NoPort(from_port, from));
        }
        if to_port >= t.inputs {
            return Err(Error::NoPort(to_port, to));
        }
        if self.would_cycle(from, to) {
            return Err(Error::Cycle);
        }
        self.connections.push(Connection::new(from, from_port, to, to_port, ring_capacity));
        Ok(())
    }

    /// Detect whether adding an edge from->to would create a cycle by checking
    /// if `to` can already reach `from`.
    fn would_cycle(&self, from: u32, to: u32) -> bool {
        if from == to {
            return true;
        }
        // Simple DFS over existing connections.
        let mut stack = vec![to];
        let mut seen = std::collections::HashSet::new();
        while let Some(n) = stack.pop() {
            if n == from {
                return true;
            }
            if !seen.insert(n) {
                continue;
            }
            for c in &self.connections {
                if c.from_node == n {
                    stack.push(c.to_node);
                }
            }
        }
        false
    }

    /// Iterate connections.
    pub fn connections(&self) -> &[Connection] {
        &self.connections
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }
}
