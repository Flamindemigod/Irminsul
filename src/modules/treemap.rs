use crate::utils::get_last_modified::get_last_modified;
use log::trace;
use std::{collections::BTreeSet, fmt::Display, hash::Hash, path::PathBuf, time::SystemTime};

use super::node::Node;

#[derive(Clone, Debug, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct Treemap {
    pub full_path: PathBuf,
    pub node: PathBuf,
    pub depth: usize,
    pub branches: BTreeSet<Box<Treemap>>,
    prev_node: Option<Box<Treemap>>,
    last_update: Option<Box<SystemTime>>,
    conf_node: BTreeSet<Box<Node>>,
}

impl Display for Treemap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "-{} depth: {}", self.node.display(), self.depth)?;
        for branch in &self.branches {
            write!(f, "| ")?;
            branch.fmt(f)?;
        }
        Ok(())
    }
}



impl Treemap {
    pub fn new(
        node: PathBuf,
        depth: usize,
        branches: BTreeSet<Box<Treemap>>,
        prev_path: PathBuf,
    ) -> Self {
        Self {
            node,
            depth,
            branches,
            last_update: None,
            full_path: prev_path.join(&node),
            prev_node: None,
            conf_node: BTreeSet::new(),
        }
    }

    pub fn link_conf_node(&mut self, conf_node: Box<Node>) -> &mut Self {
        self.conf_node.insert(conf_node);
        self
    }

    pub fn link_prev_node(&mut self, prev_node: Option<Box<Treemap>>) -> &mut Self {
        self.prev_node = prev_node;
        self
    }

    pub fn poll_point(&mut self) -> bool {
        if self.full_path.exists() {
            let t_time = Some(Box::new(
                get_last_modified(self.full_path.clone()).expect("Failed to get Last Update Time"),
            ));
            if self.last_update.lt(&t_time) {
                self.last_update = t_time;
                return true;
            }
        } else {
            if self.last_update.ne(&Box::new(None)) {
                self.last_update = None;
                return true;
            }
        }
        return false;
    }

    pub fn poll_branches(&mut self) -> Vec<PathBuf> {
        trace!("Polling Branches of {}", self.node.display());
        let mut update: Vec<PathBuf> = Vec::new();
        if self.poll_point() {
            if !self.branches.is_empty() {
                self.branches = self
                    .branches
                    .iter()
                    .map(|b| {
                        let mut binding = b.to_owned();
                        let paths = binding.as_mut().poll_branches();
                        update = paths;
                        binding
                    })
                    .collect();
            } else {
                update.push(self.full_path.clone());
            }
        } else {
            update = Vec::new();
        }
        trace!("Update Val {:#?}", update);
        update
    }
}
