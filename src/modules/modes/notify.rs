use std::time::Duration;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Notify{
    poll_rate: Duration,
}

impl Default for Notify {
    fn default() -> Self {
        Self { poll_rate: Duration::from_millis(200) }
    }
}