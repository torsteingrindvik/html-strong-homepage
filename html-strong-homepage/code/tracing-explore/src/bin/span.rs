use tracing::debug_span;

// listing 1: Some span
fn main() {
    // What happens here?
    let _some_span = debug_span!("Hey", my_data = 123, my_name = "Is confidential");
}
// ~listing
