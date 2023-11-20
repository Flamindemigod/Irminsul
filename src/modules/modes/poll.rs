use std::time::Duration;

use serde::{Deserialize, Serialize};

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
