// Recursive expansion of instrument! macro
// =========================================

// listing: Instrument proc macro expanded
async fn foo() {
    {}
    let __tracing_attr_span =
        tracing::span!(target: module_path!(), tracing::Level::INFO, "foodie",);
    let __tracing_instrument_future = async move {
        let _thing = 123;
        bar().await;
    };
    if !__tracing_attr_span.is_disabled() {
        tracing::Instrument::instrument(__tracing_instrument_future, __tracing_attr_span).await
    } else {
        __tracing_instrument_future.await
    }
}
// ~listing