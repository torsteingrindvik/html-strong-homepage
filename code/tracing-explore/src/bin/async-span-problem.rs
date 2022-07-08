#![allow(dead_code)]

use tracing::debug_span;

// listing 1: Some span, async
// Does nothing :(
async fn bar() {}

async fn foo() { 
    let _some_span = debug_span!("Hey");

    bar().await;
}
// ~listing

fn main() {
}
