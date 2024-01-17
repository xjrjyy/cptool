use crate::program::Execute;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub trait GetProgram {
    fn get_program(&self, name: &str) -> Result<&dyn Execute>;
}

pub trait GetTestBundle {
    fn get_test_bundle(&self, name: &str) -> Result<&TestBundle>;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TestCase {
    #[serde(rename = "generator")]
    pub generator_name: String,
    pub args: Vec<String>,
    #[serde(skip)]
    pub input_path: Option<std::path::PathBuf>,
    #[serde(skip)]
    pub answer_path: Option<std::path::PathBuf>,
}

impl std::fmt::Display for TestCase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} (args: `{}`)",
            self.generator_name,
            self.args.join(" ")
        )
    }
}

impl TestCase {
    pub fn generate<T>(
        &self,
        programs: &T,
        solution_name: &str,
        validator_name: Option<&str>,
    ) -> Result<()>
    where
        T: GetProgram,
    {
        let input_path = self.input_path.as_ref().unwrap();
        if input_path.exists() {
            std::fs::remove_file(&input_path)?;
        }
        let input = std::fs::File::create(&input_path)?;
        let generator = programs.get_program(&self.generator_name)?;
        generator
            .execute(&self.generator_name, self.args.clone(), None, Some(input))
            .with_context(|| format!("failed to generate data for test case `{}`", self))?;

        let input = std::fs::File::open(&input_path)?;
        let answer_path = self.answer_path.as_ref().unwrap();
        let answer = std::fs::File::create(&answer_path)?;
        let solution = programs.get_program(&solution_name)?;
        solution
            .execute(&solution_name, vec![], Some(input), Some(answer))
            .with_context(|| format!("failed to generate answer for test case `{}`", self))?;

        if let Some(validator_name) = validator_name {
            let validator = programs.get_program(validator_name)?;
            let input = std::fs::File::open(&input_path)?;
            validator
                .execute(&validator_name, vec![], Some(input), None)
                .with_context(|| format!("failed to validate test case `{}`", self))?;
        }

        Ok(())
    }

    pub fn check<T>(
        &self,
        programs: &T,
        checker_name: &str,
        output_path: &std::path::PathBuf,
    ) -> Result<()>
    where
        T: GetProgram,
    {
        let input_path = self.input_path.as_ref().unwrap();
        let answer_path = self.answer_path.as_ref().unwrap();
        let checker = programs.get_program(checker_name)?;
        // TODO: partial points
        checker
            .execute(
                &checker_name,
                vec![
                    input_path.display().to_string(),
                    output_path.display().to_string(),
                    answer_path.display().to_string(),
                ],
                None,
                None,
            )
            .with_context(|| format!("failed to check test case `{}`", self))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TestBundle {
    pub cases: Vec<TestCase>,
}

impl TestBundle {
    pub fn generate<T>(
        &self,
        programs: &T,
        solution_name: &str,
        validator_name: Option<&str>,
    ) -> Result<()>
    where
        T: GetProgram,
    {
        self.cases
            .iter()
            .map(|case| case.generate(programs, solution_name, validator_name))
            .collect::<Result<_>>()
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum TestTaskType {
    #[serde(rename = "sum")]
    Sum,
    #[serde(rename = "min")]
    Min,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TestTask {
    pub score: f64,
    #[serde(rename = "type")]
    pub task_type: TestTaskType,
    pub bundles: Vec<String>,
    #[serde(default)]
    pub dependencies: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Test {
    pub bundles: HashMap<String, TestBundle>,
    pub tasks: HashMap<String, TestTask>,
}
