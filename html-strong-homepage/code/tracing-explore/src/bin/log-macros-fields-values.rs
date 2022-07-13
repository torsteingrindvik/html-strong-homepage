use tracing::info;

// listing 1: info!() with extras
#[derive(Debug)]
struct MyStruct {
    _v: u8,
}

fn main() {
    let cat: i8;

    cat = -1;

    info!(foo = 123, thing = "Something", hi = ?MyStruct{_v: 10}, cat, "Hey");
}
// ~listing
