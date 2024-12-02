use core::{fmt::Write, str};

use alloc::vec::Vec;
use defmt::info;
use picoserve::io::Read;

use crate::helpers::read_input;

/// Helper trait that creates an iterator of sequential pairs, but where the Nth item of the input 
/// slice is skipped.
trait SkipNZip {
    type Item;
    fn skip_n_zip(self, skip: usize) -> impl Iterator<Item = Self::Item>;
}

impl<'a, T: Sized> SkipNZip for &'a [T] {
    type Item = (&'a T, &'a T);

    fn skip_n_zip(self, skip: usize) -> impl Iterator<Item = Self::Item> {
        // there's probably a better way to do this
        let a = self
            .iter()
            .enumerate()
            .filter(move |(n, _)| *n != skip)
            .map(|(_, n)| n);
        let b = self
            .iter()
            .enumerate()
            .filter(move |(n, _)| *n != skip)
            .map(|(_, n)| n)
            .skip(1);
        a.zip(b)
    }
}

/// For a given pair of reports, what is their class?
#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum ReportClass {
    /// Unsafe - `delta := {0, >3, <-3}`
    Unsafe,
    /// Safe ascending - `delta := {1..=3}`
    SafeAsc,
    /// Safe descending - `delta := {-3..=-1}`
    SafeDesc,
}

/// Classify pairs of numbers into their class, presented as `ReportClass`
fn classify_pair((l, r): (&i16, &i16)) -> ReportClass {
    match l - r {
        1..=3 => ReportClass::SafeAsc,
        -3..=-1 => ReportClass::SafeDesc,
        _ => ReportClass::Unsafe,
    }
}

/// Function designed for folding, retains counters of all of the p1-safe and p2-safe report strings
/// for a single input
/// We parse the input inside the loop because we don't have enough memory to parse the whole
/// input into a `Vec<Vec<i16>>` all at once.
fn fold_safe_reports((p1, p2): (u32, u32), report: &str) -> (u32, u32) {
    let reports: Vec<i16> = if let Ok(v) = report
        .split(' ')
        .map(str::parse)
        .collect::<Result<Vec<i16>, _>>()
    {
        v
    } else {
        return (p1, p2);
    };

    if reports
        .iter()
        .zip(reports.iter().skip(1))
        .map(classify_pair)
        .reduce(|a, b| if a == b { a } else { ReportClass::Unsafe })
        .is_some_and(|v| v != ReportClass::Unsafe)
    {
        return (p1 + 1, p2 + 1);
    }

    for i in 0..reports.len() {
        if reports
            .skip_n_zip(i)
            .map(classify_pair)
            .reduce(|a, b| if a == b { a } else { ReportClass::Unsafe })
            .is_some_and(|v| v != ReportClass::Unsafe)
        {
            return (p1, p2 + 1);
        }
    }
    (p1, p2)
}

pub async fn solve<R: Read, W: Write>(r: &mut R, w: &mut W, input_len: usize) -> crate::Result<()> {
    info!("Solving for input of size {}", input_len);
    let input = read_input(r, input_len).await?;
    info!("Input read");
    let (p1_safe, p2_safe) = input.lines().fold((0, 0), fold_safe_reports);

    writeln!(w, "Part 1: {p1_safe} Reports are safe<br>")?;
    writeln!(w, "Part 2: {p2_safe} reports are safe<br>")?;

    Ok(())
}
