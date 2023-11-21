#[cfg(all(feature = "notify", target_family = "windows"))]
use super::modes::{mix::Mix, notify::Notify};
use super::{modes::poll::Poll, node::Node};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Mode {
    #[cfg(feature = "poll")]
    Poll(Poll),
    #[cfg(all(feature = "notify", target_family = "windows"))]
    Notify(Notify),
    #[cfg(all(feature = "poll", feature = "notify", target_family = "windows"))]
    Mix(Mix),
}

impl Default for Mode {
    #[cfg(all(feature = "notify", feature = "poll", target_family = "windows"))]
    fn default() -> Self {
        return Self::Mix(Mix::default());
    }

    #[cfg(all(feature = "notify", target_family = "windows", not(feature = "poll")))]
    fn default() -> Self {
        return Self::Notify(Notify::default());
    }

    #[cfg(all(
        feature = "poll",
        not(feature = "notify"),
        not(target_family = "windows")
    ))]
    fn default() -> Self {
        return Self::Poll(Poll::default());
    }
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
struct Config {
    verbosity: usize,
    mode: Mode,
    #[serde(flatten)]
    nodes: Vec<Box<Node>>,
}
