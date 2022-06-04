use axum::{
    routing::{get, get_service},
    Router,
};
use std::net::SocketAddr;
use tower::{limit::ConcurrencyLimitLayer, ServiceBuilder};
use tower_http::{
    compression::CompressionLayer,
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};

use html_strong_homepage::{
    blender, blog, common::internal_server_error, home, training, Base, ContentUrl,
};

#[tokio::main]
pub async fn main() {
    tracing_subscriber::fmt::init();

    let content_home = ContentUrl::new(Base::Home);
    let content_blog = ContentUrl::new(Base::Blog);
    let content_blender = ContentUrl::new(Base::Blender);
    let content_training = ContentUrl::new(Base::Training);

    let app = Router::new()
        .route(&content_home.url(), get(home::home))
        .nest(&content_blender.url(), blender::blender())
        .nest(&content_blog.url(), blog::blog())
        .route(&content_training.url(), get(training::training))
        .route(
            "/favicon.ico",
            get_service(ServeFile::new("static/favicon.ico")).handle_error(internal_server_error),
        )
        .nest(
            "/static",
            get_service(ServeDir::new("static")).handle_error(internal_server_error),
        )
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CompressionLayer::new())
                .layer(ConcurrencyLimitLayer::new(64)),
        );

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
