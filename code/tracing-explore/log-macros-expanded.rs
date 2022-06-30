// Expanded from src/bin/log-macros.rs,
// edited for legibility here and there, with listings added as well.

#![feature(fmt_internals)]
// listing 1: Not our stuff
#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
// ~listing

// listing 2: Our stuff
use tracing::info;
fn main() {
    {
        use tracing::__macro_support::Callsite as _;
        static CALLSITE: ::tracing::callsite::DefaultCallsite = {
            static META: ::tracing::Metadata<'static> = {
                ::tracing_core::metadata::Metadata::new(
                    "event src/bin/log-macros.rs:6",
                    "log_macros",
                    ::tracing::Level::INFO,
                    Some("src/bin/log-macros.rs"),
                    Some(6u32),
                    Some("log_macros"),
                    ::tracing_core::field::FieldSet::new(
                        &["message"],
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
                CALLSITE.metadata().fields().value_set(&[(
                    &iter.next().expect("FieldSet corrupted (this is a bug)"),
                    Some(&::core::fmt::Arguments::new_v1(&["Hey"], &[]) as &dyn Value),
                )])
            });
        } else {
        }
    };
}
// ~listing

// This below did not come from cargo-expand, it's just to have listings.
fn _foo() {
    // listing 3: Callsite use

    // 1: What's this?                       👇
    use tracing::__macro_support::Callsite as _;
    // 👆 2: Whoops here?
    // ~listing

    // listing 4: The Callsite trait
    pub trait Callsite: Sync {
        fn set_interest(&self, interest: Interest);
        fn metadata(&self) -> &Metadata<'_>;
    }
    // ~listing

    // listing 5: Callsite initialization truncated
    // This default impls the trait that got used FYI
    static CALLSITE: ::tracing::callsite::DefaultCallsite = {
        // ⚡Static⚡ stuff.
        static META: ::tracing::Metadata<'static> = {
            ::tracing_core::metadata::Metadata::new(/* hold on */)
        };

        // Takes a ⚡static⚡ reference
        ::tracing::callsite::DefaultCallsite::new(&META)
    };
    // ~listing

    // listing 6: Callsite initialization truncated, but chill
    // This default impls the trait that got used FYI
    static CALLSITE: DefaultCallsite = {
        // ⚡Static⚡ stuff.
        static META: Metadata<'static> = {
            Metadata::new(/* hold on */)
        };

        // Takes a ⚡static⚡ reference
        DefaultCallsite::new(&META)
    };
    // ~listing

    // listing 7: Callsite initialization untruncated
    static CALLSITE: DefaultCallsite = {
        static META: Metadata<'static> = {
            Metadata::new(
                // name
                "event src/bin/log-macros.rs:6",
                // target
                "log_macros",
                // level
                INFO,
                // file
                Some("src/bin/log-macros.rs"),
                // line
                Some(6u32),
                // module
                Some("log_macros"),
                // fields
                FieldSet::new(&["message"], Identifier(&CALLSITE)),
                // kind
                Kind::EVENT,
            )
        };

        DefaultCallsite::new(&META)
    };
    // ~listing

    // listing 8: Are we enabled
    let enabled = INFO <= STATIC_MAX_LEVEL && INFO <= LevelFilter::current() && {
        let interest = CALLSITE.interest();
        !interest.is_never() && __is_enabled(CALLSITE.metadata(), interest)
    };
    if enabled {
        // Stuff
    } else {
    }
    // ~listing

    // listing 9: Enabled! What's next?
    // So this 👇 block is fancy
    (|value_set: ValueSet| {
        let meta = CALLSITE.metadata();
        Event::dispatch(meta, &value_set);
    })
    // (Fancy block end)
    ({
        use tracing::field::{debug, display, Value};
        let mut iter = CALLSITE.metadata().fields().iter();
        CALLSITE.metadata().fields().value_set(&[(
            &iter.next().expect("FieldSet corrupted (this is a bug)"),
            // Let's keep this qualified path since it's not from tracing and all
            // of this seems a bit magic
            Some(&::core::fmt::Arguments::new_v1(&["Hey"], &[]) as &dyn Value),
        )])
    });
    // ~listing

    // listing 10: Enabled! Closure called
    (|value_set: ValueSet| {
        let meta = CALLSITE.metadata();
        Event::dispatch(meta, &value_set);
    })
    // ~listing

    // listing 11: Enabled! Closure input
    ({
        use tracing::field::{debug, display, Value};
        let mut iter = CALLSITE.metadata().fields().iter();
        CALLSITE.metadata().fields().value_set(&[(
            &iter.next().expect("FieldSet corrupted (this is a bug)"),
            // Let's keep this qualified path since it's not from tracing and all
            // of this seems a bit magic
            Some(&::core::fmt::Arguments::new_v1(&["Hey"], &[]) as &dyn Value),
        )])
    });
    // ~listing
}
