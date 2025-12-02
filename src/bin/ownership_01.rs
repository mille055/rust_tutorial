// src/bin/ownership_exercises.rs
//
// Ownership, borrowing, and move semantics exercises.
//
// For each case below:
//
//   - Read the goal carefully.
//   - Fix the code so it compiles AND produces the expected result.
//   - You may need to use references, clones, or restructure the code.
//   - When you think you're right, uncomment the assert for that case.
//
// The only hard rule: don't change the expected values in the asserts.

fn main() {
    // -------------------- Case 1 --------------------
    // Understanding move semantics with String
    
    let s1 = String::from("hello");
    println!("Case 1 start: s1 = {:?}", s1);

    // Goal for Case 1:
    //
    //   - Create s2 that contains the same value as s1
    //   - Both s1 and s2 should be usable after the operation
    //   - Hint: String doesn't implement Copy...
    //
    // After your code runs, both should equal "hello"
    //
    // Write your code for Case 1 here:

    // TODO: Create s2 from s1 in a way that keeps both usable
    // let s2 = ???;

    // When you're ready to check Case 1, uncomment this:
    //
    // assert_eq!(s1, "hello", "Case 1 failed: s1 should still be 'hello'");
    // assert_eq!(s2, "hello", "Case 1 failed: s2 should be 'hello'");
    //
    // println!("Case 1 end: s1 = {:?}, s2 = {:?}", s1, s2);
    println!();

    // -------------------- Case 2 --------------------
    // Borrowing to avoid moves
    
    let mut data = vec![1, 2, 3, 4, 5];
    println!("Case 2 start: data = {:?}", data);

    // Goal for Case 2:
    //
    //   - Write a function `sum_vec` that calculates the sum of a Vec<i32>
    //   - The function should NOT take ownership (data should be usable after)
    //   - Then push 6 onto data
    //   - Call sum_vec again on the modified data
    //
    // Expected: first sum = 15, second sum = 21, final data = [1,2,3,4,5,6]
    //
    // Write your code for Case 2 here:

    // TODO: Define sum_vec with the right signature
    // fn sum_vec(???) -> i32 {
    //     ???
    // }

    // TODO: Call sum_vec, push 6, call sum_vec again
    // let first_sum = ???;
    // ???
    // let second_sum = ???;

    // When you're ready to check Case 2, uncomment this:
    //
    // assert_eq!(first_sum, 15, "Case 2 failed: first sum should be 15");
    // assert_eq!(second_sum, 21, "Case 2 failed: second sum should be 21");
    // assert_eq!(data, vec![1, 2, 3, 4, 5, 6], "Case 2 failed: data should have 6 pushed");
    //
    // println!("Case 2 end: data = {:?}", data);
    println!();

    // -------------------- Case 3 --------------------
    // Mutable references - only one at a time!
    
    let mut values = [10, 20, 30];
    println!("Case 3 start: values = {:?}", values);

    // Goal for Case 3:
    //
    //   - Write a function `double_element` that doubles a single i32 in place
    //   - Use it to double the first element, then the last element
    //   - Be mindful of mutable reference rules
    //
    // After your code runs, `values` should be [20, 20, 60]
    //
    // Write your code for Case 3 here:

    // TODO: Define double_element
    // fn double_element(???) {
    //     ???
    // }

    // TODO: Double first element, then last element
    // ???

    // When you're ready to check Case 3, uncomment this:
    //
    // assert_eq!(
    //     values,
    //     [20, 20, 60],
    //     "Case 3 failed: expected [20, 20, 60], got {:?}",
    //     values
    // );
    //
    // println!("Case 3 end: values = {:?}", values);
    println!();

    // -------------------- Case 4 --------------------
    // Returning references vs values
    
    let words = vec![
        String::from("apple"),
        String::from("banana"),
        String::from("cherry"),
    ];
    println!("Case 4 start: words = {:?}", words);

    // Goal for Case 4:
    //
    //   - Write a function `longest_word` that returns a reference to
    //     the longest string in a slice of Strings
    //   - The function should borrow, not take ownership
    //   - words should still be usable after calling the function
    //
    // Expected: longest = "banana", words unchanged
    //
    // Write your code for Case 4 here:

    // TODO: Define longest_word - think about the return type!
    // fn longest_word(???) -> ??? {
    //     ???
    // }

    // TODO: Call longest_word
    // let longest = ???;

    // When you're ready to check Case 4, uncomment this:
    //
    // assert_eq!(longest, "banana", "Case 4 failed: longest should be 'banana'");
    // assert_eq!(words.len(), 3, "Case 4 failed: words should still have 3 elements");
    //
    // println!("Case 4 end: longest = {:?}, words = {:?}", longest, words);
    println!();

    // -------------------- Case 5 --------------------
    // The classic "cannot borrow as mutable because also borrowed as immutable"
    
    let mut numbers = vec![1, 2, 3, 4, 5];
    println!("Case 5 start: numbers = {:?}", numbers);

    // Goal for Case 5:
    //
    //   - Find the first even number in the vector
    //   - If found, push its double to the end of the vector
    //   - Hint: you can't hold an immutable reference while mutating!
    //   - Think about when the borrow ends...
    //
    // After your code runs, `numbers` should be [1, 2, 3, 4, 5, 4]
    // (first even is 2, so we push 2*2=4)
    //
    // Write your code for Case 5 here:

    // TODO: Find first even, then push its double
    // This naive approach won't compile - fix it!
    // let first_even = numbers.iter().find(|&&x| x % 2 == 0);
    // if let Some(&val) = first_even {
    //     numbers.push(val * 2);
    // }

    // When you're ready to check Case 5, uncomment this:
    //
    // assert_eq!(
    //     numbers,
    //     vec![1, 2, 3, 4, 5, 4],
    //     "Case 5 failed: expected [1, 2, 3, 4, 5, 4], got {:?}",
    //     numbers
    // );
    //
    // println!("Case 5 end: numbers = {:?}", numbers);
    println!();

    println!("Done. Uncomment asserts one by one to check your solutions.");
}