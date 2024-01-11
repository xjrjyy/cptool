use crate::error::{Error, Result};
use crate::program::Program;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

type Programs = HashMap<String, Program>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TestCase {
    pub args: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TestBundle {
    // TODO: generator or raw input
    pub name: String,
    #[serde(rename = "generator")]
    pub generator_name: String,
    pub cases: Vec<TestCase>,
}

impl TestBundle {
    pub fn generate(&self, programs: &Programs, inputs: Vec<std::fs::File>) -> Result<()> {
        let generator = programs
            .get(&self.generator_name)
            .ok_or(Error::file_not_found(format!("{}", &self.generator_name)))?;
        self.cases
            .iter()
            .zip(inputs.into_iter())
            .map(|(case, input)| generator.run(case.args.clone(), None, input))
            .collect::<Result<_>>()?;
        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Problem {
    pub name: String,
    pub programs: Programs,
    pub test_bundles: Vec<TestBundle>,
    #[serde(rename = "solution")]
    pub solution_name: String,
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

        for test_bundle in &self.test_bundles {
            let output_dir = if config.subdir {
                config.output_dir.join(&test_bundle.name)
            } else {
                config.output_dir.clone()
            };
            std::fs::create_dir_all(&output_dir)?;

            let names = (0..test_bundle.cases.len())
                .map(|i| {
                    if config.subdir {
                        format!("{:02}", i)
                    } else {
                        format!("{}-{:02}", &test_bundle.name, i)
                    }
                })
                .collect::<Vec<_>>();

            let inputs: Vec<_> = names
                .iter()
                .map(|name| output_dir.join(format!("{}.in", &name)))
                .map(std::fs::File::create)
                .collect::<std::io::Result<_>>()?;
            test_bundle.generate(&self.programs, inputs)?;

            let inputs: Vec<_> = names
                .iter()
                .map(|name| output_dir.join(format!("{}.in", &name)))
                .map(std::fs::File::open)
                .collect::<std::io::Result<_>>()?;
            let answers: Vec<_> = names
                .iter()
                .map(|name| output_dir.join(format!("{}.ans", &name)))
                .map(std::fs::File::create)
                .collect::<std::io::Result<_>>()?;
            let solution = self
                .programs
                .get(&self.solution_name)
                .ok_or(Error::file_not_found(format!("{}", &self.solution_name)))?;
            inputs
                .into_iter()
                .zip(answers.into_iter())
                .map(|(input, answer)| solution.run(vec![], Some(input), answer))
                .collect::<Result<_>>()?;
        }
        Ok(())
    }
}
