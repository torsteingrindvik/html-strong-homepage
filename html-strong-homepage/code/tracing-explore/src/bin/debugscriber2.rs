#![allow(dead_code)]

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use tracing::field::Visit;
use tracing::{field, info, info_span, span, warn_span, Metadata, Subscriber};

// listing: A simple visitor
struct DbgVisitor;

impl Visit for DbgVisitor {
    fn record_debug(&mut self, field: &field::Field, value: &dyn std::fmt::Debug) {
        print!(" {field}={value:?}");
    }
}
// ~listing

// listing: Making unique IDs for spans
// Uses the fact that tracing is able to give us a unique callsite identifier
// which can be hashed.
// Uses the u64 given by hashing that, and voila we have a unique-per-callsite ID.
fn span_id_from_metadata(metadata: &'static Metadata<'static>) -> span::Id {
    let mut hasher = DefaultHasher::new();
    metadata.callsite().hash(&mut hasher);

    span::Id::from_non_zero_u64(hasher.finish().try_into().expect("non-zero u64"))
}
// ~listing

// listing: Impl which just prints things
struct DbgScriber;

impl Subscriber for DbgScriber {
    fn enabled(&self, metadata: &Metadata<'_>) -> bool {
        println!("[enabled?]: {} {}", metadata.level(), metadata.name());
        true
    }

    fn new_span(&self, span: &span::Attributes<'_>) -> span::Id {
        print!(
            "[new_span]: {} {}",
            span.metadata().level(),
            span.metadata().name(),
        );
        span.record(&mut DbgVisitor);
        println!();

        span_id_from_metadata(span.metadata())
    }

    fn record(&self, span: &span::Id, values: &span::Record<'_>) {
        print!("[record]: {span:?}");

        values.record(&mut DbgVisitor);
        println!();
    }

    fn record_follows_from(&self, span: &span::Id, follows: &span::Id) {
        println!("[record_follows_from]: {span:?} follows {follows:?}");
    }

    fn event(&self, event: &tracing::Event<'_>) {
        print!(
            "[event]: {} {}",
            event.metadata().level(),
            event.metadata().name(),
        );

        event.record(&mut DbgVisitor);
        println!();
    }

    fn enter(&self, span: &span::Id) {
        println!("[enter]: {span:?}");
    }

    fn exit(&self, span: &span::Id) {
        println!("[exit]: {span:?}");
    }
}
// ~listing

// listing: Printing subscriber use
fn main() {
    // Gotta make sure our subscriber is actually in use.
    tracing::subscriber::set_global_default(DbgScriber).expect("init should work");

    // Let's have one span we enter..
    let parent_span_guard = info_span!("I'm your parent").entered();

    // Run it twice to spot any differences.
    for bah in 0..2 {
        info!("Hi!");

        // ..and one span we just create.
        let s = warn_span!("Danger zone", bah, oh = field::Empty);

        // This is new. We indicate a "follows from" relationship.
        s.follows_from(parent_span_guard.id());

        // New as well. We provide a value for the "oh" placeholder.
        s.record("oh", &(bah * 2));

        info!("Hi, but more wary");
    }
}
// ~listing

struct Foo;
impl Foo {
    // listing: Simplest subscriber fns
    fn enabled(&self, metadata: &Metadata<'_>) -> bool {
        println!("[enabled?]: {} {}", metadata.level(), metadata.name());
        true
    }

    fn record_follows_from(&self, span: &span::Id, follows: &span::Id) {
        println!("[record_follows_from]: {span:?} follows {follows:?}");
    }

    fn enter(&self, span: &span::Id) {
        println!("[enter]: {span:?}");
    }

    fn exit(&self, span: &span::Id) {
        println!("[exit]: {span:?}");
    }
    // ~listing

    // listing: Event and new_span
    fn event(&self, event: &tracing::Event<'_>) {
        print!(
            "[event]: {} {}",
            event.metadata().level(),
            event.metadata().name(),
        );

        event.record(&mut DbgVisitor);
        println!();
    }

    fn new_span(&self, span: &span::Attributes<'_>) -> span::Id {
        print!(
            "[new_span]: {} {}",
            span.metadata().level(),
            span.metadata().name(),
        );
        span.record(&mut DbgVisitor);
        println!();

        span_id_from_metadata(span.metadata())
    }
    // ~listing
}
