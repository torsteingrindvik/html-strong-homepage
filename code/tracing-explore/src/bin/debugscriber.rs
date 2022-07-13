use tracing::{info, span, Metadata, Subscriber};

// listing: Skeleton impl
struct DbgScriberTodo;

impl Subscriber for DbgScriberTodo {
    fn enabled(&self, metadata: &Metadata<'_>) -> bool {
        todo!()
    }

    fn new_span(&self, span: &span::Attributes<'_>) -> span::Id {
        todo!()
    }

    fn record(&self, span: &span::Id, values: &span::Record<'_>) {
        todo!()
    }

    fn record_follows_from(&self, span: &span::Id, follows: &span::Id) {
        todo!()
    }

    fn event(&self, event: &tracing::Event<'_>) {
        todo!()
    }

    fn enter(&self, span: &span::Id) {
        todo!()
    }

    fn exit(&self, span: &span::Id) {
        todo!()
    }
}
// ~listing

// listing: Skeleton impl use
fn main() {
    tracing::subscriber::set_global_default(DbgScriberTodo).expect("init should work");

    info!("Hi!");
}
// ~listing
