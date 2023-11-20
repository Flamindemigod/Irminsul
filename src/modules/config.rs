use super::{
    modes::{mix::Mix, notify::Notify, poll::Poll},
    node::Node,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Mode {
    #[cfg(feature = "poll")]
    Poll(Poll),
    #[cfg(feature = "notify")]
    Notify(Notify),
    #[cfg(all(feature = "poll", feature = "notify"))]
    Mix(Mix),
}

impl Default for Mode {
    #[cfg(all(feature = "notify", feature = "poll"))]
    fn default() -> Self {
        return Self::Mix(Mix::default());
    }

    #[cfg(all(feature = "notify", not(feature = "poll")))]
    fn default() -> Self {
        return Self::Notify;
    }

    #[cfg(all(feature = "poll", not(feature = "notify")))]
    fn default() -> Self {
        return Self::Poll;
    }
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
struct Config {
    verbosity: usize,
    mode: Mode,
    #[serde(flatten)]
    nodes: Vec<Box<Node>>,
}
