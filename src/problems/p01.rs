use core::{fmt::Write, str};

use alloc::{vec, vec::Vec};
use picoserve::io::Read;

use crate::error::{AerError, IntoAer};

/// day 1: Historian's Location IDs
pub async fn solve<R: Read, W: Write>(r: &mut R, w: &mut W, input_len: usize) -> crate::Result<()> {
    let mut input_buf = vec![0u8; input_len];
    let mut read_count: usize = 0;

    loop {
        let read_size = r.read(&mut input_buf[read_count..]).await.into_aer()?;
        if read_size == 0 {
            break;
        }
        read_count += read_size;
        if read_count == input_len {
            break;
        }
    }
    let content = str::from_utf8(input_buf.strip_prefix(b"message=").ok_or(AerError::MissingMessage)?)?;
    let line_count = content.lines().count();
    let mut left_numbers: Vec<i32> = Vec::with_capacity(line_count);
    let mut right_numbers: Vec<i32> = Vec::with_capacity(line_count);
    for line in content.lines() {
        if let Some((l, r)) = line.split_once("   ") {
            left_numbers.push(l.parse()?);
            right_numbers.push(r.parse()?);
        }
    }
    left_numbers.sort_unstable();
    right_numbers.sort_unstable();
    let answer: u32 = left_numbers.iter()
        .zip(right_numbers.iter())
        .map(|(l, r)| l.abs_diff(*r))
        .sum();
    writeln!(w, "Part 1 Answer: {answer}<br>")?;
    let mut p2_answer: usize = 0;
    for l in &left_numbers {
        p2_answer += *l as usize * right_numbers.iter()
            .take_while(|r| *r <= l) // The array is sorted and it's marginally faster to solve this way
            .filter(|r| *r == l)
            .count();
    }
    writeln!(w, "Part 2 answer: {p2_answer}<br>")?;
    Ok(())
}
