use lcov::Reader;
use miette::IntoDiagnostic;

mod coverage;
use coverage::Coverage;

fn main() -> miette::Result<()> {
    let mut coverage = Coverage::new();
    let reader = Reader::open_file("../broot.nvim/target/coverage.lcov").into_diagnostic()?;
    for item in reader {
        let item = item.into_diagnostic()?;
        coverage.consume(item)?;
    }

    for (file, data) in coverage.files.iter() {
        println!("{file}: {:?}", data.coverage_fraction());
    }

    Ok(())
}
