
use defmt::info;
use embassy_executor::task;
use embassy_net::Stack;
use embassy_time::Duration;
use esp_wifi::wifi::{WifiDevice, WifiStaDevice};
use picoserve::{
    response::{Directory, File},
    routing::{get_service, parse_path_segment},
};

use crate::pages::{Index, Input, Solver};

/// Build the picoserve router. Defines http paths and the corresponding handlers.
/// Also defines the static content for the server (favicon, "stylesheet" if you can call it that.)
pub fn make_app() -> picoserve::Router<impl picoserve::routing::PathRouter> {
    let a = picoserve::Router::new()
        .route("/", get_service(Index))
        .route(
            ("/day", parse_path_segment::<u32>()),
            get_service(Input)
                    .post_service(Solver)
        )
        .nest_service(
            "/static",
            const {
                Directory {
                    files: &[
                        ("index.css", File::css(include_str!("static/index.css"))),
                        ("icon.png", File::with_content_type("image/png", include_bytes!("static/icon.png"))),
                    ],
                    ..Directory::DEFAULT
                }
            }
        );
    info!("{}", core::any::type_name_of_val(&a));
    a
}

/// Base level server task.
/// This has a pretty fundamental inefficiency, in that it creates multiple app routers (one for each task)through make_app(). 
/// I don't exactly know the size implications for that yet.
/// The alternative requires nightly compiler which I would prefer to avoid.
/// https://github.com/sammhicks/picoserve/issues/57
#[task(pool_size = crate::HTTP_SERVER_TASKS)]
pub async fn serve(id: usize, stack: &'static Stack<WifiDevice<'static, WifiStaDevice>>) {
    let config = picoserve::Config::new(picoserve::Timeouts {
        start_read_request: Some(Duration::from_secs(5)),
        read_request: Some(Duration::from_secs(1)),
        write: Some(Duration::from_secs(1)),
    })
    .keep_connection_alive();
    let port = 80;
    let mut tcp_rx_buffer = [0; 1024];
    let mut tcp_tx_buffer = [0; 1024];
    let mut http_buffer = [0; 2048];

    picoserve::listen_and_serve(
        id,
        &make_app(), 
        &config,
        stack,
        port,
        &mut tcp_rx_buffer,
        &mut tcp_tx_buffer,
        &mut http_buffer,
    )
    .await;
}
