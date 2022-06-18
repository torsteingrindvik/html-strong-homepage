use crate::components::Article;

pub fn hello_world() -> Article {
    Article::new()
        .header("Let's get started")
        .p(
            "Got distracted by looking at audio pre-amps and then Apex Legends, but at least now \
             we've started!",
        )
        .list(vec![
            "Did 'The Blender Interface', which was a nice refresh of old news, like framing \
             selected objects and the numpad for selecting different views.",
            "Did 'Adding Objects', just a refresher on how to do add meshes. Nice to remember to \
             shift+rightclick to set the cursor/\"spawn point\".",
        ])
        .p("Anyway we got to this:")
        .image("adding-objects.webp")
        .p("But that will be discarded anyway as it was just practice.")
}

pub fn simple_character() -> Article {
    Article::new()
        .header("Simple character")
        .p("Now we start over.")
        .p("So we start making simple characters now. We end up doing this:")
        .image("simple-character.webp")
        .p(
            "Then we learn a bit about materials. We mess around a bit with color, metallic, and \
             roughness.",
        )
        .p("So doing that we end up with something like this:")
        .image("color-characters.webp")
        .p("We picked up some neat tricks:")
        .list(vec![
            "Hold down shift to get smaller increments of stuff",
            "shift+ctrl+numpad0 sets camera to current view",
            "shift+z/x/y allows you to scale in axes except the one pressed",
            "shift+zz/xx/yy makes you work in the double pressed axis local to the selected object",
            "the transform pivot point allows you to do fancy stuff, will need to internalize by \
             use though",
        ])
        .p("We were given a challenge to play around a bit more with characters. The result:")
        .image("cool-character.webp")
        .p(
            "We made the legs by copy-paste, but other symmetry was done by the mirror modifier \
             which is really nice.",
        )
        .p("This was pretty fun! The character turned out pretty Playstation-1-esque?")
        .p("If we knew how to do rigging and animations, this would be even more fun!")
}
