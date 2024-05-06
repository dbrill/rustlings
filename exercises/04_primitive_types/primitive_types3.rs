// primitive_types3.rs
//
// Create an array with at least 100 elements in it where the ??? is.
//
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand
// for a hint.


fn main() {
	// originally I used a vanilla Array like ["test"; 200], which creates an array filled with 200 instances of the string "test"
	// Then I tried out this fancy Vec syntax. Vecs are dynamically sized and so they go on the heap whereas Arrays are fixed size
	// and go on the stack.
    let a = (0..200).collect::<Vec<u32>>(); 

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed")
    }
}
