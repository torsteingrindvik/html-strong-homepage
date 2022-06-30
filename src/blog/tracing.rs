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
        (
            "tidbits",
            Source::new("code/tracing-explore/src/bin/tidbits.rs"),
        ),
        (
            "log-macros-fields-values",
            Source::new("code/tracing-explore/src/bin/log-macros-fields-values.rs"),
        ),
        (
            "log-macros-fields-values-expanded",
            Source::new("code/tracing-explore/log-macros-fields-values-expanded.rs"),
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
        .h3("[This blogpost] Goal: Understanding tracing itself")
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
        .h3("[Future blogpost] Practical goal: Having some fun with tracing")
        .p("To get to grips with tracing, let's make a subscriber which does something silly.")
        .p("Pointless small projects are my favorite type of project.")
        .p("It would be fun to do something visual- let's not spoil too much yet.")
        .h3("[Future blogpost] Practical goal: Making distributed traces")
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
        .h3("[Future blogpost] Goal: The relationship with OpenTelemetry")
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
        .p("So there are callsites, metadata, field sets, static levels, interests, value sets, and dispatching.. at least. Let's just dive into it piece by piece.")
        .h3("Callsite")
        .p("Let's just try to understand stuff as it appears.")
        .p("The first thing is this:")
        .code(code["log-macros-expanded"].listing(3))
        // TODO: Better visibility of 1: somehow?
        .p("1: Here we see a ")
        .shell("use")
        .p(" declaration. When we write one of these, we can give it some name.")
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
        .p(" later on, so needing to bring the trait into scope makes sense.")
        .p("We will get back to what a callsite actually is, but first let's check out the next part:")
        .code(code["log-macros-expanded"].listing(5))
        .p("So in the above we create a static instance of a callsite. It's interesting that we create it with a statically lifetimed reference to metadata when that data is defined right above it, because trying to \"think in lifetimes\" in Rust has taught me that most static data is either owned or global. But declaring it static in some local scope should work too, so it's nice seeing an example of it.")
        .p("Oh an perhaps I should edit the listings to not have qualified paths:")
        .code(code["log-macros-expanded"].listing(6))
        .p("For me code instantly gets less spooky when presented like that. So I'll edit future listings to have unqualified paths.")
        .br()
        .shell("DefaultCallsite::new(...)")
        .p("wants a static reference to metadata. But it's not obvious (to me) why the callsite itself needs to be static. So why?")
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
        .p("Lots of concepts going on right now! Let's look at one more thing, the metadata initializer, then we'll create a graph with our understanding so far to break things up.")
        .code(code["log-macros-expanded"].listing(7))
        .p("Looking at ")
        .url("https://docs.rs/tracing-core/latest/tracing_core/struct.Metadata.html", "the docs")
        .p(" for metadata, it mostly makes sense:")
        .list(vec![
            "The name is the metadata's name",
            "The target I couldn't precisely guess. It defaults to the module path, can be overridden, and is used to \"categorize part of the system where the span or event occurred\"",
            "The level is used for level filtering",
            "The file, line, module pinpoints where the metadata was created",
            "The fields are interesting and deserve a closer look later. Short version is that variables passed to events are collected here.",
            "The kind indicates if we are dealing with an event or a span. It can also be \"hint\", which I will ignore unless it appears at some later point. But note that we used a log macro and that turned into an event kind."
        ])
        .br()
        .p("Okay! Cool! Let's take a step back and think about what we now know.")
        .list(vec![
            "When we use a log macro, it expands into lots of stuff",
            "One thing is a static callsite",
            "The callsite uniquely identifies where \"the thing\" happened",
            "\"The thing\" is an event",
            "There is some global registry of these callsites",
            "Subscribers are somehow notified of callsites",
            "Subscribers can say if they care about the callsite or not",
            "Each callsite stores the information whether anyone (subscribers) might care (😢) about them (optimization)",
            "Even though we haven't seen it yet, we can only assume subscribers get a look at the event when it occurs"
        ])
        .p("Let's draw our current assumptions. I use ")
        .url("https://excalidraw.com/", "Excalidraw")
        .p(" by the way.")
        .image("tracing-01.png")
        .p("So, an event comes from a callsite. The global registry checks if subscribers want to get events from that callsite. If they do, the event is passed on.")
        .p("Note that at this point some details are likely to be a bit off target but things are clearing up so it's all good.")
        .h3("Enabled?")
        .p("We learned a lot from looking at the callsite initialization and seeing where we ended up.")
        .p("Let's move a bit forward in the log macro expanded code, and check out this:")
        .code(code["log-macros-expanded"].listing(8))
        .p("So since we used the ")
        .shell("info!(...)")
        .p(" macro we see ")
        .shell("INFO")
        .p(" sprinkled here a couple times. And it's being compared against twice. What an oversight right? No- of course there's a good reason, and the hint is right there in the ")
        .shell("STATIC_MAX_LEVEL")
        .p(" name. This is pretty neat, in the feature flags in tracing you can set e.g. ")
        .shell("features = [\"max_level_warn\"]")
        .p(" and that will change the constant ")
        .shell("STATIC_MAX_LEVEL")
        .p(" to ")
        .shell("LevelFilter::WARN")
        .p(" and then we short-circuit the rest of the listing we're looking and nothing more needs to happen. And that should go for every use of the log macros for your whole project.")
        .br()
        .p("But wait.")
        .p("Hm, this feature flag is passed to ")
        .shell("tracing")
        .p(", and as a user you're more likely to use something like ")
        .shell("tracing-subscriber")
        .p(". So if a library you depend on uses tracing, can you add tracing itself to your own dependencies with the feature flag, and that gets propagated to dependencies?")
        .p("I think it should, but TODO check this out")
        .br()
        .p("What is ")
        .shell("LevelFilter::current()")
        .p(" then?")
        // TODO: Quote
        .p("\"Returns a LevelFilter that matches the most verbose Level that any currently active Subscriber will enable.\"")
        .p("Ah, so if you have 10 subscribers at the warning level and 1 subscriber at debug, the most verbose level is debug, so that gets returned.")
        .p("Ok, so if we then used a ")
        .shell("trace!(...)")
        .p(" call here, then that could short circuit here and do nothing.")
        .br()
        .p("Then we use the static callsite struct and call the ")
        .shell("interest()")
        .p("method. What does this do then? It returns one of ")
        .shell("Interest::{never()/always()/sometimes()}")
        .p(". But the callsite was created a few lines ago, and we (the macro) did not pass any ")
        .shell("Interest")
        .p(" to it. I peeked at the constructor, it sets it internally to something invalid. So what will it return then?")
        .p("Peeking at the ")
        .shell("interest()")
        .url("https://docs.rs/tracing-core/latest/src/tracing_core/callsite.rs.html#342-349", "source")
        .p(" shows that it goes through registration if it's in this invalid state. Cool, now we know how that is hooked up.")
        .p("And what's the TL;DR of the registration?")
        .list(vec![
            "Do some atomics juggling to ensure that registration only happens once",
            "Go through each subscriber (called dispatcher instead in the source?) and tell them about this new callsite via giving them the metadata",
            "Figure out if any subscriber might have an interest in this callsite, and store that information in the callsite",
            "Put the callsite in the global registry"
        ])
        .p("Soon done! If any subscriber might be interested (i.e. not never interested), we have another step to do.")
        .p("Lots of double underscores, so this is some very internal thing. We call ")
        .shell("__is_enabled(CALLSITE.metadata(), interest)")
        .p( ".")
        .p("Checking the ")
        .url("https://github.com/tokio-rs/tracing/blob/tracing-0.1.35/tracing/src/lib.rs#L988", "source")
        .p(" the new thing here is that the default dispatcher is asked whether this is enabled, via the metadata.")
        .p("So, is dispatcher just another name for subscriber?")
        .p("It seems, ")
        .url("https://github.com/tokio-rs/tracing/blob/tracing-0.1.35/tracing/src/dispatcher.rs#L1-L23", "not quite")
        .p(". The relevant things to know is that a dispatcher is a handle for a subscriber, which can be passed around, cloned, and is type erased. Its job is to forward trace data to the subscriber.")
        .p("Checking that ")
        .url("https://github.com/tokio-rs/tracing/blob/tracing-0.1.35/tracing-core/src/dispatcher.rs#L148-L153", "shows")
        .p(" that indeed, a dispatch wraps a subscriber using ")
        .shell("Arc")
        .p(" using dynamic dispatch.")
        .p("Great, we finally know about all the boxed that are ticked before anything happens!")
        .p("Let's visualize it.")
        .image("tracing-02.png")
        .p("I am a bit confused now.")
        .p("In the second step, we check if there are anyone that might be interested at all. If so, then we go on to check if the current default subscriber is interested. What if a non-default subscriber is interested? Is this even a valid question? Hopefully this will make sense later on.")
        .br()
        .p("We can now jump into what happens when the callsite is enabled.")
        .code(code["log-macros-expanded"].listing(9))
        .p("Let's look at the fancy block first. Is it fancy? I don't think I have written anything like this myself yet.")
        .p("So it looks like it defines a closure, which is normally bound to a variable.")
        .p("But instead of binding it to a variable, it's called right away in the connecting block. So, the latter block has a ")
        .shell("ValueSet")
        .p( " type that falls out at the end, which is fed into the fancy block. So I suppose that it wasn't as fancy as it looked at first, I've just not seen it.")
        .p("In case you're unfamiliar as well, here's another example that might clear it up:")
        .code(code["tidbits"].listing("Unfancy"))
        .p("Then the same but in the style we saw in the macro:")
        .code(code["tidbits"].listing("Fancy"))
        .p("There are a lot of parentheses, but it's necessary to separate which part is the closure and which part is the stuff fed into it.")
        .br()
        .p("Ok, back to. The closure is:")
        .code(code["log-macros-expanded"].listing("Closure called"))
        .p("We finally see where events are sent! Nice. So the event carries the metadata, and something called a ")
        .shell("ValueSet")
        .p(", which is what?")
        .p("Its definition is")
        .code(code["tidbits"].listing("ValueSet def"))
        .p("Quite a lot going on there. So values are a slice of tuples, where each tuple is a reference to a field, and maybe a value. And fields is a reference to another thing called a ")
        .shell("FieldSet")
        .p(".")
        .p("These fields and values are definitely related to user input to events (and spans?). So maybe we should do something more fancy in a log macro and expand that instead.")
        .p("This is our new thing to expand:")
        .code(code["log-macros-fields-values"].listing("info!()"))
        .p("We now attach data to the log call, which is then somehow propagated to the subscriber in the end.")
        .p("Notice that the log macro can't know how to show our struct unless we tell it how, and ")
        .shell("?")
        .p(" opts in to do it via ")
        .shell("Debug")
        .p(".")
        .p("When we expand it, a few things change. First, the metadata of the callsite:")
        .code(code["log-macros-fields-values-expanded"].listing("Metadata with field set"))
        .p("We now notice the field set, and we understand it better. \"message\" is there like before, and is related to the message we give the macro. The rest are the names of the new data we passed.")
        .p("Also note there is an identifier, which is unique per callsite (according to docs).")

    // .code(code["log-macros-expanded"].listing("Closure input"))
}
