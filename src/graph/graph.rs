use super::port::{Port, GType, GVal};
use crate::error::DmGuiError;
use log::error;
use std::{
    collections::HashMap,
    fmt::Debug,
    sync::RwLock,
};
use super::node::{Node, ConnectionTo};

#[derive(Debug)]
pub struct EvaluationGraph {
    next_id: usize,
    unused: Vec<usize>,
    pub props: HashMap<&'static str, GVal>,
    pub graph_outp: HashMap<usize, GVal>,
    pub nodes: HashMap<usize, RwLock<Node>>,
}

impl EvaluationGraph {
    pub fn new() -> Self {
        Self {
            next_id: 0,
            props: HashMap::new(),
            graph_outp: HashMap::new(),
            nodes: HashMap::new(),
            unused: Vec::new(),
        }
    }

    /*
    pub fn add_node(&mut self, new: Node) -> Result<usize, DmGuiError> {
        let nid = if let Some(last) = self.unused.pop() {
            last
        } else {
            self.next_id += 1;
            self.next_id
        };
        Ok(self.set_node(nid, new))
    }
    pub fn delete_node(&mut self, id: usize) -> Option<RwLock<Node>> {
        self.nodes.remove(&id).map(|c| c.into())
    }
    pub fn get_node_by_id(&self, id: usize) -> Option<&RwLock<Node>> {
        self.nodes.get(&id)
    }
    pub fn set_node(&mut self, id: usize, new: Node) -> usize {
        self.nodes.insert(id, RwLock::new(new));
        id
    }

    pub fn set_prop(&mut self, s: &'static str, v: GVal) {
        self.props.insert(s, v);
    }

    pub fn add_connection(&mut self, node_id: usize, con: ConnectionTo) -> Option<()> {
        if self.nodes.contains_key(&node_id) && self.nodes.contains_key(&con.end_node) {
            let (mut s_node, e_node) = (
                self.nodes[&node_id].write().unwrap(),
                self.nodes[&con.end_node].read().unwrap(),
            );
            if con.start_port < s_node.outp.len() && con.end_port < e_node.inp.len() {
                let (s_port, e_port) = (&s_node.outp[con.start_port], &e_node.inp[con.end_port]);
                let (st, et): (GType, GType) = ((&s_port.gval).into(), (&e_port.gval).into());
                if st == et {
                    s_node.adjacents.push(con);
                    Some(())
                } else {
                    error!("Port types are not identical");
                    None
                }
            } else {
                error!("Ports do not exist");
                None
            }
        } else {
            error!("Nodes do not exist");
            None
        }
    }
    pub fn delete_connection(
        &mut self,
        node_id: usize,
        con: &ConnectionTo,
    ) -> Option<ConnectionTo> {
        if self.nodes.contains_key(&node_id) && self.nodes.contains_key(&con.end_node) {
            let (mut s_node, e_node) = (
                self.nodes[&node_id].write().unwrap(),
                self.nodes[&con.end_node].read().unwrap(),
            );
            if con.start_port < s_node.outp.len() && con.end_port < e_node.inp.len() {
                s_node.adjacents.retain(|c| c != con);
                Some(con.clone())
            } else {
                error!("Ports do not exist");
                None
            }
        } else {
            error!("Nodes do not exist");
            None
        }
    }
    */
}
