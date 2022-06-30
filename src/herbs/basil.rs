use crate::components::Article;

pub fn hello_world() -> Article {
    Article::new()
        .h2("Hello World")
        .p("So we bought a mother plant from the store.")
        .p("Then we learned a lot from ")
        .url("https://www.youtube.com/embed/byoEBdVoVpM", "this")
        .p(" video.")
        .p("Notes:")
        .list(vec![
            "Cut just below a 'node', ie where smaller branches.. branch out",
            "Then snip off lower leaves from cuttings so they don't get soaked in water (and die)",
            "Then put in cup of water!",
            "Rule of thumb: Two nodes under water (no leaves), >=4 leaves on top node",
            "~18 days, he used grow light and heatmat. Looking not that much bigger but nice deep \
             green",
            "Then into small pots with compost",
            "One pot for one cutting, even though the cups shared cuttings",
            "Then ~22 days or so with grow lamp in compost (pete/peet? based, it holds water well)",
            "Last 3 days liquid fertilizer, nitrogen something?",
            "So ish 40 days.",
            "Results were mixed, all got to be big plants",
            "Some less deep green",
            "Not as good soil",
            "Too big plant for pot",
            "Now we prune tops to encourage width instead of height",
            "We cut right above a nice split, to encourage more of that.",
            "So the top will be a type of 'knot'",
        ])
        .h2("Starting setup")
        .p("We took four wine glasses with water and put some cuttings into them.")
        .p("The mother plant is then left like so:")
        .image("mother-plant.webp")
        .p("Then we put two glasses inside, close to the door to the conservatory. Top view:")
        .image("inside-top-view.webp")
        .p("Root view:")
        .image("inside-root-view.webp")
        .p("The other two glasses outside in the conservatory. Top view:")
        .image("conservatory-top-view.webp")
        .p("Root view:")
        .image("conservatory-root-view.webp")
        .p(
            "Now they will stay like this for 2-3 weeks, so let's wait a few days and then have a \
             look at some progress pictures.",
        )
        .h2("First try: Pesto")
        .p(
            "We had some actual basil to use for food, but not too much. Anyway we got to try \
             making some pesto! ",
        )
        .url("https://www.youtube.com/embed/oyh3jYKZ0E8", "This")
        .p(" video was helpful.")
        .p("I learned that you put nuts and cheese into pesto.")
        .p("I had basil, olive oil (authentic from a Greek friend!), and some almonds.")
        .image("pesto.webp")
        .p("It tastes good! I'll try with other nuts as well, and with cheese. Can't wait!")
}
