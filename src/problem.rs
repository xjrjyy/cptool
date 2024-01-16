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
    #[serde(rename = "validator")]
    pub validator_name: Option<String>,
    #[serde(rename = "solution")]
    pub solution_name: String,
    #[serde(rename = "checker")]
    pub checker_name: String,
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

impl Problem {
    pub fn generate(&self, output_dir: &std::path::PathBuf) -> Result<()> {
        let temp_dir = crate::utils::temp_dir();
        if temp_dir.exists() {
            std::fs::remove_dir_all(&temp_dir)?;
        }
        std::fs::create_dir_all(&temp_dir)?;

        self.test
            .bundles
            .iter()
            .map(|(bundle_name, bundle)| {
                bundle
                    .generate(
                        output_dir,
                        self,
                        &self.solution_name,
                        self.validator_name.as_deref(),
                    )
                    .with_context(|| format!("failed to generate test bundle `{}`", bundle_name))?;

                bundle
                    .cases
                    .iter()
                    .map(|case| {
                        let answer_path = output_dir.join(&case.answer_file()?);
                        case.check(output_dir, self, &self.checker_name, &answer_path)
                    })
                    .collect::<Result<_>>()?;

                Ok(())
            })
            .collect::<Result<_>>()?;

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

        println!(
            "total score: {:.2}",
            self.test.tasks.values().map(|task| task.score).sum::<f64>()
        );

        Ok(())
    }
}
