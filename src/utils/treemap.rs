use super::get_last_modified::get_last_modified;
use log::trace;
use std::{fmt::Display, hash::Hash, path::PathBuf, time::SystemTime};

#[derive(Clone, Debug, PartialOrd, Eq, Ord, Hash)]
pub struct Treemap {
    pub full_path: PathBuf,
    pub node: PathBuf,
    pub depth: usize,
    pub branches: Vec<Box<Treemap>>,
    last_update: Box<SystemTime>,
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

impl PartialEq for Treemap {
    fn eq(&self, other: &Self) -> bool {
        let mut a: Vec<_> = self.branches.to_owned();
        let mut b: Vec<_> = other.branches.to_owned();
        a.sort();
        b.sort();
        if a.eq(&b) && self.node.eq(&other.node) && self.depth.eq(&other.depth) && self.full_path.eq(&other.full_path){
            return true
        }
        else {
            return false
        }
    }
}


impl Treemap {
    pub fn new(
        node: PathBuf,
        depth: usize,
        branches: Vec<Box<Treemap>>,
        prev_node: PathBuf,
    ) -> Self {
        let full_path = prev_node.join(&node);
        let last_update =
            get_last_modified(full_path.clone()).expect("Failed to get Last Update Time");
        Self {
            node,
            depth,
            branches,
            last_update: Box::new(last_update),
            full_path,
        }
    }

    pub fn poll_point(&mut self) -> bool {
        let t_time =
            get_last_modified(self.full_path.clone()).expect("Failed to get Last Update Time");
        if self.last_update.lt(&Box::new(t_time)) {
            *self.last_update = t_time;
            return true;
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
