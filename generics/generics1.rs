// This shopping list program isn't compiling!
// Use your knowledge of generics to fix it.

// Execute `rustlings hint generics1` or use the `hint` watch subcommand for a hint.
//my notes
// <T> or <Element> the T usually means some placeholder type
// T is located and used from the std::cmp::PartialOrd

// Monomorphization(Turning generic code into specifics code by filling the concrete type use at compile time)
// I AM NOT DONE

fn main() {
    let mut shopping_list: Vec<&str> = Vec::new();
    shopping_list.push("milk");
}
