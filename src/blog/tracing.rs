use crate::{components::Article, listing::Source};

pub fn intro() -> Article {
    let source = Source::new("code/tracing-explore/src/bin/threads.rs");

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
        .code(source.listing(1))
        .code(source.listing("main"))
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
}
