// src/bin/slices_02_mutable.rs
//
// Mutable slices: &mut [T] as a “window” you can write through.
//
// This version is a FILL-IN-THE-BLANK exercise.
// It compiles and runs even before you fill anything in.
// Your job: make double_slice actually double the values in the slice.

fn double_slice(xs: &mut [i32]) {
    println!("  in double_slice, before: {:?}", xs);

    for x in xs.iter_mut() {
        // Replace this comment with the real line when you're ready:
        *x *= 2
    }

    println!("  in double_slice, after:  {:?}", xs);
}

fn main() {
    let mut a = [1, 2, 3, 4, 5];

    println!("original a         = {:?}", a);

    // TODO 2: Experiment with taking different mutable slices.
    //
    // Right now, this takes a mutable slice of the middle three elements: [2, 3, 4]
    // Try changing the range to see what happens, e.g.:
    //   &mut a[..]       // whole array
    //   &mut a[0..2]     // first two
    //   &mut a[2..5]     // last three
    //
    // The code below is already valid; you only change it if you want to experiment.
    double_slice(&mut a[1..3]);

    println!("after double_slice = {:?}  // will change once TODO 1 is implemented", a);

    // TODO 3 (optional): Play with another mutable slice inside a block.
    {
        // Example idea (commented out so it doesn't require you to use it yet):
        //
        // let tail: &mut [i32] = &mut a[3..5];
        // tail[0] += 10;
        // tail[1] += 100;
        //
        // For now, this block does nothing. Uncomment and tweak as you experiment.
    }
    


    println!("\nMutable slice demo (fill-in-the-blank version) ✅");
}
