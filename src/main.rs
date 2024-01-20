use clap::Parser;
use cptool::config::problem as config_problem;
use cptool::export::{syzoj, Exporter, OnlineJudge};
use std::time::Instant;

#[derive(Debug, Parser)]
#[command(version)]
struct Args {
    #[arg(short, long, default_value = ".")]
    work_dir: std::path::PathBuf,

    #[arg(short, long, default_value = "./output")]
    output_dir: std::path::PathBuf,

    #[arg(short, long, value_enum)]
    export_oj: Option<OnlineJudge>,

    #[arg(long, default_value = "./export")]
    export_dir: Option<std::path::PathBuf>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let start = Instant::now();

    std::env::set_current_dir(&args.work_dir)?;

    let problem_yaml = std::fs::read_to_string("problem.yaml")?;
    let problem_config: config_problem::Problem = serde_yaml::from_str(&problem_yaml)?;

    let problem = problem_config.generate(&args.output_dir)?;

    if let Some(OnlineJudge::Syzoj) = args.export_oj {
        let export_dir = args
            .export_dir
            .expect("export path not specified")
            .join("syzoj");
        if export_dir.exists() {
            std::fs::remove_dir_all(&export_dir)?;
        }
        std::fs::create_dir_all(&export_dir)?;

        syzoj::SyzojExporter::export(&problem, &export_dir)?;
    }

    let elapsed = start.elapsed();
    println!(
        "elapsed: {}.{:03}s",
        elapsed.as_secs(),
        elapsed.subsec_millis()
    );

    Ok(())
}
