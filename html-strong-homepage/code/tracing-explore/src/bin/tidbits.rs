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

fn foo() {
    // listing 4: str to &str
    impl<'a, T> Value for &'a T where T: 'a + Value + ?Sized {}
    // ~listing

    // listing 5: Other things which are Values
    // This one shows up a few times for Send, Sync
    impl Value for dyn Error {}

    // This is from core::fmt,
    // and is the output of the macro format_args!("{} let's go", 123).
    impl<'a> Value for Arguments<'a> {}

    // There is also something like this ish (plus lifetimes),
    // but it seems this is an unstable future thing for tracing,
    // so let's rather re-visit that in some far away blog post.
    impl Value for dyn Valuable {}

    // Now we support lots of things!
    impl<T> Value for DebugValue<T> where T: Debug {}
    impl<T> Value for DisplayValue<T> where T: Display {}

    // ~listing
}
