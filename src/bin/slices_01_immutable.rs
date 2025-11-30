// src/bin/slices_01_immutable.rs
//
// Basic immutable slices: &a[start..end]

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("array a           = {:?}", a);

    // Half-open range: start..end (end is EXCLUSIVE)
    let slice = &a[1..3];
    println!("&a[1..3]          = {:?}  // indices 1 and 2", slice);

    // From start
    let first_three = &a[..3];
    println!("&a[..3]           = {:?}  // indices 0..2", first_three);

    // To end
    let last_three = &a[2..];
    println!("&a[2..]           = {:?}  // indices 2..len-1", last_three);

    // Inclusive range
    let mid = &a[1..=3];
    println!("&a[1..=3]         = {:?}  // indices 1..3", mid);

    // Equality is by value, not by “same parent array”
    let literal_slice: &[i32] = &[2, 3];
    println!("literal_slice     = {:?}", literal_slice);
    assert_eq!(slice, literal_slice);

    println!("\nImmutable slice demo ✅");
}
