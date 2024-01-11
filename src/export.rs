pub mod syzoj;

use clap::ValueEnum;

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum OnlineJudge {
    Syzoj,
}

pub trait Exporter {
    fn export(
        problem: &crate::problem::Problem,
        config: &crate::problem::GenerateConfig,
    ) -> crate::error::Result<()>;
}
