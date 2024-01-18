use crate::core::{problem as core_problem, program as core_program};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TestCase {
    #[serde(rename = "generator")]
    pub generator_name: String,
    pub args: Vec<String>,
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
    pub fn generate(
        &self,
        programs: &HashMap<String, core_program::Program>,
        solution: &core_program::Program,
        validator: Option<&core_program::Program>,
        checker: Option<&core_program::Program>,
        input_path: &std::path::PathBuf,
        answer_path: &std::path::PathBuf,
    ) -> Result<core_problem::test::TestCase> {
        if input_path.exists() {
            std::fs::remove_file(&input_path)?;
        }
        let input = std::fs::File::create(&input_path)?;
        let generator = programs.get(&self.generator_name).with_context(|| {
            format!(
                "generator `{}` not found for test case `{}`",
                self.generator_name, self
            )
        })?;
        generator
            .execute(self.args.clone(), None, Some(input))
            .with_context(|| format!("failed to generate data for test case `{}`", self))?;

        let input = std::fs::File::open(&input_path)?;
        let answer = std::fs::File::create(&answer_path)?;
        solution
            .execute(vec![], Some(input), Some(answer))
            .with_context(|| format!("failed to generate answer for test case `{}`", self))?;

        if let Some(validator) = validator {
            let input = std::fs::File::open(&input_path)?;
            validator
                .execute(vec![], Some(input), None)
                .with_context(|| format!("failed to validate test case `{}`", self))?;
        }

        Ok(core_problem::test::TestCase {
            args: self.args.clone(),
            input_path: input_path.clone(),
            answer_path: answer_path.clone(),
            checker: checker.cloned(),
        })
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

impl Into<core_problem::test::TestTaskType> for TestTaskType {
    fn into(self) -> core_problem::test::TestTaskType {
        match self {
            TestTaskType::Sum => core_problem::test::TestTaskType::Sum,
            TestTaskType::Min => core_problem::test::TestTaskType::Min,
        }
    }
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

impl TestTask {
    pub fn generate(&self) -> Result<core_problem::test::TestTask> {
        Ok(core_problem::test::TestTask {
            score: self.score,
            task_type: self.task_type.into(),
            bundles: self.bundles.clone(),
            dependencies: self.dependencies.clone(),
        })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Test {
    pub bundles: HashMap<String, TestBundle>,
    pub tasks: HashMap<String, TestTask>,
}

impl Test {
    pub fn generate(
        &self,
        programs: &HashMap<String, core_program::Program>,
        solution: &core_program::Program,
        validator: Option<&core_program::Program>,
        checker: Option<&core_program::Program>,
        output_dir: &std::path::PathBuf,
    ) -> Result<core_problem::test::Test> {
        let mut bundles = HashMap::new();
        for (bundle_name, bundle) in &self.bundles {
            let mut cases = vec![];
            for (index, case) in bundle.cases.iter().enumerate() {
                let case_name = format!("{}-{}", bundle_name, index);
                let input_path = output_dir.join(format!("{}.in", case_name));
                let answer_path = output_dir.join(format!("{}.ans", case_name));
                cases.push(case.generate(
                    programs,
                    solution,
                    validator,
                    checker,
                    &input_path,
                    &answer_path,
                )?);
            }
            bundles.insert(
                bundle_name.clone(),
                core_problem::test::TestBundle { cases },
            );
        }

        let mut tasks = HashMap::new();
        for (task_name, task) in &self.tasks {
            tasks.insert(task_name.clone(), task.generate()?);
        }

        Ok(core_problem::test::Test { bundles, tasks })
    }
}
