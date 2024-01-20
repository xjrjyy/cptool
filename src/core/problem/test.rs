use super::super::program::Program;
use anyhow::Result;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct TestCase {
    pub args: Vec<String>,
    pub input_path: std::path::PathBuf,
    pub answer_path: std::path::PathBuf,
}

impl TestCase {
    // TODO: result struct
    pub fn check(&self, output_path: &std::path::Path, checker: &Program) -> Result<()> {
        // TODO: partial points
        // TODO: error context
        checker.execute(
            vec![
                self.input_path.to_str().unwrap().to_string(),
                output_path.to_str().unwrap().to_string(),
                self.answer_path.to_str().unwrap().to_string(),
            ],
            None,
            None,
        )
    }
}

#[derive(Clone, Debug)]
pub struct TestBundle {
    pub cases: Vec<TestCase>,
}

#[derive(Clone, Copy, Debug)]
pub enum TestTaskType {
    Sum,
    Min,
}

#[derive(Clone, Debug)]
pub struct TestTask {
    pub score: f64,
    pub task_type: TestTaskType,
    pub bundles: Vec<String>,
    pub dependencies: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct Test {
    pub bundles: HashMap<String, TestBundle>,
    pub tasks: HashMap<String, TestTask>,
}
