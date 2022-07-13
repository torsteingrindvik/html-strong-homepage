// Expanded from src/bin/log-macros-fields-values.rs

#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use tracing::info;
struct MyStruct {
    _v: u8,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for MyStruct {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            Self { _v: ref __self_0_0 } => ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "MyStruct",
                "_v",
                &&(*__self_0_0),
            ),
        }
    }
}
fn main() {
    let cat: i8;
    cat = -1;
    {
        use tracing::__macro_support::Callsite as _;
        static CALLSITE: ::tracing::callsite::DefaultCallsite = {
            static META: ::tracing::Metadata<'static> = {
                ::tracing_core::metadata::Metadata::new(
                    "event src/bin/log-macros-fields-values.rs:14",
                    "log_macros_fields_values",
                    ::tracing::Level::INFO,
                    Some("src/bin/log-macros-fields-values.rs"),
                    Some(14u32),
                    Some("log_macros_fields_values"),
                    ::tracing_core::field::FieldSet::new(
                        &["message", "foo", "thing", "hi", "cat"],
                        ::tracing_core::callsite::Identifier(&CALLSITE),
                    ),
                    ::tracing::metadata::Kind::EVENT,
                )
            };
            ::tracing::callsite::DefaultCallsite::new(&META)
        };
        let enabled = ::tracing::Level::INFO <= ::tracing::level_filters::STATIC_MAX_LEVEL
            && ::tracing::Level::INFO <= ::tracing::level_filters::LevelFilter::current()
            && {
                let interest = CALLSITE.interest();
                !interest.is_never()
                    && ::tracing::__macro_support::__is_enabled(CALLSITE.metadata(), interest)
            };
        if enabled {
            (|value_set: ::tracing::field::ValueSet| {
                let meta = CALLSITE.metadata();
                ::tracing::Event::dispatch(meta, &value_set);
            })({
                #[allow(unused_imports)]
                use tracing::field::{debug, display, Value};
                let mut iter = CALLSITE.metadata().fields().iter();
                CALLSITE.metadata().fields().value_set(&[
                    (
                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                        Some(&::core::fmt::Arguments::new_v1(&["Hey"], &[]) as &Value),
                    ),
                    (
                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                        Some(&123 as &Value),
                    ),
                    (
                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                        Some(&"Something" as &Value),
                    ),
                    (
                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                        Some(&debug(&MyStruct { _v: 10 }) as &Value),
                    ),
                    (
                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                        Some(&cat as &Value),
                    ),
                ])
            });
        } else {
        }
    };
}

// Expanded stops here, listings added below for use in blog.

// listing 1: Metadata with field set
static META: Metadata<'static> = {
    Metadata::new(
        "event src/bin/log-macros-fields-values.rs:14",
        "log_macros_fields_values",
        INFO,
        Some("src/bin/log-macros-fields-values.rs"),
        Some(14u32),
        Some("log_macros_fields_values"),
        FieldSet::new(
            &["message", "foo", "thing", "hi", "cat"],
            Identifier(&CALLSITE),
        ),
        Kind::EVENT,
    )
};
// ~listing

fn foo() {
    // listing 2: Dispatch event with fields, values
    (|value_set: ValueSet| {
        let meta = CALLSITE.metadata();
        Event::dispatch(meta, &value_set);
    })({
        use tracing::field::{debug, display, Value};
        let mut iter = CALLSITE.metadata().fields().iter();
        CALLSITE.metadata().fields().value_set(&[
            (
                &iter.next().expect("FieldSet corrupted (this is a bug)"),
                Some(&::core::fmt::Arguments::new_v1(&["Hey"], &[]) as &Value),
            ),
            (
                &iter.next().expect("FieldSet corrupted (this is a bug)"),
                Some(&123 as &Value),
            ),
            (
                &iter.next().expect("FieldSet corrupted (this is a bug)"),
                Some(&"Something" as &Value),
            ),
            (
                &iter.next().expect("FieldSet corrupted (this is a bug)"),
                Some(&debug(&MyStruct { _v: 10 }) as &Value),
            ),
            (
                &iter.next().expect("FieldSet corrupted (this is a bug)"),
                Some(&cat as &Value),
            ),
        ])
    });
}
// ~listing
