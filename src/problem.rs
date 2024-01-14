mod builtin_problem;
pub mod test;

use crate::program::{Execute, Program};
use anyhow::{Context, Result};
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
    fn get_program(&self, name: &str) -> Result<&dyn Execute> {
        if name.starts_with("$") {
            use builtin_problem::FromFile;
            match name {
                "$file" => Ok(&FromFile as &dyn Execute),
                _ => Err(anyhow::anyhow!("builtin program `{}` not found", name)),
            }
        } else {
            self.programs
                .get(name)
                .map(|program| program as &dyn Execute)
                .ok_or(anyhow::anyhow!("program `{}` not found", name))
        }
    }
}

impl GetTestBundle for Problem {
    fn get_test_bundle(&self, name: &str) -> Result<&TestBundle> {
        self.test
            .bundles
            .get(name)
            .ok_or(anyhow::anyhow!("test bundle `{}` not found", name))
    }
}

pub struct GenerateConfig {
    pub output_dir: std::path::PathBuf,
    pub subdir: bool,
    pub get_case_name: Box<dyn Fn(&str, usize) -> String>,
}

impl Problem {
    fn generate_bundle(
        &self,
        config: &GenerateConfig,
        bundle_name: &str,
        bundle: &TestBundle,
    ) -> Result<()> {
        let output_dir = if config.subdir {
            config.output_dir.join(&bundle_name)
        } else {
            config.output_dir.clone()
        };
        std::fs::create_dir_all(&output_dir)?;

        let solution = self.get_program(&self.solution_name)?;
        bundle
            .cases
            .iter()
            .enumerate()
            .map(|(index, case)| {
                let name = (*config.get_case_name)(bundle_name, index);

                let input_path = output_dir.join(format!("{}.in", &name));
                let input = std::fs::File::create(&input_path)?;
                case.generate(self, input)?;

                let input = std::fs::File::open(&input_path)?;
                let answer_path = output_dir.join(format!("{}.ans", &name));
                let answer = std::fs::File::create(&answer_path)?;
                solution.execute(&self.solution_name, vec![], Some(input), answer)?;

                Ok(())
            })
            .collect::<Result<_>>()?;

        Ok(())
    }

    pub fn generate(&self, config: &GenerateConfig) -> Result<()> {
        let temp_dir = crate::utils::temp_dir();
        if temp_dir.exists() {
            std::fs::remove_dir_all(&temp_dir)?;
        }
        std::fs::create_dir_all(&temp_dir)?;
        if config.output_dir.exists() {
            std::fs::remove_dir_all(&config.output_dir)?;
        }

        for (bundle_name, bundle) in &self.test.bundles {
            self.generate_bundle(config, bundle_name, bundle)
                .with_context(|| format!("failed to generate test bundle `{}`", bundle_name))?;
        }

        let mut used_bundles = std::collections::HashSet::new();
        for (_, task) in &self.test.tasks {
            for bundle_name in task.bundles.iter() {
                self.get_test_bundle(bundle_name)?;
                used_bundles.insert(bundle_name);
            }
        }
        for (bundle_name, _) in &self.test.bundles {
            if !used_bundles.contains(bundle_name) {
                println!("warning: unused test bundle `{}`", bundle_name);
            }
        }

        Ok(())
    }
}
