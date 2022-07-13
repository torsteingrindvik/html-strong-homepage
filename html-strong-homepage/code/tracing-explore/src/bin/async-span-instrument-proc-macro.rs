fn main() {}

use tracing::instrument;

// listing: Instrument proc macro
async fn bar() {}

#[instrument(name = "foodie")]
async fn foo() {
    let _thing = 123;
    bar().await;
}
// ~listing
