pub mod syzoj;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum OnlineJudge {
    #[serde(rename = "syzoj")]
    Syzoj,
}
