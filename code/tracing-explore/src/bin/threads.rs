use tracing::Id;
use tracing_subscriber::{prelude::*, Registry};
use tracing_tree::HierarchicalLayer;

// listing 1: the function foo
fn foo(span_id: Option<Id>, num: usize) {
    let span = info_span!(parent: span_id, "thread", me = num);
    let _guard = span.entered();

    info!("Hey");

    // What if there was more stuff here
    // Or something else
    let haha = 123;
}
// ~listing

// listing 2: main
fn main() {
    // Did not really work!
    // Single threaded this makes sense, else not really
    let subscriber = Registry::default().with(HierarchicalLayer::new(2));
    tracing::subscriber::set_global_default(subscriber).expect("global default can be set");

    // Let's make one unnamed thread.
    let mut handles = vec![thread::spawn(|| foo(None, 0))];

    let main = info_span!("main");

    for (index, &tid) in ["🚘", "🎠", "🌄"].iter().enumerate() {
        let id = main.id().clone();

        handles.push(
            thread::Builder::new()
                .name(tid.into())
                .spawn(move || foo(id, index))
                .expect("thread should be spawned without issue"),
        );
    }

    for handle in handles {
        handle.join().expect("join should work")
    }
}
// ~listing
