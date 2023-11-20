use crate::utils::treemap::Treemap;
use std::path::PathBuf;

#[cfg(all(feature = "poll", feature = "notify"))]
pub mod mix;
#[cfg(feature = "notify")]
pub mod notify;
#[cfg(feature = "poll")]
pub mod poll;

pub trait Poll {
    fn poll(&self, files: &mut Vec<Box<Treemap>>) -> Option<Vec<PathBuf>>;
}
