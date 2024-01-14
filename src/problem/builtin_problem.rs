use crate::program::Execute;
use anyhow::{Context, Result};

pub(super) struct FromFile;

impl Execute for FromFile {
    fn execute(
        &self,
        name: &str,
        args: Vec<String>,
        _input: Option<std::fs::File>,
        output: std::fs::File,
    ) -> Result<()> {
        let input_path = args
            .get(0)
            .map(std::path::PathBuf::from)
            .with_context(|| anyhow::anyhow!("`{}` requires one argument", name))?;
        let input = std::fs::File::open(&input_path)?;
        std::io::copy(
            &mut std::io::BufReader::new(input),
            &mut std::io::BufWriter::new(output),
        )?;
        Ok(())
    }
}
