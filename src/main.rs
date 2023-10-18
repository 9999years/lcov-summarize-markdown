use lcov::Reader;
use miette::IntoDiagnostic;

fn main() -> miette::Result<()> {
    let reader = Reader::open_file("../broot.nvim/target/coverage.lcov").into_diagnostic()?;
    let records = reader.collect::<Result<Vec<_>, _>>().into_diagnostic()?;
    let mut maybe_hit = None;
    let mut maybe_found = None;
    for record in records {
        match record {
            lcov::Record::LinesFound { found } => {
                maybe_found = Some(found);
            }
            lcov::Record::LinesHit { hit } => {
                maybe_hit = Some(hit);
            }
            lcov::Record::EndOfRecord => {
                if let Some(hit) = maybe_hit {
                    if let Some(found) = maybe_found {
                        println!("Coverage: {}", (hit as f64) / (found as f64));
                    }
                }
                maybe_hit = None;
                maybe_found = None;
            }
            _ => {}
        }
        println!("{:?}", record);
    }

    Ok(())
}
