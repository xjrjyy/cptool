use crate::utils::random_string;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum ProgramLauguage {
    #[serde(rename = "cpp")]
    Cpp,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Program {
    pub language: ProgramLauguage,
    pub path: String,
}

impl Program {
    pub fn run(
        &self,
        args: Vec<String>,
        input: Option<std::fs::File>,
        output: std::fs::File,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match self.language {
            ProgramLauguage::Cpp => {
                let exe_path = std::env::temp_dir().join(random_string());

                // TODO: Compile Error
                std::process::Command::new("g++")
                    .arg("-o")
                    .arg(exe_path.clone())
                    .arg("-O2")
                    .arg("-std=c++17")
                    .arg(&self.path)
                    .output()?;

                // TODO: Runtime Error / Time Limit Exceeded / Memory Limit Exceeded
                let mut command = std::process::Command::new(exe_path.clone());
                if let Some(input) = input {
                    command.stdin(input);
                }
                command.args(args).stdout(output).output()?;
                std::fs::remove_file(exe_path)?;

                Ok(())
            }
        }
    }
}
