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
    pub args: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TestBundle {
    // TODO: generator or raw input
    #[serde(rename = "generator")]
    pub generator_name: String,
    pub cases: Vec<TestCase>,
}

impl TestBundle {
    pub fn generate<T>(&self, programs: &T, inputs: Vec<std::fs::File>) -> Result<()>
    where
        T: GetProgram,
    {
        let generator = programs.get_program(&self.generator_name)?;
        self.cases
            .iter()
            .zip(inputs.into_iter())
            .map(|(case, input)| generator.run(case.args.clone(), None, input))
            .collect::<Result<_>>()?;
        Ok(())
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
