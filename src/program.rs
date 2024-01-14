use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

pub trait Execute {
    fn execute(
        &self,
        name: &str,
        args: Vec<String>,
        input: Option<std::fs::File>,
        output: Option<std::fs::File>,
    ) -> Result<()>;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CommandProgram {
    pub path: String,
    #[serde(default)]
    pub extra_args: Vec<String>,
}

impl std::fmt::Display for CommandProgram {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} (extra args: `{}`)",
            self.path,
            self.extra_args.join(" ")
        )
    }
}

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
            "{} (compile args: `{}`)",
            self.path,
            self.compile_args.join(" ")
        )
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ProgramInfo {
    #[serde(rename = "command")]
    Command(CommandProgram),
    #[serde(rename = "cpp")]
    Cpp(CppProgram),
}

impl std::fmt::Display for ProgramInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProgramInfo::Command(program) => write!(f, "Command {}", program),
            ProgramInfo::Cpp(program) => write!(f, "Cpp {}", program),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Program {
    info: ProgramInfo,
    time_limit_secs: f64,
    memory_limit_mb: f64,
}

impl std::fmt::Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} (time limit: {}s, memory limit: {}mb)",
            self.info, self.time_limit_secs, self.memory_limit_mb
        )
    }
}

impl Program {
    fn execute_command(&self, command: &mut std::process::Command) -> Result<()> {
        use process_control::{ChildExt, Control};
        let child = command.spawn()?;
        let output = child
            .controlled_with_output()
            .time_limit(std::time::Duration::from_secs_f64(self.time_limit_secs))
            .memory_limit((self.memory_limit_mb * 1024.0 * 1024.0) as usize)
            .terminate_for_timeout()
            .wait()?
            .ok_or_else(|| anyhow::anyhow!("time limit exceeded: {}", &self))?;
        if !output.status.success() {
            return Err(anyhow::anyhow!("runtime error: {}", &self));
        }
        Ok(())
    }
}

impl Execute for Program {
    fn execute(
        &self,
        name: &str,
        args: Vec<String>,
        input: Option<std::fs::File>,
        output: Option<std::fs::File>,
    ) -> Result<()> {
        match &self.info {
            ProgramInfo::Command(CommandProgram { path, extra_args }) => {
                let mut command = std::process::Command::new(path);
                if let Some(input) = input {
                    command.stdin(input);
                }
                if let Some(output) = output {
                    command.stdout(output);
                }
                command.args(extra_args).args(args.clone());

                self.execute_command(&mut command)
            }
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
                        return Err(anyhow::anyhow!(
                            "compile error: {}\n{}",
                            &self.info,
                            String::from_utf8_lossy(&output.stderr)
                        ));
                    }
                }

                let mut command = std::process::Command::new(exe_path.clone());
                if let Some(input) = input {
                    command.stdin(input);
                }
                if let Some(output) = output {
                    command.stdout(output);
                }
                command.args(args.clone());

                self.execute_command(&mut command)
            }
        }
        .with_context(|| {
            format!(
                "failed to run program `{}` (args: `{}`)",
                name,
                args.join(" ")
            )
        })
    }
}
