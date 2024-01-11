pub mod test;

use crate::error::{Error, Result};
use crate::export::OnlineJudge;
use crate::program::Program;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use test::{GetProgram, GetTestBundle, Test, TestBundle};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Problem {
    pub name: String,
    // TODO: move to config
    pub export_oj: Option<OnlineJudge>,
    pub programs: HashMap<String, Program>,
    pub test: Test,
    #[serde(rename = "solution")]
    pub solution_name: String,
}

impl GetProgram for Problem {
    fn get_program(&self, name: &str) -> Result<&Program> {
        self.programs.get(name).ok_or(Error::file_not_found(name))
    }
}

impl GetTestBundle for Problem {
    fn get_test_bundle(&self, name: &str) -> Result<&TestBundle> {
        self.test
            .bundles
            .get(name)
            .ok_or(Error::test_bundle_not_found(name))
    }
}

pub struct GenerateConfig {
    pub output_dir: std::path::PathBuf,
    pub subdir: bool,
}

impl Problem {
    pub fn generate(&self, config: GenerateConfig) -> Result<()> {
        let temp_dir = crate::utils::temp_dir();
        if temp_dir.exists() {
            std::fs::remove_dir_all(&temp_dir)?;
        }
        std::fs::create_dir_all(&temp_dir)?;
        if config.output_dir.exists() {
            std::fs::remove_dir_all(&config.output_dir)?;
        }

        let get_case_name = |name: &str, index: usize| {
            if config.subdir {
                format!("{}", index)
            } else {
                format!("{}-{}", name, index)
            }
        };

        for (name, test_bundle) in &self.test.bundles {
            let output_dir = if config.subdir {
                config.output_dir.join(&name)
            } else {
                config.output_dir.clone()
            };
            std::fs::create_dir_all(&output_dir)?;

            let inputs: Vec<_> = (0..test_bundle.cases.len())
                .map(|index| get_case_name(name, index))
                .map(|name| output_dir.join(format!("{}.in", &name)))
                .map(std::fs::File::create)
                .collect::<std::io::Result<_>>()?;
            test_bundle.generate(self, inputs)?;

            let inputs: Vec<_> = (0..test_bundle.cases.len())
                .map(|index| get_case_name(name, index))
                .map(|name| output_dir.join(format!("{}.in", &name)))
                .map(std::fs::File::open)
                .collect::<std::io::Result<_>>()?;
            let answers: Vec<_> = (0..test_bundle.cases.len())
                .map(|index| get_case_name(name, index))
                .map(|name| output_dir.join(format!("{}.ans", &name)))
                .map(std::fs::File::create)
                .collect::<std::io::Result<_>>()?;
            let solution = self.get_program(&self.solution_name)?;
            inputs
                .into_iter()
                .zip(answers.into_iter())
                .map(|(input, answer)| solution.run(vec![], Some(input), answer))
                .collect::<Result<_>>()?;
        }

        // TOOD: unused bundles

        if let Some(oj) = self.export_oj {
            match oj {
                OnlineJudge::Syzoj => {
                    if config.subdir {
                        return Err(Error::export_error(
                            "subdir is not supported by syzoj exporter",
                        ));
                    }
                    use crate::export::syzoj;
                    let subtasks = self
                        .test
                        .tasks
                        .iter()
                        .map(|(name, task)| {
                            let bundles = task
                                .bundles
                                .iter()
                                .map(|name| self.get_test_bundle(name))
                                .collect::<Result<Vec<_>>>()?;
                            let cases = bundles
                                .iter()
                                .flat_map(|bundle| {
                                    (0..bundle.cases.len()).map(|index| get_case_name(name, index))
                                })
                                .collect::<Vec<_>>();
                            Ok(syzoj::Subtask {
                                // TODO: task type
                                subtask_type: syzoj::SubtaskType::Min,
                                score: task.score,
                                cases,
                            })
                        })
                        .collect::<Result<Vec<_>>>()?;
                    let problem = syzoj::Problem {
                        input_file: Some("#.in".to_string()),
                        output_file: Some("#.ans".to_string()),
                        answer_file: None,
                        subtasks,
                    };

                    let yaml = serde_yaml::to_string(&problem)?;
                    std::fs::write(config.output_dir.join("data.yml"), yaml)?;
                }
            }
        }

        Ok(())
    }
}
