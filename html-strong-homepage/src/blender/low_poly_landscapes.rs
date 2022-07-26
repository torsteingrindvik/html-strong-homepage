use crate::components::Article;

pub fn hello_world() -> Article {
    Article::new()
        .h2("Building a Cabin")
        .p(
            "The first bulk of this tutorial revolves around basic shapes, mirroring, edge loops, \
             insets, the solidify modifier.",
        )
        .br()
        .p("Some placeholder trees are added. The landscape is the focus of the next part.")
        .image("cabin-early.webp")
        .p("Here we have already done a lot.")
        .br()
        .p("Here is another angle. We added a door.")
        .image("cabin-door.webp")
        .p("Then some more details are added.")
        .image("cabin-done.webp")
        .p(
            "That's the final cabin. A lamp was added as well as a fun crooked pipe. We will texture and add lights and such in \
             a later part.",
        )
}
