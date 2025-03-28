fn main() {
    tracing_subscriber::fmt::init();

    tokio::runtime::Runtime::new().unwrap().block_on(async move {
        let axum_app = axum::Router::<()>::new().route(
            "/ws",
            axum::routing::any(backend::wsjrpc::upgrade_websocket),
        );

        let addr = "0.0.0.0:8080";
        let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

        axum::serve(listener, axum_app.into_make_service())
            .await
            .unwrap();
    })
}
