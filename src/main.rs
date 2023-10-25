use clap::Parser;
use lcov::Reader;
use miette::Context;
use miette::IntoDiagnostic;

mod coverage;
use coverage::Coverage;

mod cli;
mod command;
mod git;

fn main() -> miette::Result<()> {
    let opts = cli::Opts::parse();

    let mut coverage = Coverage::new();
    let reader = Reader::open_file(&opts.coverage_file)
        .into_diagnostic()
        .wrap_err_with(|| format!("Failed to open file {:?}", opts.coverage_file))?;
    for item in reader {
        let item = item.into_diagnostic()?;
        coverage.consume(item)?;
    }

    for (file, data) in coverage.files.iter() {
        println!(
            "{file}: {:.2}",
            data.coverage_fraction().unwrap_or_default()
        );
    }
    println!("Total coverage: {:.2}", coverage.total_coverage());

    Ok(())
}
