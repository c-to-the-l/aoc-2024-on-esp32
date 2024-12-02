use defmt::{error, info};
use picoserve::response::IntoResponse;
use portable_atomic::{AtomicU16, Ordering};

use crate::pages::HtmlPage;

use core::fmt::Write;

pub struct Index;

static CTR: AtomicU16 = AtomicU16::new(0);


pub fn serve_index_page() -> crate::Result<HtmlPage> {
    let mut page = HtmlPage::new()
        .with_size_hint(1024);
    page.insert_header()?;
    writeln!(page, "<h1>Advent of Code Solver</h1><br><hr>")?;
    writeln!(page, "Choose a day to solve:<ul>")?;
    for day in 1..=25 {
        writeln!(page, r#"<li><a href="/day/{day}">Day {day}</a></li>"#)?;
    }
    writeln!(page, "</ul><hr>")?;
    writeln!(page, "This page has been requested {} times", CTR.fetch_add(1, Ordering::Relaxed))?;
    page.insert_footer()?;
    Ok(page)
}


impl picoserve::routing::RequestHandlerService<()> for Index {
    async fn call_request_handler_service<
        R: embedded_io_async::Read,
        W: picoserve::response::ResponseWriter<Error = R::Error>,
    >(
        &self,
        _state: &(),
        _params: (),
        r: picoserve::request::Request<'_, R>,
        w: W,
    ) -> Result<picoserve::ResponseSent, W::Error> {
        match serve_index_page() {
            Ok(page) => {
                info!("Index page rendered, size {}", page.len());
                page.into_chunks().into_response()
                    .write_to(r.body_connection.finalize().await?, w)
                    .await
            }
            Err(e) => {
                error!("Error when trying to render index page: {:?}", e);
                format_args!("Error when trying to render index page")
                    .write_to(r.body_connection.finalize().await?, w)
                    .await
            },

        }
    }
}
