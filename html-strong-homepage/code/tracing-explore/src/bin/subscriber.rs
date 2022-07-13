use std::sync::atomic::AtomicU64;

use tracing::span::Attributes;
use tracing::{info, info_span, span, Metadata, Subscriber};

struct MySubscriber {
    // This was not allowed, we need Sync
    // latest_id: RefCell<u64>,

    // Easier than Arc<Mutex>
    latest_id: AtomicU64,
}

impl MySubscriber {
    fn new() -> Self {
        Self {
            // Plz dont use 0
            latest_id: 1.into(),
        }
    }
}

impl Subscriber for MySubscriber {
    fn enabled(&self, metadata: &Metadata<'_>) -> bool {
        // So here I think I have to look at metadata, and figure out
        // if a span/event with that metadata would be recorded.

        dbg!("Enabled:", metadata);
        true
        // false
    }

    fn new_span(&self, span: &Attributes<'_>) -> span::Id {
        // Someone is creating a span.. we have to provide an id for it.
        // Hmm but we can't mutate self.. hmm, so we wrapped our inner thing in refcell and see what happens.
        //
        // Also id == 0 is not allowed, so increment before returning.
        //
        // Also we are now not thread safe.
        //
        // UPDATE: Switched to atomics, this works, needed Sync
        dbg!("New span:", span);
        // *self.latest_id.borrow_mut() += 1;
        let id = self
            .latest_id
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);

        span::Id::from_u64(id)
    }

    fn record(&self, span: &span::Id, values: &span::Record<'_>) {
        dbg!("Record:", span, values);
    }

    fn record_follows_from(&self, span: &span::Id, follows: &span::Id) {
        dbg!("Record follows from: ", span, follows);
    }

    fn event(&self, event: &tracing::Event<'_>) {
        dbg!("Event:", event);
    }

    fn enter(&self, span: &span::Id) {
        dbg!("Enter:", span);
    }

    fn exit(&self, span: &span::Id) {
        dbg!("Exit:", span);
    }
}

fn main() {
    tracing::subscriber::set_global_default(MySubscriber::new()).expect("Init should work");

    info!("Hey");

    {
        let _span = info_span!("I'm the spanman").entered();
        info!(bing = 123, "Huh");
    }

    let _span = info_span!("I'm the spanman v2").entered();
    let _span = info_span!("I'm the spanman v3").entered();
    info!(bing = 123, "Huh v2, v3");
}
