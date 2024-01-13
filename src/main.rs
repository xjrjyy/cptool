use clap::Parser;
use cptool::export::{syzoj, Exporter, OnlineJudge};
use cptool::problem::{GenerateConfig, Problem};
use std::time::Instant;

#[derive(Debug, Parser)]
struct Args {
    #[arg(short, long, default_value = ".")]
    work_dir: std::path::PathBuf,

    #[arg(short, long, default_value = "./data")]
    output_dir: std::path::PathBuf,

    #[arg(long, default_value = "false")]
    subdir: bool,

    #[arg(short, long, value_enum)]
    export_oj: Option<OnlineJudge>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let start = Instant::now();

    std::env::set_current_dir(&args.work_dir)?;

    let problem_yaml = std::fs::read_to_string("problem.yaml")?;
    let problem: Problem = serde_yaml::from_str(&problem_yaml)?;

    let get_case_name = if args.subdir {
        |_: &str, index: usize| format!("{}", index)
    } else {
        |name: &str, index: usize| format!("{}-{}", name, index)
    };
    let config = GenerateConfig {
        output_dir: args.output_dir.clone(),
        subdir: args.subdir,
        get_case_name: Box::new(get_case_name),
    };
    problem.generate(&config)?;

    match args.export_oj {
        Some(OnlineJudge::Syzoj) => {
            syzoj::SyzojExporter::export(&problem, &config)?;
        }
        None => {}
    }

    let elapsed = start.elapsed();
    println!(
        "elapsed: {}.{:03}s",
        elapsed.as_secs(),
        elapsed.subsec_millis()
    );

    Ok(())
}
