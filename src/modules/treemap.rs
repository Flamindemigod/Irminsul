use crate::utils::get_last_modified::get_last_modified;
use log::trace;
use rayon::iter::*;
use std::{collections::BTreeSet, fmt::Display, hash::Hash, path::PathBuf, time::SystemTime};

use super::node::Node;

#[derive(Clone, Debug, PartialOrd, Eq, Ord, Hash)]
pub struct Treemap {
    pub full_path: PathBuf,
    pub node: PathBuf,
    pub branches: Vec<Box<Treemap>>,
    last_update: Option<Box<SystemTime>>,
    conf_node: Vec<Box<Node>>,
}

impl PartialEq for Treemap {
    fn eq(&self, other: &Self) -> bool {
        if self.full_path != other.full_path {
            return false;
        }

        if self.node != other.node {
            return false;
        }

        if self.last_update != other.last_update {
            return false;
        }
        {
            let a: BTreeSet<_> = self.branches.par_iter().collect();
            let b: BTreeSet<_> = other.branches.par_iter().collect();

            if a != b {
                return false;
            }
        }

        {
            let a: BTreeSet<_> = self.conf_node.par_iter().collect();
            let b: BTreeSet<_> = other.conf_node.par_iter().collect();

            if a != b {
                return false;
            }
        }

        return true;
    }
}

impl Display for Treemap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "-{} ", self.node.display())?;
        for branch in &self.branches {
            write!(f, "| ")?;
            branch.fmt(f)?;
        }
        Ok(())
    }
}

impl Treemap {
    pub fn new(node: PathBuf, branches: Vec<Box<Treemap>>, prev_path: PathBuf) -> Self {
        let full_path;
        if cfg!(windows) && node == PathBuf::from("WinRoot") {
            full_path = prev_path;
        } else {
            full_path = prev_path.join(node.clone())
        }
        let mut val = Self {
            node: node,
            branches,
            last_update: None,
            full_path: full_path,
            conf_node: Vec::new(),
        };
        val.poll_point();
        return val;
    }

    pub fn merge(&mut self, other: &mut Self) {
        let mut branches = Vec::new();
        '_outer: loop {
            if self.branches.is_empty() {
                break;
            }
            let mut branch = self.branches.pop().unwrap();

            other.branches = other
                .branches
                .iter_mut()
                .filter_map(|b_branch| {
                    if b_branch.full_path == branch.full_path {
                        branch.merge(b_branch);
                        None
                    } else {
                        Some(b_branch.to_owned())
                    }
                })
                .collect::<Vec<_>>();
            branches.push(branch);
        }
        branches.append(&mut other.branches);
        self.branches = branches;
    }

    pub fn link_conf_node(&mut self, conf_node: Box<Node>) -> &mut Self {
        self.conf_node.push(conf_node);
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
        println!("No Update for {}", self.node.display());
        return false;
    }

    pub fn poll_branches(&mut self) -> Vec<PathBuf> {
        trace!("Polling Branches of {}", self.node.display());
        let mut update: Vec<PathBuf> = Vec::new();
        if self.node == PathBuf::from("WinRoot")
            || self.node == PathBuf::from("/")
            || self.poll_point()
        {
            if !self.branches.is_empty() {
                self.branches.iter_mut().for_each(|b| {
                    let paths = b.poll_branches();
                    update = paths;
                })
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
