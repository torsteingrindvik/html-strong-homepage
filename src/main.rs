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
    herbs, home,
    page::{self, Rhs},
    training, Base, ContentUrl,
};

#[cfg(feature = "tls")]
async fn serve_tls(app: Router) {
    use axum_server::tls_rustls::RustlsConfig;

    let addr = SocketAddr::from(([0, 0, 0, 0], 443));
    println!("listening on {}", addr);

    let config = RustlsConfig::from_pem_file(
        "/etc/letsencrypt/live/torste.in/fullchain.pem",
        "/etc/letsencrypt/live/torste.in/privkey.pem",
    )
    .await
    .unwrap();

    axum_server::bind_rustls(addr, config)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[cfg(feature = "tls")]
async fn http_upgrade(uri: axum::http::Uri) -> axum::response::Redirect {
    use axum::response::Redirect;

    let uri = format!("https://torste.in:443{}", uri.path());
    Redirect::temporary(&uri)
}

#[cfg(feature = "tls")]
async fn serve_acme() {
    let acme_app = Router::new()
        .nest(
            "/.well-known/acme-challenge",
            get_service(ServeDir::new("acme/.well-known/acme-challenge"))
                .handle_error(internal_server_error),
        )
        .fallback(get(http_upgrade));

    serve(acme_app).await
}

async fn serve(app: Router) {
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    println!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

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
    .post(
        "cyborg",
        "Cyborg",
        "2022-06-23",
        "Having some fun, creating a cyborg!",
        Rhs::one_image("cyborg.webp"),
        blender::low_poly_characters::cyborg(),
    )
    .post(
        "cyborg-follow-up",
        "Cyborg Follow-Up",
        "2022-07-03",
        "Let's finish up the cyborg with some emissive materials.",
        Rhs::one_image("cyborg-lights.webp"),
        blender::low_poly_characters::cyborg_follow_up(),
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

    let herbs = page::PageBuilder::new("/herbs", "Herbs", "Let's try growing some of these!")
        .series(
            "basil",
            "Basil",
            "Homemade pesto yum",
            "Posts about growing basil",
            Rhs::one_image("hello-world/mother-plant.webp"),
        )
        .post(
            "hello-world",
            "Starting out",
            "2022-06-23",
            "Starting out basil growth from a store bought mother plant.",
            Rhs::one_image("pesto.webp"),
            herbs::basil::hello_world(),
        )
        .post(
            "death-and-decay",
            "Death and Decay",
            "2022-07-02",
            "And... hope?",
            Rhs::one_image("mother-plant.webp"),
            herbs::basil::death_and_decay(),
        )
        .build();

    let app = Router::new()
        .route(&content_home.url(), get(home::home))
        .nest(blog.url(), blog.router())
        .nest(blender.url(), blender.router())
        .nest(training.url(), training.router())
        .nest(herbs.url(), herbs.router())
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

    // If using TLS, we need to have a separate
    // server running on port 80 for Let's Encrypt to be able
    // to renew.
    #[cfg(feature = "tls")]
    tokio::join!(serve_tls(app), serve_acme());

    #[cfg(not(feature = "tls"))]
    serve(app).await;
}
