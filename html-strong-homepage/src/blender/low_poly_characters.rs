use crate::components::Article;

pub fn hello_world() -> Article {
    Article::new()
        .h2("Let's get started")
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
        .h2("Simple character")
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

pub fn block_characters() -> Article {
    Article::new()
        .h2("Block characters")
        .p("We start fresh, and we will be using two reference images provided in the tutorial.")
        .p("The first is female-like:")
        .image("block-female.webp")
        .p("Then male-like:")
        .image("block-male.webp")
        .p(
            "Now we learn how to use some loop cuts to make more vertices, and we edit points in \
             edit mode and so on. After a while we have something like",
        )
        .image("trace-leg.webp")
        .p(
            "But when we do work we normally use one of the orthogonal views with transparent \
             geometry (rendered solid in the following):",
        )
        .image("trace-leg-ortho.webp")
        .p("After some work, we arrive here:")
        .image("male-final-maybe.webp")
        .p("Pretty cool I think!")
        .p("We learned a really nice trick.")
        .list(vec![
            "Select lots of objects, they have a red outline",
            "The last object will have an orange outline; it is the active selection",
            "If you ctrl+L, you can \"link\" stuff",
            "This includes linking modifiers",
            "So you can e.g. make many arm-parts mirrored in the same way",
            "Profit!",
        ])
        .p("So on we go. Next up is creating a similar block character in a more female form.")
        .p(
            "We learned a trick! Placing the 3D cursor somewhere and using that as pivot point is \
             nice.",
        )
        .p(
            "Using that you can e.g. select many objects (like parts of an arm) and rotate around \
             that pivot. Nice. Anyway, we got this:",
        )
        .image("woman-final-maybe.webp")
        .p("Not too shabby.")
        .p("Then we got challenged to mess around with creating new characters.")
        .p("So let's try that, and let's not use reference images and see how we fare.")
        .p("So we ended up making a big guy. Now we have some kind of family:")
        .image("family.webp")
}

pub fn cyborg() -> Article {
    Article::new()
        .h2("Cyborg")
        .p("We start out by copying one of the previous models we have created.")
        .p("Then we learn some nice tricks")
        .list(vec![
            "In edit mode, we can alt+leftclick an edge in edge mode to select a loop of edges",
            "If we do that in face mode we select an entire loop of faces",
            "Clicking i for inset allows us to... create insets, which can model neat stuff",
            "Loop cuts are great",
            "Extruding is cool, but also scary since it can mess up geometry if... I'm unlucky?",
            "We can ctrl+E to have more extrude options, like extruding along face normals",
        ])
        .p(
            "So the goal was to create a cyborg by using the above tricks. This was fun, almost \
             like drawing badass characters as a kid on paper. Look:",
        )
        .image("cyborg.webp")
        .p("Fun!")
        .p(
            "I also know there is something called a three point light, but no idea how that's \
             supposed to be set up.",
        )
        .p(
            "I added three lights in a triangular fashion around the character and added some \
             colors to them.",
        )
}

pub fn cyborg_follow_up() -> Article {
    Article::new()
        .h2("Cyborg follow-up")
        .p("Cyborg part of tutorial is done after this.")
        .p("I learned about slots, which can be used to assign materials to a selection of faces on an object.")
        .p("So I then used that to give certain faces emissive materials, which was also a part of the tutorial. Very fun:")
        .image("cyborg-lights.webp")
        .p("The tutorial does not include anything about animations, so I fumbled my way to do something:")
        .video("cyborg-lights-animation.webm")
        .p("The rotation messed up badly, but that's just funny. And we get to test video on the blog.")
}
