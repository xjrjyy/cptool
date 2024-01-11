pub mod test;

use crate::error::{Error, Result};
use crate::program::Program;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use test::{GetProgram, GetTestBundle, Test, TestBundle};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Problem {
    pub name: String,
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
    pub get_case_name: Box<dyn Fn(&str, usize) -> String>,
}

impl Problem {
    pub fn generate(&self, config: &GenerateConfig) -> Result<()> {
        let temp_dir = crate::utils::temp_dir();
        if temp_dir.exists() {
            std::fs::remove_dir_all(&temp_dir)?;
        }
        std::fs::create_dir_all(&temp_dir)?;
        if config.output_dir.exists() {
            std::fs::remove_dir_all(&config.output_dir)?;
        }

        for (name, test_bundle) in &self.test.bundles {
            let output_dir = if config.subdir {
                config.output_dir.join(&name)
            } else {
                config.output_dir.clone()
            };
            std::fs::create_dir_all(&output_dir)?;

            let cases: Vec<_> = (0..test_bundle.cases.len())
                .map(|index| (*config.get_case_name)(name, index))
                .collect();

            let inputs: Vec<_> = cases
                .iter()
                .map(|name| output_dir.join(format!("{}.in", &name)))
                .map(std::fs::File::create)
                .collect::<std::io::Result<_>>()?;
            test_bundle.generate(self, inputs)?;

            let inputs: Vec<_> = cases
                .iter()
                .map(|name| output_dir.join(format!("{}.in", &name)))
                .map(std::fs::File::open)
                .collect::<std::io::Result<_>>()?;
            let answers: Vec<_> = cases
                .iter()
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

        Ok(())
    }
}
