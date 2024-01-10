use crate::error::{Error, Result};
use crate::utils::temp_dir;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CppProgram {
    pub path: String,
    #[serde(default = "default_compile_args")]
    pub compile_args: Vec<String>,
}

fn default_compile_args() -> Vec<String> {
    vec!["-O2".to_string(), "-std=c++17".to_string()]
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Program {
    #[serde(rename = "cpp")]
    Cpp(CppProgram),
}

impl Program {
    pub fn run(
        &self,
        args: Vec<String>,
        input: Option<std::fs::File>,
        output: std::fs::File,
    ) -> Result<()> {
        match self {
            Self::Cpp(CppProgram { path, compile_args }) => {
                let exe_name = std::path::PathBuf::from(path)
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
                        .args(compile_args)
                        .arg(path)
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
