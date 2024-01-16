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
    pub name: Option<String>,
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
    pub fn input_file(&self) -> Result<String> {
        Ok(format!(
            "{}.in",
            self.name.as_ref().ok_or_else(|| {
                anyhow::anyhow!("test case name not found for `{}`", self.generator_name)
            })?
        ))
    }

    pub fn answer_file(&self) -> Result<String> {
        Ok(format!(
            "{}.ans",
            self.name.as_ref().ok_or_else(|| {
                anyhow::anyhow!("test case name not found for `{}`", self.generator_name)
            })?
        ))
    }

    pub fn generate<T>(
        &self,
        output_dir: &std::path::PathBuf,
        programs: &T,
        solution_name: &str,
        validator_name: Option<&str>,
    ) -> Result<()>
    where
        T: GetProgram,
    {
        let input_path = output_dir.join(self.input_file()?);
        if input_path.exists() {
            std::fs::remove_file(&input_path)?;
        }
        let input = std::fs::File::create(&input_path)?;
        let generator = programs.get_program(&self.generator_name)?;
        generator
            .execute(&self.generator_name, self.args.clone(), None, Some(input))
            .with_context(|| format!("failed to generate `{}`", self.name.as_ref().unwrap()))?;

        let input = std::fs::File::open(&input_path)?;
        let answer_path = output_dir.join(self.answer_file()?);
        let answer = std::fs::File::create(&answer_path)?;
        let solution = programs.get_program(&solution_name)?;
        solution
            .execute(&solution_name, vec![], Some(input), Some(answer))
            .with_context(|| {
                format!(
                    "failed to generate answer for `{}`",
                    self.name.as_ref().unwrap()
                )
            })?;

        if let Some(validator_name) = validator_name {
            let validator = programs.get_program(validator_name)?;
            let input = std::fs::File::open(&input_path)?;
            validator
                .execute(&validator_name, vec![], Some(input), None)
                .with_context(|| format!("failed to validate `{}`", self.name.as_ref().unwrap()))?;
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TestBundle {
    pub cases: Vec<TestCase>,
}

impl TestBundle {
    pub fn generate<T>(
        &self,
        output_dir: &std::path::PathBuf,
        programs: &T,
        solution_name: &str,
        validator_name: Option<&str>,
    ) -> Result<()>
    where
        T: GetProgram,
    {
        self.cases
            .iter()
            .map(|case| case.generate(output_dir, programs, solution_name, validator_name))
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
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Test {
    pub bundles: HashMap<String, TestBundle>,
    pub tasks: HashMap<String, TestTask>,
}
