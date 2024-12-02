use core::fmt::Write;

use defmt::{error, info};
use picoserve::response::IntoResponse;

use crate::{error::AerError, pages::HtmlPage, problems, Result};

pub async fn do_problem<R: picoserve::io::Read>(
    r: &mut R,
    day: u32,
    input_len: usize,
) -> Result<HtmlPage> {
    info!("Doing day {}", day);
    let mut page = HtmlPage::new().with_size_hint(2048);
    page.insert_header()?;
    if !(1..=25).contains(&day) {
        error!("Invalid day {}", day);
        writeln!(page, "<h1>Unrecognised Day</h1>")?;
        writeln!(page, "<h2>Day {day} doesn't exist!</h2>")?;
        page.insert_footer()?;
        return Ok(page);
    }
    writeln!(page, "<h1>Advent of Code day {day}</h1><hr>")?;
    writeln!(page, "<code>")?;
    info!("Problem Start");
    let start = embassy_time::Instant::now();
    let result = lookup_problem(r, &mut page, day, input_len).await;
    if let Err(e) = result {
        writeln!(page, "<br>Encountered error: {e}")?;
    }
    writeln!(page, "Evaluated in {}ms", start.elapsed().as_millis())?;
    writeln!(page, r"</code>")?;
    page.insert_footer()?;
    Ok(page)
}

async fn lookup_problem<R: picoserve::io::Read, W: core::fmt::Write>(
    r: &mut R,
    w: &mut W,
    day: u32,
    input_len: usize,
) -> Result<()> {
    match day {
        1 => problems::p01::solve(r, w, input_len).await,
        2 => problems::p02::solve(r, w, input_len).await,
        3 => problems::p03::solve(r, w, input_len).await,
        _ => Err(AerError::BadDay { day }),
    }
}

pub struct Solver;
impl picoserve::routing::RequestHandlerService<(), (u32,)> for Solver {
    async fn call_request_handler_service<
        R: embedded_io_async::Read,
        W: picoserve::response::ResponseWriter<Error = R::Error>,
    >(
        &self,
        _state: &(),
        (day,): (u32,),
        mut r: picoserve::request::Request<'_, R>,
        w: W,
    ) -> core::result::Result<picoserve::ResponseSent, W::Error> {
        let content_length = r.body_connection.content_length();
        info!("Doing problem {}, input length {}", day, content_length);
        match do_problem(&mut r.body_connection.body().reader(), day, content_length).await {
            Ok(page) => {
                info!("Problem complete, Response size {}", page.len());
                page.into_chunks()
                    .into_response()
                    .write_to(r.body_connection.finalize().await?, w)
                    .await
            },
            Err(e) => {
                error!("Error when doing problem: {:?}", e);
                format_args!(
                    r#"Error when processing day {day}: {e}<hr><a href="/">Return Home</a><br>"#
                )
                .write_to(r.body_connection.finalize().await?, w)
                .await
            },
        }
    }
}
