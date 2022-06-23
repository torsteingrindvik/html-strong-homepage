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
    blender, blog,
    common::internal_server_error,
    home,
    page::{self, Rhs},
    training, Base, ContentUrl,
};

#[tokio::main]
pub async fn main() {
    tracing_subscriber::fmt::init();

    let content_home = ContentUrl::new(Base::Home);

    let blog = page::PageBuilder::new(
        "/blog",
        "Blog series",
        "Here you'll find links to blog series. These will likely be explorations of various Rust \
         related things. 🦀",
    )
    .series(
        "tracing",
        "Tracing",
        "Learning better ways to do distributed logging",
        "Understanding tokio's tracing library.",
        Rhs::code("tracing_subscriber::fmt::init();"),
    )
    .post(
        "overview",
        "Overview",
        "2022-todo-todo",
        "Hello world/tracing! Let's get an overview of what tracing is and why we'd want to use \
         it.",
        Rhs::Nothing,
        blog::tracing::intro(),
    )
    .build();

    let blender = page::PageBuilder::new(
        "/blender",
        "Blender Work Logs",
        "Here follows my work logs (e.g. in-progress images and such).
        
        I might log work from following paid tutorials, youtube videos, or just doodling.
        
        The point anyway is to have something to look back at in the future, and to not take \
         learning Blender too seriously.",
    )
    .series(
        "low-poly-landscapes",
        "Low Poly Landscapes",
        "I love this style!",
        "A stylized tutorial by Grant Abbitt on creating low polygon count landscapes.",
        Rhs::two_images("after-the-mirror-modifier.webp", "goal.webp"),
    )
    .series(
        "low-poly-characters",
        "Low Poly Characters",
        "We have to populate our worlds with something, right?",
        "A stylized tutorial by Grant Abbitt on creating low polygon count characters.",
        Rhs::two_images("simple-character/simple-character.webp", "goal.webp"),
    )
    .post(
        "hello-world",
        "Hello lowpoly character world!",
        "2022-06-15",
        "Starting out this tutorial, let's see how far we get.",
        Rhs::one_image("adding-objects.webp"),
        blender::low_poly_characters::hello_world(),
    )
    .post(
        "simple-character",
        "Simple character",
        "2022-06-16",
        "Making simple blocky characters.",
        Rhs::one_image("cool-character.webp"),
        blender::low_poly_characters::simple_character(),
    )
    .post(
        "block-characters",
        "Block Characters",
        "2022-06-18",
        "Using reference images to trace some slightly more advanced characters.",
        Rhs::one_image("trace-leg.webp"),
        blender::low_poly_characters::block_characters(),
    )
    .build();

    let training = page::PageBuilder::new(
        "/training",
        "Training notes",
        "Notes from videos about working out. Written in shorthand, so likely only understood by \
         me!",
    )
    .series(
        "hypertrophy",
        "Hypertrophy",
        "Gotta get stronk 🏋️",
        "Notes from videos specifically about hypertrophy.",
        Rhs::Nothing,
    )
    .post(
        "galpin-huberman-podcast",
        "Strength, Muscle Size & Endurance",
        "So how does one grow?",
        "Dr. Andy Galpin: How to Build Strength, Muscle Size & Endurance | Huberman Lab Podcast \
         #65",
        Rhs::Nothing,
        training::huberman_podcast_with_andy_galpin(),
    )
    .post(
        "eating-for-hypertrophy",
        "Eating for Hypertrophy",
        "So how does one eat?",
        "A 5 minute video by Andy Galpin.",
        Rhs::Nothing,
        training::eating_for_hypertrophy(),
    )
    .post(
        "new-science-of-muscle-hypertrophy-1",
        "New Science of Muscle Hypertrophy 1",
        "So how do you grow literally?",
        "A long series by Dr. Andy Galpin. Episode theme: Physiology.",
        Rhs::Nothing,
        training::new_science_of_muscle_hypertrophy_1(),
    )
    .post(
        "new-science-of-muscle-hypertrophy-2",
        "New Science of Muscle Hypertrophy 2",
        "How do you signal the start of growth?",
        "A long series by Dr. Andy Galpin. Episode theme: Stimuli.",
        Rhs::Nothing,
        training::new_science_of_muscle_hypertrophy_2(),
    )
    .post(
        "new-science-of-muscle-hypertrophy-3",
        "New Science of Muscle Hypertrophy 3",
        "What do you eat?",
        "A long series by Dr. Andy Galpin. Episode theme: Eating and training.",
        Rhs::Nothing,
        training::new_science_of_muscle_hypertrophy_3(),
    )
    .build();

    let app = Router::new()
        .route(&content_home.url(), get(home::home))
        .nest(blog.url(), blog.router())
        .nest(blender.url(), blender.router())
        .nest(training.url(), training.router())
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
