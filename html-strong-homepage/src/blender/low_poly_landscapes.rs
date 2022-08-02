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
            "That's the final cabin. A lamp was added as well as a fun crooked pipe. We will \
             texture and add lights and such in a later part.",
        )
}

pub fn bye_world() -> Article {
    Article::new()
        .h2("Finishing Everything")
        .p("We continue a bit in the colorless life, adding details:")
        .image("landscape-details.webp")
        .p(
            "So now the trees are cooler, we have a pond with some life in it, and the island \
             looks more convincing.",
        )
        .br()
        .p("But let's add color!")
        .image("color-early.webp")
        .p(
            "So much more fun in just a few minutes by adding materials to objects. Very \
             impactful!",
        )
        .p(
            "Then we add some emission materials (using slots) on some objects, which adds a lot. \
             Also the island base gets some love.",
        )
        .image("color-more.webp")
        .p("We really bring things to life by adding a background image.")
        .image("final.webp")
        .p(
            "The lighting is improved by using suns from different angles and colors, and using \
             an HDRI background.",
        )
        .p(
            "I also went offroad from the tutorial by adding a few details: A clothesline with \
             some jeans, a ladder, and a broom.",
        )
        .br()
        .p("Let's take it for a literal spin:")
        .video("island.mp4")
}
