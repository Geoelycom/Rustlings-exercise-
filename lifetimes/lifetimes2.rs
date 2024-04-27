// lifetimes2.rs
//
// So if the compiler is just validating the references passed
// to the annotated parameters and the return type, what do
// we need to change?
//
// Execute `rustlings hint lifetimes2` or use the `hint` watch subcommand for a hint.


fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");
    let string2 = String::from("xyz");
    let result;
    {
        // string2 is inside the curly braces which makes it only valid inside this curly braces and lives shorter than its lifetime we intend to compare it with. hence it won't live long enough for this comparison to happen.
        // Solution: we take the string2 outside of the curly braces so it lives long enough to be compared
        // we can also remove the curly braces from the main function. both of this solution works.
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is '{}'", result);
}
