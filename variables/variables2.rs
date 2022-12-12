// variables2.rs
// Execute `rustlings hint variables2` or use the `hint` watch subcommand for a hint.
// An integer is a number without a fractional component. we have the u32 type
// this indicate that the value its associated with is unsigned
//(signed integer types starts with i and usigned starts with u)

fn main() {
    let x:u32 = 10;
    if x == 10 {
        println!("x is ten!");
    } else {
        println!("x is not ten!");
    }
}
