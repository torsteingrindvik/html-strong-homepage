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

pub fn big_changes() -> Article {
    Article::new()
        .h2("Big Changes")
        .p("I did not have confidence in just leaving pots around in the declining temperatures in the coming months.")
        .p("So I made a plan. I bought stuff. We needed pots:")
        .image("pots.webp")
        .p("We also need some electronics. A thermometer, and power which can be set on a timer:")
        .image("thermo-timer-power.webp")
        .p("And a bit more fancy, I got a light strip specifically made for plant growth, and a heat pad:")
        .image("light-heat.webp")
        .p("We got DIRT:")
        .image("dirt.webp")
        .p("And leca pellets:")
        .image("leca.webp")
        .p("Also BOX:")
        .image("box.webp")
        .p("What's in the box then?")
        .image("box-messy.webp")
        .p("A bit neater:")
        .image("box-tidy.webp")
        .p("This cover as well:")
        .image("box-tent.webp")
        .p("Ah and the light looks like this when turned on:")
        .image("light-on.webp")
        .p("Ok then. The box contents is a miniature greenhouse. Let's assemble.")
        .image("frame.webp")
        .p("That was plug and play, no nuts and bolts. Add shelves:")
        .image("frame-shelves.webp")
        .p("And the cover:")
        .image("frame-tent.webp")
        .p("Nice! Let's put the thermometer in at the top:")
        .image("thermo-in-gh.webp")
        .p("Now we need to add the light strip. How? A couple of clips:")
        .image("light-clip.webp")
        .p("Noice. It looks like this when fastened to the frame with some bands:")
        .image("light-in-gh.webp")
        .p("So far so good!")
        .br()
        .p("But how are the plants doing?")
        .p("The mother plant is a bit under the weather..")
        .image("mother-plant.webp")
        .p("Let's just place that on the lower shelf.")
        .image("mother-plant-in-gh.webp")
        .p("The cuttings are doing a bit better:")
        .image("cuttings-1.webp")
        .image("cuttings-2.webp")
        .p("Let's prep a couple of pots:")
        .image("pots-dirt-leca.webp")
        .p("My understanding is that the leca will allow water to drain faster and generally make a more aerated soil for the basil.")
        .p("So let's add cuttings:")
        .image("plants-pot-1.webp")
        .image("plants-pot-2.webp")
        .p("And add some more soil and a bit of leca pellets:")
        .image("plants-filled.webp")
        .p("Meanwhile, the heatpad is getting warmed up:")
        .image("pot-on-heatpad.webp")
        .p("It uncurls itself as it warms up.")
        .p("Let's put the pots with cuttings in them in:")
        .image("plants-in-gh.webp")
        .p("Nice! The setup looks like this now:")
        .image("plants-in-gh-overview.webp")
        .p("We can hide some stuff at the bottom, neat and tidy:")
        .image("plants-in-gh-final.webp")
        .p("That's it! Initially I turn the light strip and heatpad on for 16 hours at a time, because I read somewhere that the basil should have a natural rest.")
        .p("No clue if that's true though.")
        .p("Anyway we have the possibility for more consistent light now, a bit higher temperature, and likely higher humidity (good? not sure).")
        .br()
        .p("It will be fun to monitor this setup in the coming days and weeks!")
}

pub fn seeds() -> Article {
    Article::new()
        .h2("Seeds")
        .p("I was curious to try to make basil from seeds as well.")
        .p("And why wait to see if the cuttings pan out? If they fail and I have to start over, there will be a lot of days with not much happening. So I bought stuff.")
        .h3("Buy things")
        .image("new-buys.webp")
        .p("Some nutrition, two packets of basil seeds (Genevese sort is what they had), and another strip of lights which can be put onto a new \"floor\" in the miniature greenhouse.")
        .p("However, they were out of heating pads, so I will put the whole bunch in the top floor- the penthouse.")
        .h3("Potting the newbies")
        .p("So first lets put dirt and leca pellets for the seeds to live in:")
        .image("pot-seeds.webp")
        .p("We water it:")
        .image("seeds-planted.webp")
        .p("But we're trying a new scheme. We will be putting a dash of fertilizer when watering the rightmost pot of each \"category\".")
        .p("So the rightmost cutting-pot gets a bit of fertilizer, and the same for the rightmost seeds-pot. Let's see if that makes a difference.")
        .h3("The old-timers")
        .p("So old-timer left pot has had a little whoops:")
        .image("left-pot.webp")
        .p("Is that mold? Removing that.")
        .p("Right pot is doing ok:")
        .image("right-pot.webp")
        .p("Oh an the mother plant from the store was doing.. not great. So we pillaged mother's remains, and we will see if that produces offspring.")
        .p("I'm not sure if anything from it was usable, but let's just try. In water:")
        .image("mothers-remains.webp")
        .p("They are really very short and might all die off, but then we know.")
        .h3("The whole family")
        .p("So since we could not get another heating pad, let's cramp the whole lot together:")
        .image("whole-family.webp")
}