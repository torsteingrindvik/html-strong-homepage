#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use tracing::debug_span;

// listing 1: Some span, expanded
fn main() {
    let _some_span = {
        use Callsite as _;
        // A callsite, very similar to the previous time we saw this.
        static CALLSITE: DefaultCallsite = {
            static META: Metadata<'static> = {
                Metadata::new(
                    "Hey",
                    "span",
                    DEBUG,
                    Some("src/bin/span.rs"),
                    Some(6u32),
                    Some("span"),
                    FieldSet::new(&["my_data", "my_name"], Identifier(&CALLSITE)),
                    // Now span, not event!
                    Kind::SPAN,
                )
            };
            DefaultCallsite::new(&META)
        };
        // This too checks similar things we've seen before.
        // The basic goal: Is there any subscriber that might want to know about this span?
        let mut interest = Interest::never();
        if DEBUG <= level_filters::STATIC_MAX_LEVEL
            && Level::DEBUG <= LevelFilter::current()
            && {
                interest = CALLSITE.interest();
                !interest.is_never()
            }
            && __is_enabled(CALLSITE.metadata(), interest)
        {
            // Someone might care..
            let meta = CALLSITE.metadata();
            Span::new(meta, &{
                use tracing::field::{debug, display, Value};
                let mut iter = meta.fields().iter();
                meta.fields().value_set(&[
                    (
                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                        Some(&123 as &Value),
                    ),
                    (
                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                        Some(&"Is confidential" as &Value),
                    ),
                ])
            })
        } else {
            // No chance anyone cares.
            let span = __disabled_span(CALLSITE.metadata());
            {};
            span
        }
    };
}
// ~listing

// Interesting parts:

fn _foo() {
    // listing 2: Span creation
    let meta = CALLSITE.metadata();
    Span::new(meta, &{
        use tracing::field::{debug, display, Value};
        let mut iter = meta.fields().iter();
        meta.fields().value_set(&[
            (
                &iter.next().expect("FieldSet corrupted (this is a bug)"),
                Some(&123 as &Value),
            ),
            (
                &iter.next().expect("FieldSet corrupted (this is a bug)"),
                Some(&"Is confidential" as &Value),
            ),
        ])
    })
    // ~listing
}
