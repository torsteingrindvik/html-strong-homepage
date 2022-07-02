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

pub fn death_and_decay() -> Article {
    Article::new()
        .h2("Death and Decay")
        .p("So initially I separated four glasses with basil, two inside, and two outside in the conservatory.")
        .p("The hope was to learn something by seeing a difference between the batches. I got what I asked for. Inside top view:")
        .image("inside-top-view.webp")
        .p("I'm not an expert but that doesn't look healthy.")
        .p("Root view:")
        .image("inside-root-view.webp")
        .p("No action on the roots either. Rest in peace sweet princes.")
        .br()
        .p("Let's see what's going on outside.")
        .p("So the mother plant isn't doing so swell either:")
        .image("mother-plant.webp")
        .p("I'm not sure if it can be salvaged but I will keep watering it until it look as decay-like as the above cuttings.")
        .p("The two conservatory glasses contain smidgens of hope. See:")
        .image("conservatory-top-view.webp")
        .p("\"Is this hope?\", you say, and yes the leaves are concerning. But look at this:")
        .image("conservatory-root-view-01.webp")
        .p("and this:")
        .image("conservatory-root-view-02.webp")
        .p("This might turn into something. I also learned that I get a creeped out by roots.")
        .br()
        .p("As a note the ")
        .url("https://www.youtube.com/embed/byoEBdVoVpM", "video")
        .p(" has longer roots, and stayed in water for ~18 days. These have been in water for ~8 days. The days have been hot though, between 25 and 30 Celsius at times.")
        .br()
        .p("Next up is putting these in soil (compost?).")
}
