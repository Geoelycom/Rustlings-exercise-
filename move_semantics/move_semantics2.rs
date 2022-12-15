// move_semantics2.rs
// Make me compile without changing line 13 or moving line 10!
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand for a hint.


fn main() {
    let vec0 = Vec::new(); // vec0 comes into scope and its valid
    let mut vec1 = fill_vec(vec0.clone()); //vec1 gets into scope and its valid
    // vec1 = 22,44,66,88 and vec0 = 22,44,66,88. due to the deep copy of clone()
    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88); //vec1 is still in scope here and valid.

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
} //vec0 goes out of scope and data is dropped making it invalid.

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;
// the only variable that owns any data in this case is vec = [22,44,66]
// how do i make vec1 = [88] after pushing into the stack inherit the 
// data in vec if they don't share thesame scope?(reference and ownership, move) 
//  -->>> the clone() method can be used to deeply clone the varaible and its content when passed into the fill_vec function as an argument.
    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
