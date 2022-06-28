use std::collections::HashMap;

use crate::{components::Article, listing::Source};

pub fn intro() -> Article {
    let code: HashMap<&'static str, Source> = [
        (
            "threads",
            Source::new("code/tracing-explore/src/bin/threads.rs"),
        ),
        (
            "log-macros",
            Source::new("code/tracing-explore/src/bin/log-macros.rs"),
        ),
        (
            "log-macros-expanded",
            Source::new("code/tracing-explore/log-macros-expanded.rs"),
        ),
    ]
    .into_iter()
    .collect();

    Article::new()
        .h2("Tracing: An overview")
        .p("Tokio ")
        .url("https://github.com/tokio-rs/tracing", "tracing")
        .p(
            " looks really neat. I've sporadically used it for various small projects, with \
             various degrees of success. Most times I just use it to get the various standard log \
             macros, and rely on ",
        )
        .shell("RUST_LOG=<some log level>")
        .p(" to make things show up in my terminal.")
        .p("But there seems to be a lot more to the story!")
        .p(
            "Let's lay out some goals if what to figure out. Note that at the time of writing, I \
             don't know the answers, and as such the goals might not be accurate until I know \
             enough to refine them.",
        )
        .h2("Goals")
        .p(
            "There are several goals I'd like to achieve by investigating tracing. The goals can \
             be split into a few categories, so let's do that.",
        )
        .h3("Goal: Understanding tracing itself")
        .p("I'd like to understand the concepts better.")
        .list(vec![
            "What is a subscriber?",
            "Can I use many subscribers at the same time? Should I?",
            "How does writing my own subscriber look like?",
            "What is an event?",
            "What is a span?",
            "How can the act of entering a span impact code in the same scope? In other words, \
             how can one part of code introduce spooky state on following code?",
            "Why do we have to be careful with spans when using async?",
            "What does the instrument attribute do, and what does the instrument call on a future \
             do?",
            "How is global state managed? Specifically, how does logging interact with a global \
             subscriber?",
        ])
        .h3("Practical goal: Making distributed traces")
        .p(
            "I want to know how I can make an application where I have some servers with clients \
             talking to each server, and end up with traces which are scoped in such a way that \
             it's easy to see what happened during each connection.",
        )
        .p(
            "Also, let's say we have 10 000 of the above sessions. Can I have little/no verbosity \
             for happy connections, and higher verbosity if an error occurs in misbehaving \
             connections?",
        )
        .h3("Goal: The relationship with OpenTelemetry")
        .p("For having this neat overview of distributed traces, I think ")
        .url("https://opentelemetry.io/docs/", "OpenTelemetry")
        .p(
            " is a good way to go about things. There seems to be some nice integrations between \
             tracing and OpenTelemetry. But OpenTelemetry too has concepts which I know little \
             of. So...",
        )
        .list(vec![
            "How does OpenTelemetry work?",
            "How well does it map to tracing?",
            "Which collectors should I use?",
            "Is a collector what I think it is: The thing which receives traces? Is it very close \
             to being a (tracing) subscriber?",
        ])
        .h2("Understanding tracing")
        .h3("The log macros")
        .p("Let's start somewhere.")
        .p("As a user of tracing, I'll be using the various log macros regularly. What do these do?")
        .code(code["log-macros"].listing(1))
        .p("There is a nice way to see exactly what it is without trying to understand the macro expansion by the source code.")
        .p("Enter ")
        .url("https://github.com/dtolnay/cargo-expand", "cargo-expand")
        .p(". Running it without nightly produced compile errors due to use of unstable flags.")
        .p("So first order is to ")
        .shell("rustup install nightly")
        .p(" then do (from ")
        .url("https://github.com/torsteingrindvik/html-strong-homepage/tree/main/code/tracing-explore", "this")
        .p( " folder) ")
        .shell("cargo +nightly expand --bin log-macros > log-macros-expanded.rs")
        .p(".")
        .p("So now we have an expanded source file. First of all we have some stuff that's there in all normal Rust binaries:")
        .code(code["log-macros-expanded"].listing(1))
        .p("And now, get ready for it, here's the expansion of our log call:")
        .code(code["log-macros-expanded"].listing(2))
        .p("That's a lot of stuff!")
        .p("So there are callsites, metadata, field sets, static levels, interests, value sets, and dispatching.. at least. Let's just dive into it.")
        .h3("Callsite")
        .p("Let's just try to understand stuff as it appears.")
        .p("The first thing is this:")
        .code(code["log-macros-expanded"].listing(3))
        // TODO: Better visibility of 1: somehow?
        .p("1: Here we see a use declaration. When we write one of these, we can give it some name.")
        .p("But why would we give it the name ")
        .shell("_")
        .p("?")
        .p("Turns out, rustlang has an ")
        .url("https://github.com/rust-lang/rfcs/blob/master/text/2166-impl-only-use.md", "RFC for this")
        .p(".")
        .p("The answer is simply that we can bring a trait into scope, and also use the same name as the trait for other things.")
        .p("Which is then probably why we can use ")
        .shell("CALLSITE")
        .p(" as a symbol name.")
        .br()
        .p("2: Here we see ")
        .shell("tracing::<stuff>")
        .p(" instead of ")
        .shell("::tracing::<stuff>")
        .p(" which can cause issues if there is something else called ")
        .shell("tracing")
        .p(" locally. The other ")
        .shell("use")
        .p(" declarations following has these leading double colons, so this one was probably just missed.")
        .br()
        .p("Anyway. The ")
        .url("https://docs.rs/tracing-core/latest/tracing_core/trait.Callsite.html", "trait")
        .p(" is defined like so:")
        .code(code["log-macros-expanded"].listing(4))
        .p("We see the use of ")
        .shell("metadata()")
        .p(" later on, so at least that makes sense now.")
        .p("We will get back to what a callsite actually is, but first let's check out the next part:")
        .code(code["log-macros-expanded"].listing(5))
        .p("So in the above we create a static instance of a callsite. It's interesting that we create it with a statically lifetimed reference to metadata when that data is defined right above it, because trying to \"think in lifetimes\" in Rust has taught me that most static data is either owned or global. But declaring it static in some local scope should work too, so it's nice seeing an example of it.")
        .br()
        .shell("DefaultCallsite::new(...)")
        .p("wants a static reference to metadata. But it's not obvious why the callsite itself needs to be static. So why?")
        .p("Looking at the struct ")
        .url("https://docs.rs/tracing-core/latest/tracing_core/callsite/struct.DefaultCallsite.html", "definition")
        .p(" the methods require a static shared reference to self. It uses it when registering with the global callsite registry, which we at this point know nothing about. But it feels like it could make sense, which is good enough right now.")
        .br()
        .p("Helpfully ")
        .shell(" DefaultCallsite")
        .p("'s definition page links us to some ")
        .url("https://docs.rs/tracing-core/latest/tracing_core/callsite/index.html#", "great docs")
        .p(" about what callsites actually are. I'll summarize it.")
        // TODO: More space around lists
        .list(vec![
            "Callsites represent locations where spans/events originate (we will look more into both)",
            "A span/event is made unique since its callsite is unique",
            "Callsites are added to a global registry. When an event/span is used for the first time, the registry is updated, and subscribers are notified (more on subscribers later!) (also note that we say subscribers plural here. One of our goals was to figure out if that is allowed, so yay!)",
            "Subscribers look at the new callsite and indicate whether they care about it or not, which may allow optimizations",
        ])
        .p("There's a lot of text and information right now. Let's look at one more thing, the metadata initializer, then we'll create a graph with our understanding so far.")
        .code(code["log-macros-expanded"].listing(6))
        .p("Looking at ")
        .url("https://docs.rs/tracing-core/latest/tracing_core/struct.Metadata.html", "the docs")
        .p(" for metadata, it mostly makes sense:")
        .list(vec![
            "The name is the metadata's name",
            "The target I couldn't precisely guess. It defaults to the module path, can be overridden, and is used to \"categorize part of the system where the span or event occurred\"",
            "The level is used for level filtering",
            "The file is where the metadata was created",
            "The line is which line in the file the metadata was created",
            "The module is which module... you get it",
            "The fields are interesting and deserve a closer look later",
            "The kind indicates if we are dealing with an event or a span. It can also be \"hint\", which I will ignore unless it appears at some later point. But note that we used a log macro and that turned into an event"
        ])
        .p("Okay! Cool! What do we now know?")
        .list(vec![
            "When we use a log macro, it expands into lots of stuff",
            "One thing is a static callsite",
            "The callsite uniquely identifies where \"the thing\" happened",
            "\"The thing\" is an event",
            "There is some global registry of these callsites",
            "Subscribers are somehow notified of callsites",
            "Subscribers can say if they care about the callsite or not",
            "Even though we haven't seen it yet, we can only assume subscribers get a look at the event when it occurs"
        ])
        .p("Let's try drawing our understanding thus far.")
}
