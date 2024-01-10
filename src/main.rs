use clap::Parser;
use cptool::problem::Problem;
use std::time::Instant;

#[derive(Debug, Parser)]
struct Args {
    #[arg(short, long)]
    work_dir: String,

    #[arg(short, long, default_value = "data")]
    output_dir: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let start = Instant::now();

    let work_dir = std::path::PathBuf::from(args.work_dir);
    std::env::set_current_dir(&work_dir)?;

    let problem_yaml = std::fs::read_to_string("problem.yaml")?;
    let problem: Problem = serde_yaml::from_str(&problem_yaml)?;

    let output_dir = std::path::PathBuf::from(args.output_dir);
    std::fs::create_dir_all(&output_dir)?;
    problem.generate(output_dir)?;

    let elapsed = start.elapsed();
    println!(
        "elapsed: {}.{:03}s",
        elapsed.as_secs(),
        elapsed.subsec_millis()
    );

    Ok(())
}
