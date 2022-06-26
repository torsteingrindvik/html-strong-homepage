use ::tracing::{info, instrument};
use tracing::{debug, info_span};

// mod bing {}
// use bing as tracing;

mod modulius {
    use super::*;
    #[instrument(
	fields(well = num * 2, gotchu = num as f32 / 0.01, upper = thing.to_uppercase().as_str(), huh),
	// name = "baz",
	skip(num),
	// target = "🐌",
	level = "info",
	parent = None,
	ret
)]
    pub fn foo(num: usize, thing: &str) -> usize {
        tracing::Span::current().record("huh", &500);
        tracing::Span::current().record("huh-v2", &501);

        info!("How are ya");
        debug!("How debug are ya");

        num * 123
    }
}

// fn bar(num: usize, thing: &str) {
//     let __tracing_attr_span;
//     let __tracing_attr_guard;

//     if tracing::level_enabled!(tracing::Level::INFO) {
//         __tracing_attr_span = tracing::span!(
//             target: module_path!(),
//             tracing::Level::INFO,
//             "bar",
//             num = num,
//             thing = thing,
//         );
//         __tracing_attr_guard = __tracing_attr_span.enter();
//         info!("How are ya, but v2");
//     }
// }

fn main() {
    tracing_subscriber::fmt().pretty().init();

    let _span_guard_parent = info_span!("🤔").entered();
    let _span_guard = info_span!("🚌").entered();
    modulius::foo(123, "Thing!");
}
