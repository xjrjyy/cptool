use serde::{Deserialize, Serialize};

// https://github.com/syzoj/syzoj/blob/573796fa7670e28d428692f1d91e7ea50ee154e5/utility.js#L192

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum SubtaskType {
    #[serde(rename = "sum")]
    Sum,
    #[serde(rename = "min")]
    Min,
    #[serde(rename = "mul")]
    Mul,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Subtask {
    #[serde(rename = "type")]
    pub subtask_type: SubtaskType,
    pub score: f64,
    pub cases: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Problem {
    #[serde(rename = "inputFile")]
    pub input_file: Option<String>,
    #[serde(rename = "outputFile")]
    pub output_file: Option<String>,
    #[serde(rename = "userOutput")]
    pub answer_file: Option<String>,

    pub subtasks: Vec<Subtask>,
    // TODO: specialJudge
}
