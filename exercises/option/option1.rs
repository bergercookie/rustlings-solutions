// option1.rs
// Make me compile! Execute `rustlings hint option1` for hints

// you can modify anything EXCEPT for this function's sig
fn print_number(maybe_number: Option<u16>) {
    println!("printing: {}", maybe_number.unwrap());
}

fn main() {
    print_number(Some(13));
    print_number(Some(99));

    // you have to initialize it otherwise rust doesn't detect that it's being initialized inside
    // the loop for some reason...
    // = [None; 5];
    let mut numbers: [Option<u16>; 5] = [None; 5];
    for iter in 0..5 {
        let number_to_add: u16 = { ((iter * 5) + 2) / (4 * 16) };

        // indices should be caste to usize explicitly
        numbers[iter as usize] = Some(number_to_add);
    }
}
