use crate::program::Program;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TestCase {
    pub args: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TestBundle {
    // TODO: generator or raw input
    pub name: String,
    pub generator: Program,
    pub test_cases: Vec<TestCase>,
}

impl TestBundle {
    pub fn generate(&self, inputs: Vec<std::fs::File>) -> Result<(), Box<dyn std::error::Error>> {
        self.test_cases
            .iter()
            .zip(inputs.into_iter())
            .for_each(|(case, input)| {
                self.generator.run(case.args.clone(), None, input).unwrap();
            });
        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Problem {
    pub name: String,
    pub test_bundles: Vec<TestBundle>,
    pub solution: Program,
}

impl Problem {
    pub fn generate(&self, path: std::path::PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        for test_bundle in &self.test_bundles {
            let names = (0..test_bundle.test_cases.len())
                .map(|i| format!("{}-{:02}", &test_bundle.name, i))
                .collect::<Vec<_>>();

            let inputs: Vec<_> = names
                .iter()
                .map(|name| path.join(format!("{}.in", &name)))
                .map(std::fs::File::create)
                .collect::<Result<_, _>>()?;
            test_bundle.generate(inputs)?;

            let inputs: Vec<_> = names
                .iter()
                .map(|name| path.join(format!("{}.in", &name)))
                .map(std::fs::File::open)
                .collect::<Result<_, _>>()?;
            let answers: Vec<_> = names
                .iter()
                .map(|name| path.join(format!("{}.ans", &name)))
                .map(std::fs::File::create)
                .collect::<Result<_, _>>()?;
            inputs
                .into_iter()
                .zip(answers.into_iter())
                .for_each(|(input, answer)| {
                    self.solution.run(vec![], Some(input), answer).unwrap();
                });
        }
        Ok(())
    }
}
