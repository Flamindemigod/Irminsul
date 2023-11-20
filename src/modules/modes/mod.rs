use std::path::PathBuf;

use super::treemap::Treemap;

#[cfg(all(feature = "poll", feature = "notify"))]
pub mod mix;
#[cfg(feature = "notify")]
pub mod notify;
#[cfg(feature = "poll")]
pub mod poll;

pub trait Poll {
    fn poll(&self, files: &mut Vec<Box<Treemap>>) -> Option<Vec<PathBuf>>;
}

trait PollMap<T> {
    fn poll_map(&mut self, branch_depth_ratio: f32) -> Vec<&mut Self>;
}
