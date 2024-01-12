use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CppProgram {
    pub path: String,
    #[serde(default = "default_compile_args")]
    pub compile_args: Vec<String>,
}

fn default_compile_args() -> Vec<String> {
    vec!["-O2".to_string()]
}

impl std::fmt::Display for CppProgram {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "C++ {} (args: {})",
            self.path,
            self.compile_args.join(" ")
        )
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ProgramInfo {
    #[serde(rename = "cpp")]
    Cpp(CppProgram),
}

impl std::fmt::Display for ProgramInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProgramInfo::Cpp(program) => write!(f, "{}", program),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Program {
    info: ProgramInfo,
    time_limit_secs: f64,
    memory_limit_mb: f64,
}

impl Program {
    pub fn run(
        &self,
        name: &str,
        args: Vec<String>,
        input: Option<std::fs::File>,
        output: std::fs::File,
    ) -> Result<()> {
        match &self.info {
            ProgramInfo::Cpp(CppProgram { path, compile_args }) => {
                let exe_name = if cfg!(windows) {
                    format!("{}.exe", name)
                } else {
                    name.to_string()
                };
                let exe_path = crate::utils::temp_dir().join(exe_name);

                if !exe_path.exists() {
                    let output = std::process::Command::new("g++")
                        .arg("-o")
                        .arg(exe_path.clone())
                        .args(compile_args)
                        .arg(path)
                        .output()?;
                    if !output.status.success() {
                        return Err(Error::compile_error(
                            &self.info,
                            String::from_utf8(output.stderr).unwrap(),
                        ));
                    }
                }

                let mut command = std::process::Command::new(exe_path.clone());
                if let Some(input) = input {
                    command.stdin(input);
                }
                command.args(args).stdout(output);

                use process_control::{ChildExt, Control};
                let child = command.spawn()?;
                let output = child
                    .controlled_with_output()
                    .time_limit(std::time::Duration::from_secs_f64(self.time_limit_secs))
                    .memory_limit((self.memory_limit_mb * 1024.0 * 1024.0) as usize)
                    .terminate_for_timeout()
                    .wait()?
                    .ok_or_else(|| Error::time_limit_exceeded(&self.info))?;
                if !output.status.success() {
                    return Err(Error::runtime_error(&self.info));
                }

                Ok(())
            }
        }
    }
}
