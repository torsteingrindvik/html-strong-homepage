use axum::{
    routing::{get, get_service},
    Extension, Router,
};
use chrono::{Local, TimeZone};
use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use timelapsifier::TimestampedFile;
use tokio::sync::RwLock;
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
use tracing::info;

#[cfg(feature = "tls")]
async fn serve_tls(app: Router) {
    use axum_server::tls_rustls::RustlsConfig;

    let addr = SocketAddr::from(([0, 0, 0, 0], 443));
    info!("listening on {}", addr);

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
    // If TLS, need to redirect from 80 -> 443.
    #[cfg(feature = "tls")]
    let addr = SocketAddr::from(([0, 0, 0, 0], 80));

    #[cfg(not(feature = "tls"))]
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));

    info!("listening on {}", addr);

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
        Local.ymd(1999, 1, 1),
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
    .post(
        "hello-landscapes",
        "Cabin",
        Local.ymd(2022, 07, 26),
        "We get quite far by finishing the cabin of this tutorial.",
        Rhs::one_image("cabin-done.webp"),
        blender::low_poly_landscapes::hello_world(),
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
        Local.ymd(2022, 06, 15),
        "Starting out this tutorial, let's see how far we get.",
        Rhs::one_image("adding-objects.webp"),
        blender::low_poly_characters::hello_world(),
    )
    .post(
        "simple-character",
        "Simple character",
        Local.ymd(2022, 06, 16),
        "Making simple blocky characters.",
        Rhs::one_image("cool-character.webp"),
        blender::low_poly_characters::simple_character(),
    )
    .post(
        "block-characters",
        "Block Characters",
        Local.ymd(2022, 06, 18),
        "Using reference images to trace some slightly more advanced characters.",
        Rhs::one_image("trace-leg.webp"),
        blender::low_poly_characters::block_characters(),
    )
    .post(
        "cyborg",
        "Cyborg",
        Local.ymd(2022, 06, 23),
        "Having some fun, creating a cyborg!",
        Rhs::one_image("cyborg.webp"),
        blender::low_poly_characters::cyborg(),
    )
    .post(
        "cyborg-follow-up",
        "Cyborg Follow-Up",
        Local.ymd(2022, 07, 03),
        "Let's finish up the cyborg with some emissive materials.",
        Rhs::one_image("cyborg-lights.webp"),
        blender::low_poly_characters::cyborg_follow_up(),
    )
    .post(
        "more-folks",
        "More folks!",
        Local.ymd(2022, 07, 23),
        "Course finished! More folks created, also texturing via UV editing and some rigging.",
        Rhs::one_image("pose.webp"),
        blender::low_poly_characters::more_folks(),
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
        Local.ymd(2022, 06, 04),
        "Dr. Andy Galpin: How to Build Strength, Muscle Size & Endurance | Huberman Lab Podcast \
         #65",
        Rhs::Nothing,
        training::huberman_podcast_with_andy_galpin(),
    )
    .post(
        "eating-for-hypertrophy",
        "Eating for Hypertrophy",
        Local.ymd(2022, 06, 04),
        "A 5 minute video by Andy Galpin.",
        Rhs::Nothing,
        training::eating_for_hypertrophy(),
    )
    .post(
        "new-science-of-muscle-hypertrophy-1",
        "New Science of Muscle Hypertrophy 1",
        Local.ymd(2022, 06, 04),
        "A long series by Dr. Andy Galpin. Episode theme: Physiology.",
        Rhs::Nothing,
        training::new_science_of_muscle_hypertrophy_1(),
    )
    .post(
        "new-science-of-muscle-hypertrophy-2",
        "New Science of Muscle Hypertrophy 2",
        Local.ymd(2022, 06, 04),
        "A long series by Dr. Andy Galpin. Episode theme: Stimuli.",
        Rhs::Nothing,
        training::new_science_of_muscle_hypertrophy_2(),
    )
    .post(
        "new-science-of-muscle-hypertrophy-3",
        "New Science of Muscle Hypertrophy 3",
        Local.ymd(2022, 06, 04),
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
            Local.ymd(2022, 06, 23),
            "Starting out basil growth from a store bought mother plant.",
            Rhs::one_image("pesto.webp"),
            herbs::basil::hello_world(),
        )
        .post(
            "death-and-decay",
            "Death and Decay",
            Local.ymd(2022, 07, 02),
            "And... hope?",
            Rhs::one_image("mother-plant.webp"),
            herbs::basil::death_and_decay(),
        )
        .post(
            "big-changes",
            "Big Changes",
            Local.ymd(2022, 07, 07),
            "We spend money and things happen.",
            Rhs::one_image("plants-filled.webp"),
            herbs::basil::big_changes(),
        )
        .post(
            "seeds",
            "Seeds",
            Local.ymd(2022, 07, 09),
            "We spend money and other things happen.",
            Rhs::one_image("whole-family.webp"),
            herbs::basil::seeds(),
        )
        .post(
            "pruning",
            "Pruning",
            Local.ymd(2022, 07, 16),
            "Let's cut down things.",
            Rhs::one_image("done.webp"),
            herbs::basil::pruning(),
        )
        .build();

    let timelapse_output_folder = PathBuf::from(format!(
        "{}/{}",
        env!("CARGO_MANIFEST_DIR"),
        shared::herbs::timelapse_output_relative_folder()
    ));
    let timelapse_videos = timelapsifier::files_of_ext_in(&timelapse_output_folder, &["mp4"])
        .await
        .into_iter()
        .filter_map(|video| TimestampedFile::new_ymd(video).ok())
        .collect();

    let timelapse_options = timelapsifier::TimelapserOptions {
        unprocessed_images_folder: PathBuf::from(format!(
            "{}/{}",
            env!("CARGO_MANIFEST_DIR"),
            shared::herbs::new_image_output_relative_folder()
        )),
        processed_images_folder: PathBuf::from(format!(
            "{}/{}",
            env!("CARGO_MANIFEST_DIR"),
            shared::herbs::processed_image_output_relative_folder()
        )),
        timelapse_output_folder,
        timelapse_videos: Arc::new(RwLock::new(timelapse_videos)),
    };
    // Whenever a new video is created, its path is updated in this state.
    // Share this state with the herbs post which displays these.
    let videos = timelapse_options.timelapse_videos.clone();

    let (herbs_new_image, herbs_new_image_router) = herbs::timelapsify_init(timelapse_options);

    let mut all_posts = vec![&blog, &blender, &training, &herbs]
        .iter()
        .flat_map(|page| {
            page.posts()
                .iter()
                .map(|post| post.lead())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    all_posts.sort_by_key(|post| post.date);

    let all_posts = Arc::new(all_posts);

    let app = Router::new()
        .route(
            &content_home.url(),
            get(home::home).layer(Extension(all_posts)),
        )
        .nest(blog.url(), blog.router())
        .nest(blender.url(), blender.router())
        .nest(training.url(), training.router())
        .nest(herbs.url(), herbs.router())
        // TODO: Merge these into one thing
        .nest(herbs_new_image, herbs_new_image_router)
        .nest(
            "/timelapse",
            Router::new()
                .route("/", get(herbs::basil::timelapse))
                .layer(Extension(videos)),
        )
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
