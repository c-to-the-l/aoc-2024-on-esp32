use core::fmt::Write;

use picoserve::io::Read;

use crate::helpers::read_input;

pub async fn solve<R: Read, W: Write>(r: &mut R, w: &mut W, input_len: usize) -> crate::Result<()> {
    let _input = read_input(r, input_len).await?;
    writeln!(w, "Waiting for the problem to be ready!")?;
    Ok(())
}