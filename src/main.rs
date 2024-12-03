use axum::{
    routing::get,
    Router,
};

use std::{net::SocketAddr, path::PathBuf};
use tower_http::{
    services::ServeDir,
    trace::{DefaultMakeSpan, TraceLayer},
};

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod ws {
    pub mod iot_server_rust;
}

mod redis_manager {
    pub mod channels {
        pub mod secnav;
        pub mod secsrv;
        pub mod rpmgnt;
        pub mod secmgnt;
    }
    pub mod utils  {
        pub mod get_redis_value;
        pub mod set_redis_value;
    }
}

mod lib {
    pub mod cmd_linux;
    pub mod infos_system;
    pub mod reset_websocket;
}

use ws::iot_server_rust;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "iot_server_rust=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let assets_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets");

    let app = Router::new()
        .fallback_service(ServeDir::new(assets_dir).append_index_html_on_directories(true))
        .route("/iot", get(iot_server_rust::ws_handler))
        // logging so we can see whats going on
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        );

    // run it with hyper
    let listener = tokio::net::TcpListener::bind("127.0.0.1:6002")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
