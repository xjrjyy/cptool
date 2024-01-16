use clap::Parser;
use cptool::export::{syzoj, Exporter, OnlineJudge};
use cptool::problem::Problem;
use std::time::Instant;

#[derive(Debug, Parser)]
#[command(version)]
struct Args {
    #[arg(short, long, default_value = ".")]
    work_dir: std::path::PathBuf,

    #[arg(short, long, default_value = "./data")]
    output_dir: std::path::PathBuf,

    #[arg(short, long, value_enum)]
    export_oj: Option<OnlineJudge>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let start = Instant::now();

    std::env::set_current_dir(&args.work_dir)?;

    let problem_yaml = std::fs::read_to_string("problem.yaml")?;
    let mut problem: Problem = serde_yaml::from_str(&problem_yaml)?;

    if args.output_dir.exists() {
        std::fs::remove_dir_all(&args.output_dir)?;
    }
    std::fs::create_dir_all(&args.output_dir)?;

    problem
        .test
        .bundles
        .iter_mut()
        .for_each(|(bundle_name, bundle)| {
            bundle
                .cases
                .iter_mut()
                .enumerate()
                .for_each(|(index, case)| {
                    case.name = Some(format!("{}-{}", bundle_name, index));
                });
        });
    problem.generate(&args.output_dir)?;

    match args.export_oj {
        Some(OnlineJudge::Syzoj) => {
            syzoj::SyzojExporter::export(&problem, &args.output_dir)?;
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
