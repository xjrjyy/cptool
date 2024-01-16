use crate::program::Execute;
use anyhow::Result;
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

    pub fn generate<T>(&self, output_dir: &std::path::PathBuf, programs: &T) -> Result<()>
    where
        T: GetProgram,
    {
        let input_path = output_dir.join(self.input_file()?);
        if input_path.exists() {
            std::fs::remove_file(&input_path)?;
        }
        let input = std::fs::File::create(input_path)?;
        let generator = programs.get_program(&self.generator_name)?;
        generator.execute(&self.generator_name, self.args.clone(), None, Some(input))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TestBundle {
    pub cases: Vec<TestCase>,
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
