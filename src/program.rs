use crate::error::{Error, Result};
use crate::utils::temp_dir;
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
    ) -> Result<()> {
        match self.language {
            ProgramLauguage::Cpp => {
                let exe_name = std::path::PathBuf::from(&self.path)
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string();
                let exe_path = temp_dir().join(exe_name);

                if !exe_path.exists() {
                    let output = std::process::Command::new("g++")
                        .arg("-o")
                        .arg(exe_path.clone())
                        .arg("-O2")
                        .arg("-std=c++17")
                        .arg(&self.path)
                        .output()?;
                    if !output.status.success() {
                        return Err(Error::Compile(String::from_utf8(output.stderr).unwrap()));
                    }
                }

                // TODO: Runtime Error / Time Limit Exceeded / Memory Limit Exceeded
                let mut command = std::process::Command::new(exe_path.clone());
                if let Some(input) = input {
                    command.stdin(input);
                }
                command.args(args).stdout(output).output()?;

                Ok(())
            }
        }
    }
}
