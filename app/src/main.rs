use items::{Color, Shirt};

mod items;

fn main() {
    let mut shirt = Shirt::default();

    // This produces a deprecation warning:
    //shirt.deprecated_color = Color::Blue as _;

    // But not this:
    shirt.set_deprecated_color(Color::Blue);

    drop(shirt)
}
