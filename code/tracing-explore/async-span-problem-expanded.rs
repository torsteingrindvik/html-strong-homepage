#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
#[allow(dead_code)]
use tracing::debug_span;
async fn bar() {}
async fn foo() {
    let _some_span = {
        use ::tracing::__macro_support::Callsite as _;
        static CALLSITE: ::tracing::callsite::DefaultCallsite = {
            static META: ::tracing::Metadata<'static> = {
                ::tracing_core::metadata::Metadata::new(
                    "Hey",
                    "async_span_problem",
                    ::tracing::Level::DEBUG,
                    Some("src/bin/async-span-problem.rs"),
                    Some(10u32),
                    Some("async_span_problem"),
                    ::tracing_core::field::FieldSet::new(
                        &[],
                        ::tracing_core::callsite::Identifier(&CALLSITE),
                    ),
                    ::tracing::metadata::Kind::SPAN,
                )
            };
            ::tracing::callsite::DefaultCallsite::new(&META)
        };
        let mut interest = ::tracing::subscriber::Interest::never();
        if ::tracing::Level::DEBUG <= ::tracing::level_filters::STATIC_MAX_LEVEL
            && ::tracing::Level::DEBUG <= ::tracing::level_filters::LevelFilter::current()
            && {
                interest = CALLSITE.interest();
                !interest.is_never()
            }
            && ::tracing::__macro_support::__is_enabled(CALLSITE.metadata(), interest)
        {
            let meta = CALLSITE.metadata();
            ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
        } else {
            let span = ::tracing::__macro_support::__disabled_span(CALLSITE.metadata());
            {};
            span
        }
    };
    bar().await;
}
fn main() {}
