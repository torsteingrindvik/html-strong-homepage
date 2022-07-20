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
        ("span", Source::new("code/tracing-explore/src/bin/span.rs")),
        (
            "span-expanded",
            Source::new("code/tracing-explore/span-expanded.rs"),
        ),
        (
            "async-span-problem",
            Source::new("code/tracing-explore/src/bin/async-span-problem.rs"),
        ),
        (
            "async-span-instrument",
            Source::new("code/tracing-explore/src/bin/async-span-instrument.rs"),
        ),
        (
            "async-span-instrument-proc-macro",
            Source::new("code/tracing-explore/src/bin/async-span-instrument-proc-macro.rs"),
        ),
        (
            "async-span-instrument-proc-macro-expanded",
            Source::new("code/tracing-explore/async-span-instrument-proc-macro-expanded.rs"),
        ),
        (
            "debugscriber",
            Source::new("code/tracing-explore/src/bin/debugscriber.rs"),
        ),
        (
            "debugscriber2",
            Source::new("code/tracing-explore/src/bin/debugscriber2.rs"),
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
        .sidenote(Article::new().h3("[Future blogpost] Practical goal: Having some fun with tracing")
            .p("To get to grips with tracing, let's make a subscriber which does something silly.")
            .p("Pointless small projects are my favorite type of project.")
            .p("It would be fun to do something visual- let's not spoil too much yet.")
        )
        .sidenote(Article::new()
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
        )
        .sidenote(Article::new().h3("[Future blogpost] Goal: The relationship with OpenTelemetry")
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
        )
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
        .code_inline("tracing::<stuff>")
        .p(" instead of ")
        .code_inline("::tracing::<stuff>")
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
        .code_inline("DefaultCallsite::new(...)")
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
        .image("tracing-01.webp")
        .p("So, an event comes from a callsite. The global registry checks if subscribers want to get events from that callsite. If they do, the event is passed on.")
        .p("Note that at this point some details are likely to be a bit off target but things are clearing up so it's all good.")
        .h3("Enabled?")
        .p("We learned a lot from looking at the callsite initialization and seeing where we ended up.")
        .p("Let's move a bit forward in the log macro expanded code, and check out this:")
        .code(code["log-macros-expanded"].listing(8))
        .p("So since we used the ")
        .code_inline("info!(...)")
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
        .code_inline("LevelFilter::current()")
        .p(" then?")
        .quote(Article::new().p("Returns a LevelFilter that matches the most verbose Level that any currently active Subscriber will enable."))
        .p("Ah, so if you have 10 subscribers at the warning level and 1 subscriber at debug, the most verbose level is debug, so that gets returned.")
        .p("Ok, so if we then used a ")
        .code_inline("trace!(...)")
        .p(" call here, then that could short circuit here and do nothing.")
        .br()
        .p("Then we use the static callsite struct and call the ")
        .code_inline("interest()")
        .p(" method. What does this do then? It returns one of ")
        .code_inline("Interest::{never()/always()/sometimes()}")
        .p(". But the callsite was created a few lines ago, and we (the macro) did not pass any ")
        .shell("Interest")
        .p(" to it. I peeked at the constructor, it sets it internally to something invalid. So what will it return then?")
        .p("Peeking at the ")
        .code_inline("interest()")
        .p(" ")
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
        .code_inline("__is_enabled(CALLSITE.metadata(), interest)")
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
        .image("tracing-02.webp")
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
        .br()
        .p("So then the dispatching changes to add the values:")
        .code(code["log-macros-fields-values-expanded"].listing("Dispatch event with fields, values"))
        .p("We see here that things which are a ")
        .shell("Value")
        .p(" can be used for events. So which types are these?")
        .p("The ")
        .url("https://docs.rs/tracing/0.1.35/tracing/trait.Value.html", "docs")
        .p(" show that most primitive types like ")
        .shell("u8, bool, str")
        .p(" can be used. Wait, ")
        .shell("str")
        .p("? Not the borrowed form? Well there is also")
        .code(code["tidbits"].listing("str to &str"))
        .p("Here ")
        .shell("T")
        .p(" is ")
        .shell("str")
        .p(", which is unsized, or ")
        .shell("?Sized")
        .p(", so I believe this blanket implementation makes it so we can use the normal borrowed string slice.")
        .br()
        .code(code["tidbits"].listing("Other things which are Values"))
        .p("This is neat. So if we have some code which handles an error, it can be logged directly as a field.")
        .br()
        .p("The ")
        .shell("Arguments")
        .p(" impl explains the ")
        .code_inline("Some(&::core::fmt::Arguments::new_v1(&[\"Hey\"], &[]) as &Value)")
        .p(", which the macro produced. Does it though? ")
        .shell("T::new_v1")
        .p(" is a weird thing to have exposed to users, which might explain why it ")
        .url("https://doc.rust-lang.org/beta/src/core/fmt/mod.rs.html#389", "isn't")
        .p(" (not sure how long this link will point to the correct thing, search for ")
        .shell("new_v1")
        .p("). It is marked as an unstable internal function with hidden docs.")
        .p("Also it's behind a feature flag ")
        .shell("fmt_internals")
        .p(", so then how is it used? This confused me, and what's worse I couldn't find any use of ")
        .shell("new_v1")
        .p(" in tracing. Then I realized the expanded macro we're seeing has expanded ")
        .code_inline("format_args!(...)")
        .p(" which IS used in tracing. So macro expansions might use macros as well, which are also expanded. It's macros all way down.")
        .br()
        .p("And the struct? Well, we saw ")
        .code_inline("Some(&debug(&MyStruct { _v: 10 }) as &Value)")
        .p(", and that produces a ")
        .shell("DebugValue")
        .p(", and that can be used.")
        .br()
        .p("So all in all, there are fields and values, and these are matched up and forwarded to the default subscriber. Let's move on.")
        .h2("Spans")
        .p("My understanding is that a span represents a span of time, and when events happen at some instant of time, the event happens in the context of some span- so they form a relationship. Spans can also nest, and as such they have parents. Spans can go in and out of scope, which then \"pauses\" them.")
        .p("I will re-read the ")
        .url("https://docs.rs/tracing/0.1.35/tracing/index.html#spans", "docs")
        .p(" and see if I forgot some important things.")
        .p("There were a few things. Spans contain our friends ")
        .shell("Metadata")
        .p(" and ")
        .shell("Field")
        .p(". This mean we can pinpoint where spans are located in code, give them names, as well as attach user defined data to them. Neat.")
        .br()
        .p("Looking at the ")
        .url("https://docs.rs/tracing/0.1.35/src/tracing/span.rs.html#1-1617", "source")
        .p(" I see that spans have an ")
        .shell("Id")
        .p(", which can uniquely identify a span, nothing surprising there.")
        .p("Spans also have a reference to the current default subscriber, via a ")
        .shell("Dispatch")
        .p(", which we learned is a handle for a subscriber, so that falls into place, nice.")
        .br()
        .p("Other noteworthy parts is that when a span is \"entered\", which I take to mean that user code now resides in this span's context, the subscriber is notified about this. Entering a span gives the user a guard, which when dropped, notifies the subscriber about it.")
        .p("We will look more at these events when we look at subscribers specifically.")
        .h3("Create a span and ex(s)pand it!")
        .p("Let's do a similar exercise as we did with one the log macros. Take this:")
        .code(code["span"].listing("Some span"))
        .p("And we feed it into the expand machinery again, and it spits out this (lightly edited for readbility again plus some comments):")
        .code(code["span-expanded"].listing("Some span"))
        .p("This is so similar to the log macro, since creating a callsite and checking for potential interest is the same.")
        .p("So the most interesting part then is:")
        .code(code["span-expanded"].listing("Span creation"))
        .p("Not much new here either, since we already know about ")
        .shell("ValueSet")
        .p(". So in summary a span is created, given metadata, and given fields and values.")
        .p("Noteworthy: Nothing really happens yet, since we don't enter the span. In other words, no subscriber knows about this yet.")
        .h3("Async: Spans are problematic?")
        .p("There is a warning in the documentation about entering a span and then using await after. Why would this be an issue?")
        .p("The docs explain that the issue is that when we ")
        .shell(".await")
        .p(" the execution likely resumes elsewhere, but the variables in scope are not dropped. Because when we return back later, we still need to be able to use those variables. That makes sense.")
        .p("And based on what we know about spans and subscribers, we see the problem. We know that the mechanism for keeping track of spans is to inform the subscriber when entered and when dropped. Async creates a situation which this doesn't happen as expected.")
        .h3("Async: Can we see the problem by expanding?")
        .p("I naively believed I could just expand something like this:")
        .code(code["async-span-problem"].listing("Some span"))
        .p("And we'd get some obvious super way to just see our problem. But cargo expand does not touch async.")
        .p("Perhaps I should have guessed that? Expanding is for macros, and ")
        .shell("async")
        .p(" is a different beast. So I will leave this be for now.")
        .h3("Async: Solution candidate #1")
        .p("The ")
        .url("https://docs.rs/tracing/0.1.35/tracing/struct.Span.html#in-asynchronous-code", "docs")
        .p(" tells us what to do in async code to make it work as we want.")
        .br()
        .p("It looks like this:")
        .code(code["async-span-instrument"].listing("instrumented"))
        .p("Ok, that isn't that bad. Any place we would normally write ")
        .shell(".await")
        .p(" we can put ")
        .code_inline(".instrument(<some span>)")
        .p(" in front and carry on with our day.")
        .p("What does it do though? The definition is so:")
        .code(code["async-span-instrument"].listing("Instrument trait"))
        .p("So ")
        .shell("instrument")
        .p("is part of a trait, which is why you need to fetch that into scope if you ever use the method. What is ")
        .shell("self")
        .p(" here though? We only passed one argument, the debug span.")
        .p("The way this works is pretty cool- it's an extension trait. This type of trait allows you to make methods on you did not create yourself. Hence extension.")
        .p("So we're meant to use this on any async function (any ")
        .shell("Future")
        .p("). This explains it:")
        .code(code["async-span-instrument"].listing("Instrument blanket"))
        .p("There isn't any restriction to only async functions here though? So this is perfectly fine:")
        .code(code["async-span-instrument"].listing("Instrument non-future"))
        .p("It isn't that awkward. It just doesn't do anything. It only does useful (i.e. tracing related) work when used as a future. See:")
        .code(code["async-span-instrument"].listing("Instrumented future impl"))
        .p("This is a super-thin wrapper of the future the type ")
        .shell("T")
        .p(" already implements, but it enters a span. And since there are no ")
        .shell(".await")
        .p(" usages in a ")
        .shell("Future")
        .p(" impl, the old rules apply. Meaning:")
        .list(vec![
            "We enter the span, the current subscriber is told about it",
            "The wrapped future is polled, which might enter more spans and/or fire events",
            "Spans drop in the natural order of normal non-async code, and subscribers are told about this too"
        ])
        .p("Great, that made sense (hopefully to you too!).")
        .br()
        .p("One thing I skipped. There was a ")
        .shell("Sized")
        .p(" constraint on the ")
        .shell("Instrument")
        .p(" trait. Why? My understanding is that the ")
        .shell("self")
        .p(" parameter in the ")
        .shell("instrument")
        .p(" method imposes this:")
        .code(code["async-span-instrument"].listing("Instrument method"))
        .p("If we don't, the compiler helpfully notes:")
        .br()
        .quote(Article::new().p("the size for values of type Self cannot be known at compilation time"))
        .br()
        .p("Since the trait needs to work for any ")
        .shell("Self")
        .p(" type, that includes things which we do not know the size of at compile time (assuming we don't have the ")
        .shell("Sized")
        .p(" constraint now).")
        .p("And what's the problem with that? If you have a function that does ")
        .code_inline("let i_am_small_and_big_and_both_oh_no = some_type.instrument();")
        .p(" the compiler can't know how big the returned type is, and then we can't create a properly sized stack for this function either.")
        .h3("Async: Solution candidate #2")
        .p("The other candidate is to use a proc macro:")
        .code(code["async-span-instrument-proc-macro"].listing("Instrument proc macro"))
        .p("Since we are back to using proc macros we can use the nice feature in rust-analyer to expand it for us.")
        .p("The result is thus:")
        .code(code["async-span-instrument-proc-macro-expanded"].listing("Instrument proc macro"))
        .p("We didn't have to but we passed ")
        .shell("foodie")
        .p(" to the proc macro to see the effect. It's unsurprisingly passed on to the span creation macro. The docs list a number of similar things we can do to affect the other arguments passed to the span being created.")
        .br()
        .p("This solution in effect is close to the same as the previous solution, since ")
        .code_inline("instrument()")
        .p(" is used. There is also an optimization done with ")
        .code_inline(".is_disabled()")
        .p(", which helps skip doing work when e.g. the level is more verbose than what is enabled.")
        .br()
        .p("What should you use, then? Whichever you want. I personally tend to prefer non-macro solution whenever I can, as they feel less magic to me.")
        .h2("Subscribers")
        .p("There is one thing left to look at in this blogpost- subscribers.")
        .br()
        .p("We have picked up some information about them along the way already:")
        .list(vec![
            "Tracing has a way to access the globally set default subscriber",
            "A dispatch is a handle to a subscriber",
            "They receive events from log macros",
            "They get notified when spans are entered",
            "They get notified when the entered-span-guard is dropped",
        ])
        .br()
        .p("The remaining questions I want answers to:")
        .list(vec![
            "There is only one active subscriber at a time (I think!). So how do we e.g. log to a combination of file, stdout, stderr, and others with one subscriber?",
            "What other responsibilities does the subscriber have?",
            "How does a simple subscriber implementation look like?"
        ])
        .h3("A simple subscriber")
        .p("I think we'll start by just making a struct and letting rust-analyzer add a skeleton implementation for us for the ")
        .url("https://docs.rs/tracing/0.1.35/tracing/trait.Subscriber.html", "Subscriber")
        .p(" trait.")
        .br()
        .p("Rust-analyzer helps us out by making a skeleton impl:")
        .code(code["debugscriber"].listing("Skeleton impl"))
        .p("We will fill these in soon. Using this is done like so:")
        .code(code["debugscriber"].listing("Skeleton impl use"))
        .p("But since we have ")
        .code_inline("todo!()")
        .p("s everywhere it will crash. Can we predict in which function?")
        .p("We expect " )
        .shell("event")
        .p(" to be called at one point. But thinking back to the log macro expansion, there are checks in place to see if the event should be dispatched at all or no, based on the metadata.")
        .p("So we expect ")
        .shell("enabled")
        .p(" to panic the program.")
        .br()
        .p("Running this leads to:")
        .quote(Article::new().p("thread 'main' panicked at 'not yet implemented', src/bin/debugscriber.rs:8:9") .p("note: run with RUST_BACKTRACE=1 environment variable to display a backtrace"))
        .p("Checking that line it panicked where we predicted. Let's enable the backtrace to see if we can learn anything by it.")
        .br()
        .p("Instead of dumping the backtrace, here's a summary, which solidifies our earlier investigation into the log macros.")
        .list(vec![
            "Before passing the event to the default subscriber, the tracing machinery asks if anyone is interested",
            "Interest is normally cached, but this is the first time this event has been issued, so..",
            "..register this callsite for the first time..",
            "..by asking the default subscriber (ours!) if this callsite should be enabled..",
            "..which leads to the panic."
        ])
        .br()
        .p("Now, let's try making a simple subscriber.")
        .code(code["debugscriber2"].listing("Impl which just prints things"))
        .p("We'll talk about all the functions. First, let's run it via this:")
        .code(code["debugscriber2"].listing("Printing subscriber use"))
        .p("Running that gives us:")
        .shell_multiline("code/tracing-explore/src/bin/debugscriber2.txt")
        .p("A few things to notice here.")
        .list(vec![
            "Checking if a callsite is enabled is cached, as it does not happen the second loop iteration",
            "new_span happens when a span is created, even though we don't ever enter (and exit) it",
        ])
        .br()
        .p("Now we can look at the functions we had to implement. Let's do the easy ones first.")
        .code(code["debugscriber2"].listing("Simplest subscriber fns"))
        .p("We enable all callsites (whether span or event) unconditionally. When a span is entered or exited, we print it.")
        .br()
        .p("Follows from is new. The ")
        .url("https://docs.rs/tracing/0.1.35/tracing/span/struct.Span.html#method.follows_from", "docs")
        .p(" show that you can indicate that a span has this type of relationship to another span. It might be appropriate to use instead of parent-child in some cases.")
        .p("This type of span can be open and running even though the thing that it followed closed. For example a thing which spawns tasks which may live longer than itself.")
        .code(code["debugscriber2"].listing("Event and new_span"))
}
