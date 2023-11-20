use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, time::Duration};

use crate::modules::treemap::Treemap;

use super::{Poll as PollTrait, PollMap};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Poll {
    poll_rate: Duration,
}

impl Default for Poll {
    fn default() -> Self {
        Self {
            poll_rate: Duration::from_millis(500),
        }
    }
}

impl PollTrait for Poll {
    fn poll(&self, path_map: &mut Vec<Box<Treemap>>) -> Option<Vec<std::path::PathBuf>> {
        let res: Vec<PathBuf> = path_map
            .par_iter_mut()
            .map(|p| {
                <Treemap as PollMap<Poll>>::poll_map(p, 0.0)
                    .par_iter_mut()
                    .map(|point| point.poll_branches())
                    .flatten()
                    .collect::<Vec<PathBuf>>()
            })
            .flatten()
            .collect();
        if !res.is_empty() {
            return Some(res);
        } else {
            return None;
        }
    }
}

impl PollMap<Poll> for Treemap {
    fn poll_map(&mut self, _: f32) -> Vec<&mut Self> {
        if self.branches.is_empty() {
            return vec![self];
        } else {
            return self
                .branches
                .par_iter()
                .map(|mut t| <Treemap as PollMap<Poll>>::poll_map(t.as_mut(), 0.0))
                .flatten()
                .collect::<Vec<_>>();
        }
    }
}
