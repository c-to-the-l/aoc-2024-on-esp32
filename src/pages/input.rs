use core::fmt::Write;

use defmt::{error, info};
use picoserve::response::IntoResponse;

use crate::{pages::HtmlPage, Result};

const FORM_DATA: &str = r#"<h2>Paste input into the box and hit submit:</h2>
<form enctype="text/plain" method="post">
<textarea name="message" rows="20" cols="80"></textarea>
<input type="submit">
</form>
"#;

pub struct Input;

fn serve_input_page(day: u32) -> Result<HtmlPage> {
    let mut page = HtmlPage::new()
        .with_size_hint(1024);
    page.insert_header()?;
    if (1..=25).contains(&day) {
        writeln!(page, "<h1>Advent of Code day {day}</h1>")?;
        page.write_str(FORM_DATA)?;
    } else {
        writeln!(page, "<h1>Unrecognised Day</h1>")?;
        writeln!(page, "<h2>Day {day} doesn't exist!</h2>")?;
    }
    page.insert_footer()?;
    Ok(page)
}

impl picoserve::routing::RequestHandlerService<(), (u32,)> for Input {
    async fn call_request_handler_service<
        R: embedded_io_async::Read,
        W: picoserve::response::ResponseWriter<Error = R::Error>,
    >(
        &self,
        _state: &(),
        (day,): (u32,),
        r: picoserve::request::Request<'_, R>,
        w: W,
    ) -> core::result::Result<picoserve::ResponseSent, W::Error> {
        match serve_input_page(day) {
            Ok(page) => {
                info!("Index page rendered, size {}", page.len());
                page.into_chunks().into_response()
                    .write_to(r.body_connection.finalize().await?, w)
                    .await
            }
            Err(e) => {
                error!("Error when trying to render input prompt: {:?}", e);
                format_args!("Error when trying to render index page")
                    .write_to(r.body_connection.finalize().await?, w)
                    .await
            },
        }

    }
}