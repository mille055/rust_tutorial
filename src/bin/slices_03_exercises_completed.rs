// src/bin/slices_03_exercise.rs
//
// Array + slice exercises.
//
// For each case below:
//
//   - Do NOT change the initial array.
//   - Use slices (and any helper functions you choose to write) to
//     transform the array as described in the comments.
//   - When you think you’re right, uncomment the assert for that case
//     to automatically check your work.
//
// You decide:
//   - whether to write helper functions
//   - what their signatures are
//   - how to slice the arrays
//
// The only hard rule: don’t change the initial values or the expected
// arrays in the asserts.

fn main() {
    // -------------------- Case 1 --------------------
    let mut a = [1, 2, 3, 4];
    println!("Case 1 start: a = {:?}", a);

    // Goal for Case 1 (starting from [1, 2, 3, 4]):
    //
    //   - Add 2 to the *first* element.
    //   - Multiply the *last three* elements by 5.
    //
    // After your code runs, `a` should be:
    //   [3, 10, 15, 20]
    //
    // Write your code for Case 1 here:
    fn add_two(xs: &mut [i32]) {
        for x in xs.iter_mut() {*x += 2}
    }

    fn multiply_five(xs: &mut [i32]) {
        for x in xs.iter_mut() {*x *= 5}
    }
    
    fn first(xs: &mut [i32]) {
        add_two(&mut xs[..1]);
        multiply_five(&mut xs[1..]);
    }

    first(&mut a);

    // When you're ready to check Case 1, uncomment this:
    //
    assert_eq!(
        a,
        [3, 10, 15, 20],
        "Case 1 failed: expected [3, 10, 15, 20], got {:?}",
        a
    );
    //
    println!("Case 1 end:   a = {:?}", a);
    println!("Case 1 expected:[3, 10, 15, 20]");
    println!();

    // -------------------- Case 2 --------------------
    let mut b = [10, 20, 30, 40, 50];
    println!("Case 2 start: b = {:?}", b);

    // Goal for Case 2 (starting from [10, 20, 30, 40, 50]):
    //
    //   - Subtract 5 from the 2nd, 3rd, and 4th elements
    //     (i.e., the values 20, 30, and 40).
    //   - Divide the *last* element by 10.
    //
    // After your code runs, `b` should be:
    //   [10, 15, 25, 35, 5]
    //
    // Write your code for Case 2 here:

    // TODO: your slice-based operations for `b` go here.
    fn case_2(xs: &mut [i32]) {
        for x in xs[1..4].iter_mut() {*x -= 5 }
        xs[xs.len() - 1] /= 10;
    }
    case_2(&mut b);

    // When you're ready to check Case 2, uncomment this:
    //
    assert_eq!(
        b,
        [10, 15, 25, 35, 5],
        "Case 2 failed: expected [10, 15, 25, 35, 5], got {:?}",
        b
    );
    //
    println!("Case 2 end:   b = {:?}", b);
    println!("Case 2 expected [10, 15, 25, 35, 5]");
    println!();

    // -------------------- Case 3 --------------------
    let mut c = [2, 4, 6, 8, 10, 12];
    println!("Case 3 start: c = {:?}", c);

    // Goal for Case 3 (starting from [2, 4, 6, 8, 10, 12]):
    //
    //   - Set the *first two* elements to 0.
    //   - Triple the *middle two* elements (currently 6 and 8).
    //   - Halve the *last two* elements (currently 10 and 12).
    //
    // After your code runs, `c` should be:
    //   [0, 0, 18, 24, 5, 6]
    //
    // Write your code for Case 3 here:

    // TODO: your slice-based operations for `c` go here.
    fn case_3(xs: &mut[i32]) {
        for x in xs[..2].iter_mut() {*x = 0};    
        for x in xs[2..4].iter_mut() {*x *= 3};
        for x in xs[4..6].iter_mut() {*x /= 2};

    }

    case_3(&mut c);

    // When you're ready to check Case 3, uncomment this:
    //
    assert_eq!(
        c,
        [0, 0, 18, 24, 5, 6],
        "Case 3 failed: expected [0, 0, 18, 24, 5, 6], got {:?}",
        c
    );
    //
    println!("Case 3 end:   c = {:?}", c);
    println!("Case 3 expects [0, 0, 18, 24, 5, 6]");
    println!();

    println!("Done. Uncomment asserts one by one to check your solutions.");
}
