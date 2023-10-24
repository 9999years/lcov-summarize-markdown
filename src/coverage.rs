use std::collections::BTreeMap;

use camino::Utf8PathBuf;
use lcov::Record;
use miette::miette;
use miette::IntoDiagnostic;

#[derive(Debug)]
pub struct Coverage {
    state: State,
    pub files: BTreeMap<Utf8PathBuf, LineData>,
}

#[derive(Debug)]
enum State {
    None,
    SourceFile { path: Utf8PathBuf, data: LineData },
}

impl Coverage {
    pub fn new() -> Self {
        Self {
            state: State::None,
            files: Default::default(),
        }
    }

    pub fn total_coverage(&self) -> f64 {
        let mut hit = 0;
        let mut found = 0;

        for data in self.files.values() {
            if let (Some(file_hit), Some(file_found)) = (data.hit, data.found) {
                hit += file_hit;
                found += file_found;
            }
        }

        (hit as f64) / (found as f64)
    }

    pub fn consume(&mut self, record: Record) -> miette::Result<()> {
        match &mut self.state {
            State::None => match record {
                Record::SourceFile { path } => {
                    self.state = State::SourceFile {
                        path: path.try_into().into_diagnostic()?,
                        data: LineData::default(),
                    }
                }
                _ => {
                    return Err(miette!("Unexpected record: {record:?}"));
                }
            },
            State::SourceFile { path, data } => match record {
                Record::LineData {
                    line,
                    count,
                    checksum: _,
                } => {
                    *data.counts.entry(line).or_default() += count;
                }
                Record::LinesFound { found } => {
                    data.found = Some(found);
                }
                Record::LinesHit { hit } => {
                    data.hit = Some(hit);
                }
                Record::EndOfRecord => {
                    let data = std::mem::take(data);
                    self.files.insert(path.clone(), data);
                    self.state = State::None;
                }
                _ => {
                    return Err(miette!("Unexpected record: {record:?}"));
                }
            },
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Default)]
pub struct LineData {
    hit: Option<u32>,
    found: Option<u32>,
    /// Map from line numbers to counts.
    counts: BTreeMap<u32, u64>,
}

impl LineData {
    pub fn coverage_fraction(&self) -> Option<f64> {
        match (self.hit, self.found) {
            (Some(hit), Some(found)) => Some((hit as f64) / (found as f64)),
            _ => None,
        }
    }
}
