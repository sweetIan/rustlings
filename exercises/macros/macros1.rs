// macros1.rs
// Make me compile! Execute `rustlings hint macros1` for hints :)

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

macro_rules! my2 {
    () => {
        println!("z")
    };
}

fn main() {
    my2!();
}
