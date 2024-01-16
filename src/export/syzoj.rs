use super::Exporter;
use crate::problem::test::{GetTestBundle, TestTaskType};
use anyhow::Result;
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

impl From<TestTaskType> for SubtaskType {
    fn from(task_type: TestTaskType) -> Self {
        match task_type {
            TestTaskType::Sum => SubtaskType::Sum,
            TestTaskType::Min => SubtaskType::Min,
        }
    }
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

pub struct SyzojExporter;

impl Exporter for SyzojExporter {
    fn export(problem: &crate::problem::Problem, output_dir: &std::path::PathBuf) -> Result<()> {
        let subtasks = problem
            .test
            .tasks
            .iter()
            .map(|(_, task)| {
                let cases = task
                    .bundles
                    .iter()
                    .map(|bundle_name| {
                        let bundle = problem.get_test_bundle(bundle_name)?;
                        Ok(bundle
                            .cases
                            .iter()
                            .map(|case| {
                                case.name.clone().ok_or_else(|| {
                                    anyhow::anyhow!(
                                        "test case name not found for `{}`",
                                        case.generator_name
                                    )
                                })
                            })
                            .collect::<Result<Vec<_>>>()?)
                    })
                    .collect::<Result<Vec<_>>>()?
                    .into_iter()
                    .flatten()
                    .collect::<Vec<_>>();
                Ok(Subtask {
                    subtask_type: task.task_type.into(),
                    score: task.score,
                    cases,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        let problem = Problem {
            input_file: Some("#.in".to_string()),
            output_file: Some("#.ans".to_string()),
            answer_file: None,
            subtasks,
        };

        let yaml = serde_yaml::to_string(&problem)?;
        std::fs::write(output_dir.join("data.yml"), yaml)?;

        Ok(())
    }
}
