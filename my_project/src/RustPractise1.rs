fn main() {
    let x: i32 = 5; // Initialize x with a value
    let y: i32; // Uninitialized but unused, still a warning, but doesn't cause an error

    assert_eq!(x, 5); // Now x is initialized, so this works
    println!("Success!");
}