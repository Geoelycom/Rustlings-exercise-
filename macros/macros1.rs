// macros1.rs
// Execute `rustlings hint macros1` or use the `hint` watch subcommand for a hint.
// Macros are special type of rust sytanx
// I AM NOT DONE

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro();
}
