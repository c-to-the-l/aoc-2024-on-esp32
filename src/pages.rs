use core::fmt::Write;

use alloc::vec::Vec;

mod index;
pub use index::Index;

mod input;
pub use input::Input;

mod solver;
use picoserve::response::chunked::{ChunkWriter, ChunkedResponse, Chunks, ChunksWritten};
pub use solver::Solver;

use crate::Result;

pub const HTML_HEADER: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>AOC on ESP32</title>
<link rel="stylesheet" href="/static/index.css">
<link rel="icon" href="/static/icon.png">
</head>
<body>"#;

pub const HTML_FOOTER: &str = r#"<hr><a href="/">Return Home</a><br></body></html>"#;

/// A helper type for rendering HTML pages. It can be used to insert default headers & footers, 
/// and implements various `fmt::Write` traits to allow use of `writeln!` macros.
/// 
/// I won't offend smarter people by calling it "templating", and there is no `no_std` template engine.
pub struct HtmlPage {
    content: alloc::vec::Vec<u8>,
}

impl HtmlPage {
    /// Create a new `HtmlPage`
    pub fn new() -> Self {
        Self {
            content: Vec::new(),
        }
    }

    /// Provide a size hint for the internal buffer. 
    /// Can speed up page rendering if you roughly know in advance how big your page will be
    pub fn with_size_hint(mut self, hint: usize) -> Self {
        let cap = self.content.capacity();
        if cap < hint {
            self.content.reserve(cap.saturating_sub(hint));
        }
        self
    }

    /// Append the header to the end of the buffer. 
    /// It's expected that you'll call this function first and once.
    /// You can call this multiple times but you'll get multiple invalid headers. Do you want that?
    pub fn insert_header(&mut self) -> Result<()> {
        self.write_str(HTML_HEADER)?;
        Ok(())
    }

    /// Inserts the footer. Do this last before you return.
    pub fn insert_footer(&mut self) -> Result<()> {
        self.write_str(HTML_FOOTER)?;
        Ok(())
    }

    pub fn len(&self) -> usize {
        self.content.len()
    }

    /// Convert this page into a chunk writer, suitable for sending as a reply.
    /// This is mandatory, more specifically it is required for any page that is larger than your TCP MTU.
    pub fn into_chunks(self) -> ChunkedResponse<Self> {
        ChunkedResponse::new(self)
    }
}

impl Chunks for HtmlPage {
    fn content_type(&self) -> &'static str {
        "text/html"
    }
    
    async fn write_chunks<W: picoserve::io::Write>(
        self,
        mut chunk_writer: ChunkWriter<W>,
    ) -> core::result::Result<ChunksWritten, W::Error> {
        // just a guess at an appropriate chunk size? Completely depends on the TCP MTU but we 
        // can't detect that.
        for chunk in self.content.chunks(1200) {
            chunk_writer.write_chunk(chunk).await?;
        }
        chunk_writer.finalize().await
    }
}

impl core::fmt::Write for HtmlPage {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.content.extend_from_slice(s.as_bytes());
        Ok(())
    }
}