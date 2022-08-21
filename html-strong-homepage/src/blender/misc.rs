use crate::components::Article;

pub fn mushrooms() -> Article {
    Article::new()
        .h2("Mushrooms")
        .p("I saw a nice YouTube video about texture painting: ")
        .url("https://www.youtube.com/watch?v=4d4N8d4ki2Y", "Ryan King Texture Painting Tutorial")
        .br()
        .p("I followed it. First making a basic mesh:")
        .image("start.webp")
        .p("Then comes UV unwrapping:")
        .image("uv-unwrap.webp")
        .p("Which is an interesting process.")
        .p("The goal as I understand it is to maximize the area of the most detailed area of the texture.")
        .p("So where the mesh needs a lot of detail, it should map to a bigger area in the texture (which is determined by the UV map), since there is simply room for more colors there.")
        .p("Also, the mapping above isn't the best since it would waste about half of the texture data.")
        .image("base-texture.webp")
        .p("No the mapping is better since it uses more of the texture.")
        .p("Also a base texture is now in place, unpainted but with a base color applied.")
        .image("painting.webp")
        .p("Some time later, this is what we got! It looks like a mushroom. Nice.")
        .image("with-hdri.webp")
        .p("Rendered with an HDRI forest background looks more impressive.")
        .image("with-normals.webp")
        .p("Adding a normal map felt like it didn't do much, but comparing the two above I see that it's actually a lot better.")
        .p("Without a drawing tablet it's pretty awkward to add normals, as it's too rigid without pressure drawing.")
        .p("Seems less bad from a distance though.")
        .br()
        .p("I found a reference image of several mushrooms, and just went to town.")
        .p("After a while I made all these:")
        .image("shrooms.webp")
}
