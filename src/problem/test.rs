use crate::error::Result;
use crate::program::Program;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub trait GetProgram {
    fn get_program(&self, name: &str) -> Result<&Program>;
}

pub trait GetTestBundle {
    fn get_test_bundle(&self, name: &str) -> Result<&TestBundle>;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TestCase {
    // TODO: raw input
    #[serde(rename = "generator")]
    pub generator_name: String,
    pub args: Vec<String>,
}

impl TestCase {
    pub fn generate<T>(&self, programs: &T, input: std::fs::File) -> Result<()>
    where
        T: GetProgram,
    {
        let generator = programs.get_program(&self.generator_name)?;
        generator.run(self.args.clone(), None, input)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TestBundle {
    pub cases: Vec<TestCase>,
}

impl TestBundle {
    pub fn generate<T>(&self, programs: &T, inputs: Vec<std::fs::File>) -> Result<()>
    where
        T: GetProgram,
    {
        self.cases
            .iter()
            .zip(inputs.into_iter())
            .map(|(case, input)| case.generate(programs, input))
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
