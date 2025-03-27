use dioxus::prelude::*;

fn main() {
    dioxus::logger::initialize_default();

    #[cfg(feature = "web")]
    LaunchBuilder::web().launch(frontend::App);

    #[cfg(feature = "server")]
    {
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async move {
                use dioxus::prelude::DioxusRouterExt;
                let axum_app = axum::Router::<()>::new()
                    .route(
                        "/ws",
                        axum::routing::any(backend::wsjrpc::upgrade_websocket),
                    )
                    .serve_dioxus_application(ServeConfig::new().unwrap(), frontend::App);

                let addr = dioxus::cli_config::fullstack_address_or_localhost();
                let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

                axum::serve(listener, axum_app.into_make_service())
                    .await
                    .unwrap();
            })
    }
}
