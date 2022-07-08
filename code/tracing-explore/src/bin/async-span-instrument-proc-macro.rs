#![allow(dead_code)]

fn main() {}

use std::future::Future;

use tracing::{debug_span, info, info_span, Span};

// listing: Async span, instrumented
async fn bar() {
    // This event will fire as a part of the "I'm instrumental" debug level span.
    info!("Hi there");
}

async fn foo() {
    bar().instrument(debug_span!("I'm instrumental")).await;
}
// ~listing

struct Instrumented<T> {
    inner: T,
    span: Span,
}

// listing: Instrument trait
trait Instrument {
    fn instrument(self, span: Span) -> Instrumented<Self> {
        Instrumented { inner: self, span }
    }
}
// ~listing

// listing: Instrument blanket
impl<T> Instrument for T {}
// ~listing

fn hi() {
    // listing: Instrument non-future
    let what = 123.instrument(info_span!("Is this awkward?"));
    // ~listing
}

// listing: Instrumented future impl
impl<T: Future> Future for Instrumented<T> {
    type Output = T::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        let _enter = this.span.enter();
        this.inner.poll(cx)
    }
}
// ~listing


// listing: Instrument method
fn instrument(self, span: Span) -> Instrumented<Self> { }
// ~listing
