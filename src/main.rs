use std::path::PathBuf;

use clap::Parser;
use lcov::Reader;
use miette::Context;
use miette::IntoDiagnostic;

mod coverage;
use coverage::Coverage;

#[derive(Parser)]
struct Opts {
    /// Coverage file to read.
    file: PathBuf,
}

fn main() -> miette::Result<()> {
    let opts = Opts::parse();

    let mut coverage = Coverage::new();
    let reader = Reader::open_file(&opts.file)
        .into_diagnostic()
        .wrap_err_with(|| format!("Failed to open file {:?}", opts.file))?;
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
