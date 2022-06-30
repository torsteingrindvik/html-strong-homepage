fn fancy_block() {
    // listing 1: Unfancy
    // Create a closure
    let my_closure = |num| println!("The number is {num}");

    // Call it with some stuff
    my_closure({
        let foo = 1;
        foo + 1
    });
    // ~listing

    // listing 2: Fancy
    // Create a closure, use it right away
    (|num| println!("The number is fancily {num}"))({
        let bar = 1;
        bar + 1
    })
    // ~listing
}

fn main() {
    fancy_block();
}

// listing 3: ValueSet def
pub struct ValueSet<'a> {
    values: &'a [(&'a Field, Option<&'a (dyn Value + 'a)>)],
    fields: &'a FieldSet,
}
// ~listing
