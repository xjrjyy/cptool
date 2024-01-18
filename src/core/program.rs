use anyhow::Result;

#[derive(Clone, Debug)]
pub struct CommandProgram {
    pub path: std::path::PathBuf,
    pub extra_args: Vec<String>,
}

#[derive(Clone, Debug)]
pub enum ProgramInfo {
    Command(CommandProgram),
}

#[derive(Clone, Debug)]
pub struct Program {
    pub info: ProgramInfo,
    pub time_limit_secs: f64,
    pub memory_limit_mb: f64,
}

impl Program {
    fn execute_command(&self, command: &mut std::process::Command) -> Result<()> {
        use process_control::{ChildExt, Control};
        let child = command.spawn()?;
        // TODO: error context
        let output = child
            .controlled_with_output()
            .time_limit(std::time::Duration::from_secs_f64(self.time_limit_secs))
            .memory_limit((self.memory_limit_mb * 1024.0 * 1024.0) as usize)
            .terminate_for_timeout()
            .wait()?
            .unwrap();
        if !output.status.success() {
            // TODO: error context
            return Err(anyhow::anyhow!(
                "program exited with non-zero status code: {}",
                output.status
            ));
        }
        Ok(())
    }

    pub fn execute(
        &self,
        args: Vec<String>,
        input: Option<std::fs::File>,
        output: Option<std::fs::File>,
    ) -> Result<()> {
        // TODO: error context
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
        }
    }
}
